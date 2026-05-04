/// WrapMap: Yumuşak sarma katmanı (tampon satırları -> sarma satırları).
///
/// Bu modül mevcut TextWrapper'ı sarar ve şunları sağlar:
/// - BufferPoint ↔ WrapPoint eşleme
/// - Önek toplam önbelleğiyle verimli buffer_line -> sarma satırı sorguları
/// - Metin veya yerleşim değiştiğinde artımlı güncellemeler
use std::ops::Range;

use crate::ham_gpui::{App, Font, Pixels};
use ropey::Rope;

use super::fold_map::FoldMap;
use super::text_wrapper::{LineItem, TextWrapper, WrapDisplayPoint};
use super::{BufferPoint, WrapPoint};
use crate::input::rope_ext::RopeExt;

/// WrapMap yumuşak sarmayı yönetir ve tampon ile sarma koordinatları arasında eşleme sağlar.
pub struct WrapMap {
    /// Alttaki metin sarmalayıcı (mevcut uygulamayı yeniden kullanır).
    wrapper: TextWrapper,

    /// Önek toplam önbelleği: buffer_line_starts[line] = `line` tampon satırının ilk sarma satırı.
    /// Bu, buffer_line -> sarma satırı aramasını O(1) yapar.
    buffer_line_starts: Vec<usize>,

    /// Son yeniden oluşturmadan kalan satır sayısı önbelleği.
    cached_line_count: usize,

    /// Son yeniden oluşturmadan kalan toplam sarma satırı sayısı önbelleği.
    /// `cached_line_count` ile birlikte önbelleğin bayatlayıp bayatlamadığını algılar.
    /// Yumuşak sarma bir satırın sarma sayısını değiştirip tampon satır sayısını değiştirmediğinde,
    /// bu alan bayat önbelleği yakalar.
    cached_wrap_row_count: usize,
}

impl WrapMap {
    pub fn new(font: Font, font_size: Pixels, wrap_width: Option<Pixels>) -> Self {
        Self {
            wrapper: TextWrapper::new(font, font_size, wrap_width),
            buffer_line_starts: Vec::new(),
            cached_line_count: 0,
            cached_wrap_row_count: 0,
        }
    }

    /// Toplam sarma satırı sayısını (yumuşak sarma sonrası görsel satırlar) döndürür.
    #[inline]
    pub fn wrap_row_count(&self) -> usize {
        self.wrapper.len()
    }

    /// Toplam tampon satırı sayısını (mantıksal satırlar) döndürür.
    #[inline]
    pub fn buffer_line_count(&self) -> usize {
        self.wrapper.lines.len()
    }

    /// Tampon konumunu sarma konumuna dönüştürür.
    pub(super) fn buffer_pos_to_wrap_pos(&self, pos: BufferPoint) -> WrapPoint {
        let BufferPoint { line, col } = pos;

        // Clamp to valid range
        let line = line.min(self.buffer_line_count().saturating_sub(1));
        let line_item = self.wrapper.lines.get(line);

        let col = if let Some(line_item) = line_item {
            col.min(line_item.len())
        } else {
            0
        };

        // Calculate offset in rope
        let line_start_offset = self.wrapper.text().line_start_offset(line);
        let offset = line_start_offset + col;

        // Use TextWrapper's existing conversion
        let display_point = self.wrapper.offset_to_display_point(offset);

        WrapPoint::new(display_point.row, display_point.column)
    }

    /// Sarma konumunu tampon konumuna dönüştürür.
    pub(super) fn wrap_pos_to_buffer_pos(&self, pos: WrapPoint) -> BufferPoint {
        let WrapPoint { row, col } = pos;

        // Clamp wrap_row to valid range
        let row = row.min(self.wrap_row_count().saturating_sub(1));

        // Use TextWrapper's existing conversion
        let display_point = WrapDisplayPoint::new(row, 0, col);
        let offset = self.wrapper.display_point_to_offset(display_point);

        // Convert offset to buffer position
        let point = self.wrapper.text().offset_to_point(offset);
        let line_start = self.wrapper.text().line_start_offset(point.row);
        let col = offset.saturating_sub(line_start);

        BufferPoint::new(point.row, col)
    }

    /// Verilen sarma satırının ait olduğu tampon satırını döndürür.
    pub fn wrap_row_to_buffer_line(&self, wrap_row: usize) -> usize {
        if wrap_row >= self.wrap_row_count() {
            return self.buffer_line_count().saturating_sub(1);
        }

        // Binary search in prefix sum cache
        match self.buffer_line_starts.binary_search(&wrap_row) {
            Ok(line) => line,
            Err(insert_pos) => insert_pos.saturating_sub(1),
        }
    }

    /// Verilen tampon satırının ilk sarma satırını döndürür.
    pub fn buffer_line_to_first_wrap_row(&self, line: usize) -> usize {
        if line >= self.buffer_line_starts.len() {
            return self.wrap_row_count();
        }
        self.buffer_line_starts[line]
    }

    /// Bir tampon satırının sarma satırı aralığını [başlangıç, bitiş) olarak döndürür.
    pub fn buffer_line_to_wrap_row_range(&self, line: usize) -> Range<usize> {
        let start = self.buffer_line_to_first_wrap_row(line);
        let end = if line + 1 < self.buffer_line_starts.len() {
            self.buffer_line_starts[line + 1]
        } else {
            self.wrap_row_count()
        };
        start..end
    }

    /// Metni artımlı veya tam olarak günceller.
    pub fn on_text_changed(
        &mut self,
        changed_text: &Rope,
        range: &Range<usize>,
        new_text: &Rope,
        cx: &mut App,
    ) {
        self.wrapper.update(changed_text, range, new_text, cx);
        self.rebuild_cache();
    }

    /// Yerleşim parametrelerini (sarma genişliği veya font) günceller.
    pub fn on_layout_changed(&mut self, wrap_width: Option<Pixels>, cx: &mut App) {
        self.wrapper.set_wrap_width(wrap_width, cx);
        self.rebuild_cache();
    }

    /// Font parametrelerini ayarlar.
    pub fn set_font(&mut self, font: Font, font_size: Pixels, cx: &mut App) {
        self.wrapper.set_font(font, font_size, cx);
        self.rebuild_cache();
    }

    /// Metnin hazır olduğundan emin olur; gerekiyorsa sarmalayıcıyı başlatır.
    pub fn ensure_text_prepared(&mut self, text: &Rope, cx: &mut App) -> bool {
        let did_initialize = self.wrapper.prepare_if_need(text, cx);
        if did_initialize {
            self.rebuild_cache();
        }
        did_initialize
    }

    /// Metinle başlatır.
    pub fn set_text(&mut self, text: &Rope, cx: &mut App) {
        self.wrapper.set_default_text(text);
        self.wrapper.prepare_if_need(text, cx);
        self.rebuild_cache();
    }

    /// Önek toplam önbelleğini yeniden oluşturur: buffer_line_starts.
    fn rebuild_cache(&mut self) {
        let line_count = self.wrapper.lines.len();
        let wrap_row_count = self.wrapper.len();

        // Skip if nothing changed: both buffer line count and total wrap row count must match.
        // Checking wrap_row_count is essential because soft-wrap can change the number of
        // wrap rows per line without changing the buffer line count.
        if line_count == self.cached_line_count
            && wrap_row_count == self.cached_wrap_row_count
            && !self.buffer_line_starts.is_empty()
        {
            return;
        }

        self.buffer_line_starts.clear();

        let mut wrap_row = 0;
        for line_item in &self.wrapper.lines {
            self.buffer_line_starts.push(wrap_row);
            wrap_row += line_item.lines_len();
        }

        self.cached_line_count = line_count;
        self.cached_wrap_row_count = wrap_row_count;
    }

    /// Çizim ve hit-test için alttaki sarmalayıcıya erişim döndürür.
    pub(crate) fn wrapper(&self) -> &TextWrapper {
        &self.wrapper
    }

    /// Çizim için satır öğelerine erişim döndürür.
    pub(crate) fn lines(&self) -> &[LineItem] {
        &self.wrapper.lines
    }

    /// Rope metnini döndürür.
    pub fn text(&self) -> &Rope {
        self.wrapper.text()
    }

    /// Bir tampon satırında kaç görünür (katlanmamış) sarma satırı olduğunu hesaplar.
    pub fn visible_wrap_row_count_for_line(&self, line: usize, fold_map: &FoldMap) -> usize {
        let wrap_range = self.buffer_line_to_wrap_row_range(line);
        wrap_range
            .filter(|&wr| fold_map.wrap_row_to_display_row(wr).is_some())
            .count()
    }
}
