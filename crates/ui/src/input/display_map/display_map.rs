/// DisplayMap: Düzenleyici/girdi gösterim eşlemesi için genel cephe.
///
/// WrapMap ve FoldMap katmanlarını birleştirerek tek bir API sağlar:
/// - BufferPoint <-> DisplayPoint dönüşümü
/// - Katlama yönetimi (adaylar, açıp kapatma, sorgu)
/// - Metin veya yerleşim değiştiğinde otomatik projeksiyon güncellemeleri
use std::ops::Range;

use crate::ham_gpui::{App, Font, Pixels};
use ropey::Rope;

use super::fold_map::FoldMap;
use super::folding::FoldRange;
use super::text_wrapper::{LineItem, WrapDisplayPoint};
use super::wrap_map::WrapMap;
use super::{BufferPoint, DisplayPoint};
use crate::input::Point as TreeSitterPoint;
use crate::input::display_map::WrapPoint;
use crate::input::rope_ext::RopeExt as _;

/// DisplayMap, düzenleyici/girdi koordinat eşlemesinin ana arayüzüdür.
///
/// İki katmanlı projeksiyonu yönetir:
/// 1. Tampon → Wrap (yumuşak-sarma)
/// 2. Wrap → Gösterim (katlama)
///
/// Düzenleyici/girdi tarafının yalnızca BufferPoint ve DisplayPoint ile çalışması yeterlidir.
pub struct DisplayMap {
    wrap_map: WrapMap,
    fold_map: FoldMap,
}

impl DisplayMap {
    pub fn new(font: Font, font_size: Pixels, wrap_width: Option<Pixels>) -> Self {
        Self {
            wrap_map: WrapMap::new(font, font_size, wrap_width),
            fold_map: FoldMap::new(),
        }
    }

    // ==================== Core Coordinate Mapping ====================

    /// Tampon konumunu gösterim konumuna dönüştürür.
    pub fn buffer_pos_to_display_pos(&self, pos: BufferPoint) -> DisplayPoint {
        // Buffer → Wrap
        let wrap_pos = self.wrap_map.buffer_pos_to_wrap_pos(pos);

        // Wrap → Display
        if let Some(display_row) = self.fold_map.wrap_row_to_display_row(wrap_pos.row) {
            DisplayPoint::new(display_row, wrap_pos.col)
        } else {
            // Cursor is in a folded region, find nearest visible row
            let display_row = self.fold_map.nearest_visible_display_row(wrap_pos.row);
            DisplayPoint::new(display_row, 0) // Column 0 at fold boundary
        }
    }

    /// Gösterim konumunu tampon konumuna dönüştürür.
    pub fn display_pos_to_buffer_pos(&self, pos: DisplayPoint) -> BufferPoint {
        // Display → Wrap
        let wrap_row = self.fold_map.display_row_to_wrap_row(pos.row).unwrap_or(0);

        // Wrap → Buffer
        let wrap_pos = WrapPoint::new(wrap_row, pos.col);
        self.wrap_map.wrap_pos_to_buffer_pos(wrap_pos)
    }

    /// Toplam görünür gösterim satırı sayısını döndürür.
    #[inline]
    pub fn display_row_count(&self) -> usize {
        self.fold_map.display_row_count()
    }

    /// Verilen gösterim satırının ait olduğu tampon satırını döndürür.
    pub fn display_row_to_buffer_line(&self, display_row: usize) -> usize {
        // Display → Wrap
        let wrap_row = self
            .fold_map
            .display_row_to_wrap_row(display_row)
            .unwrap_or(0);

        // Wrap → Buffer line
        self.wrap_map.wrap_row_to_buffer_line(wrap_row)
    }

    /// Bir tampon satırının gösterim satırı aralığını [başlangıç, bitiş) olarak döndürür.
    /// Tampon satırı tamamen gizliyse None döndürür.
    pub fn buffer_line_to_display_row_range(&self, line: usize) -> Option<Range<usize>> {
        // Buffer line → Wrap row range
        let wrap_row_range = self.wrap_map.buffer_line_to_wrap_row_range(line);

        // Find first and last visible display rows in this range
        let mut first_display_row = None;
        let mut last_display_row = None;

        for wrap_row in wrap_row_range {
            if let Some(display_row) = self.fold_map.wrap_row_to_display_row(wrap_row) {
                if first_display_row.is_none() {
                    first_display_row = Some(display_row);
                }
                last_display_row = Some(display_row);
            }
        }

        if let (Some(start), Some(end)) = (first_display_row, last_display_row) {
            Some(start..end + 1)
        } else {
            None // Completely folded
        }
    }

    /// Bir tampon satırının tamamen gizlenip gizlenmediğini kontrol eder.
    #[inline]
    pub fn is_buffer_line_hidden(&self, line: usize) -> bool {
        self.buffer_line_to_display_row_range(line).is_none()
    }

    /// tree-sitter/LSP kaynaklı katlama adaylarını ayarlar.
    pub fn set_fold_candidates(&mut self, candidates: Vec<FoldRange>) {
        self.fold_map.set_candidates(candidates);
        self.rebuild_fold_projection();
    }

    /// Verilen start_line değerindeki katlamayı ayarlar; satır adaylar içinde olmalıdır.
    pub fn set_folded(&mut self, start_line: usize, folded: bool) {
        self.fold_map.set_folded(start_line, folded);
        self.rebuild_fold_projection();
    }

    /// Verilen start_line değerindeki katlamayı açıp kapatır.
    pub fn toggle_fold(&mut self, start_line: usize) {
        self.fold_map.toggle_fold(start_line);
        self.rebuild_fold_projection();
    }

    /// Bir satırın şu anda katlanmış olup olmadığını kontrol eder.
    #[inline]
    pub fn is_folded_at(&self, start_line: usize) -> bool {
        self.fold_map.is_folded_at(start_line)
    }

    /// Bir satırın katlama adayı olup olmadığını kontrol eder.
    #[inline]
    pub fn is_fold_candidate(&self, start_line: usize) -> bool {
        self.fold_map.is_fold_candidate(start_line)
    }

    /// Şu anda katlanmış tüm aralıkları döndürür.
    #[inline]
    pub fn folded_ranges(&self) -> &[FoldRange] {
        self.fold_map.folded_ranges()
    }

    /// Tüm katlamaları temizler.
    pub fn clear_folds(&mut self) {
        self.fold_map.clear_folds();
        self.rebuild_fold_projection();
    }

    // ==================== Text and Layout Updates ====================

    /// Sarma eşlemesi güncellenmeden önce katlamaları ve adayları metin düzenlemesine göre ayarlar.
    ///
    /// Eski metinle (değiştirme öncesi), düzenleme aralığıyla ve new_text ile çağrılmalıdır;
    /// böylece etkilenen eski satırlar hesaplanabilir.
    pub fn adjust_folds_for_edit(&mut self, old_text: &Rope, range: &Range<usize>, new_text: &str) {
        if self.fold_map.folded_ranges().is_empty() && self.fold_map.fold_candidates().is_empty() {
            return;
        }

        let edit_start_line = old_text.offset_to_point(range.start).row;
        let edit_end_line = old_text.offset_to_point(range.end.min(old_text.len())).row;

        let old_lines_in_range = edit_end_line.saturating_sub(edit_start_line);
        let new_lines_in_range = new_text.chars().filter(|c| *c == '\n').count();
        let line_delta = new_lines_in_range as isize - old_lines_in_range as isize;

        self.fold_map
            .adjust_folds_for_edit(edit_start_line, edit_end_line, line_delta);
    }

    /// Metin düzenlemesinden sonra katlama adaylarını artımlı olarak günceller.
    ///
    /// Yeni katlama adaylarını yalnızca düzenlenen bayt aralığından çıkarır
    /// ve mevcut (zaten ayarlanmış) adaylarla birleştirir.
    pub fn update_fold_candidates_for_edit(
        &mut self,
        tree: &super::folding::Agac,
        edit_byte_range: Range<usize>,
        new_text: &Rope,
    ) {
        let new_start_line = new_text.offset_to_point(edit_byte_range.start).row;
        let new_end_line = new_text
            .offset_to_point(edit_byte_range.end.min(new_text.len()))
            .row;

        let new_candidates = super::folding::extract_fold_ranges_in_range(tree, edit_byte_range);
        self.fold_map
            .merge_candidates_for_edit(new_start_line, new_end_line, new_candidates);
    }

    /// Metni artımlı veya tam olarak günceller.
    pub fn on_text_changed(
        &mut self,
        changed_text: &Rope,
        range: &Range<usize>,
        new_text: &Rope,
        cx: &mut App,
    ) {
        self.wrap_map
            .on_text_changed(changed_text, range, new_text, cx);
        self.rebuild_fold_projection();
    }

    /// Yerleşim parametrelerini (sarma genişliği veya font) günceller.
    pub fn on_layout_changed(&mut self, wrap_width: Option<Pixels>, cx: &mut App) {
        self.wrap_map.on_layout_changed(wrap_width, cx);
        self.rebuild_fold_projection();
    }

    /// Font parametrelerini ayarlar.
    pub fn set_font(&mut self, font: Font, font_size: Pixels, cx: &mut App) {
        self.wrap_map.set_font(font, font_size, cx);
        self.rebuild_fold_projection();
    }

    /// Metnin hazır olduğundan emin olur; gerekiyorsa sarmalayıcıyı başlatır.
    pub fn ensure_text_prepared(&mut self, text: &Rope, cx: &mut App) {
        let did_initialize = self.wrap_map.ensure_text_prepared(text, cx);
        if did_initialize {
            self.rebuild_fold_projection();
        }
    }

    /// Metinle başlatır.
    pub fn set_text(&mut self, text: &Rope, cx: &mut App) {
        self.wrap_map.set_text(text, cx);
        self.rebuild_fold_projection();
    }

    // ==================== Internal Helpers ====================

    /// wrap_map veya katlama durumu değiştikten sonra katlama projeksiyonunu yeniden oluşturur.
    /// Yalnızca gerçekten katlanmış aralıklar varsa yeniden oluşturur.
    fn rebuild_fold_projection(&mut self) {
        if !self.fold_map.folded_ranges().is_empty() {
            self.fold_map.rebuild(&self.wrap_map);
        } else {
            // No active folds: identity mapping (wrap_row == display_row).
            // Just update cached count so query methods work without Vec allocation.
            self.fold_map
                .mark_dirty_with_wrap_count(self.wrap_map.wrap_row_count());
        }
    }

    // ==================== Wrap Display Point Operations ====================

    /// Bayt ofsetini, yumuşak sarma bilgisini içeren sarma gösterim noktasına dönüştürür.
    #[inline]
    pub(crate) fn offset_to_wrap_display_point(&self, offset: usize) -> WrapDisplayPoint {
        self.wrap_map.wrapper().offset_to_display_point(offset)
    }

    /// Sarma gösterim noktasını bayt ofsetine dönüştürür.
    #[inline]
    pub(crate) fn wrap_display_point_to_offset(&self, point: WrapDisplayPoint) -> usize {
        self.wrap_map.wrapper().display_point_to_offset(point)
    }

    /// Sarma gösterim noktasını TreeSitterPoint değerine (tampon satır/sütun) dönüştürür.
    #[inline]
    pub(crate) fn wrap_display_point_to_point(&self, point: WrapDisplayPoint) -> TreeSitterPoint {
        self.wrap_map.wrapper().display_point_to_point(point)
    }

    /// Bir sarma satırını gösterim satırına dönüştürür; katlanmış satırları atlar.
    /// Sarma satırı katlanmışsa None döndürür.
    #[inline]
    pub fn wrap_row_to_display_row(&self, wrap_row: usize) -> Option<usize> {
        self.fold_map.wrap_row_to_display_row(wrap_row)
    }

    /// nearest görünür gösterim satır için bir verilen sarma satır. bulur.
    #[inline]
    pub fn nearest_visible_display_row(&self, wrap_row: usize) -> usize {
        self.fold_map.nearest_visible_display_row(wrap_row)
    }

    /// bir gösterim satır için bir sarma satır. dönüştürür.
    #[inline]
    pub fn display_row_to_wrap_row(&self, display_row: usize) -> Option<usize> {
        self.fold_map.display_row_to_wrap_row(display_row)
    }

    /// en uzun satır indeks (ile bayt uzunluk) döndürür.
    #[inline]
    pub(crate) fn longest_row(&self) -> usize {
        self.wrap_map.wrapper().longest_row.row
    }

    // ==================== Access Methods ====================

    /// Çizim için satır öğelerine erişim döndürür.
    #[inline]
    pub(crate) fn lines(&self) -> &[LineItem] {
        self.wrap_map.lines()
    }

    /// Rope metnini döndürür.
    #[inline]
    pub fn text(&self) -> &Rope {
        self.wrap_map.text()
    }

    /// Bir tampon satırında kaç görünür (katlanmamış) sarma satırı olduğunu hesaplar.
    #[inline]
    pub fn visible_wrap_row_count_for_buffer_line(&self, line: usize) -> usize {
        self.wrap_map
            .visible_wrap_row_count_for_line(line, &self.fold_map)
    }

    /// sarma satır sayı (önce katlama) döndürür.
    #[inline]
    pub fn wrap_row_count(&self) -> usize {
        self.wrap_map.wrap_row_count()
    }

    /// tampon satır sayı (mantıksal satırlar) döndürür.
    #[inline]
    pub fn buffer_line_count(&self) -> usize {
        self.wrap_map.buffer_line_count()
    }
}
