/// FoldMap: Katlama projeksiyon katmanı (sarma satırları -> gösterim satırları).
///
/// Bu modül kod katlamayı şu şekilde yönetir:
/// - Katlanmış bölgelere ait sarma satırlarını filtreler
/// - Çift yönlü eşlemeyi korur: sarma_satırı <-> gösterim_satırı
/// - Katlama durumu değişikliklerini işler ve projeksiyonu yeniden oluşturur
use super::folding::FoldRange;
use super::wrap_map::WrapMap;

/// FoldMap, katlanmış bölgeleri gizleyerek sarma satırlarını gösterim satırlarına yansıtır.
pub struct FoldMap {
    /// Mapping: gösterim_satırı → sarma_satırı
    /// indeks = gösterim_satırı, değer = gerçek sarma_satırı
    visible_wrap_rows: Vec<usize>,

    /// Reverse eşleme: sarma_satırı → gösterim_satırı
    /// indeks = sarma_satırı, değer = Some(gösterim_satırı) ise görünür, None ise katlanmış
    wrap_row_to_display_row: Vec<Option<usize>>,

    /// Aday katlama aralıkları (tree-sitter/LSP kaynaklı).
    /// start_line değerine göre sıralıdır; start_line değerleri benzersizdir.
    candidates: Vec<FoldRange>,

    /// Şu anda katlanmış aralıklar.
    /// Adayların alt kümesidir ve start_line değerine göre sıralıdır.
    folded: Vec<FoldRange>,

    /// Katlama projeksiyonunun yeniden oluşturulması gerekip gerekmediğini belirten bayrak.
    /// Her metin değişiminde maliyetli yeniden oluşturmaları önlemek için tembel değerlendirmede kullanılır.
    needs_rebuild: bool,

    /// Son yeniden oluşturmadan kalan wrap_row_count önbelleği.
    /// WrapMap değiştiğinde yeniden oluşturma gerekip gerekmediğini algılamak için kullanılır.
    cached_wrap_row_count: usize,
}

impl FoldMap {
    pub fn new() -> Self {
        Self {
            visible_wrap_rows: Vec::new(),
            wrap_row_to_display_row: Vec::new(),
            candidates: Vec::new(),
            folded: Vec::new(),
            needs_rebuild: true,
            cached_wrap_row_count: 0,
        }
    }

    /// Tam yeniden oluşturma yapmadan önbellekteki wrap_row_count değerini günceller.
    /// Etkin katlama yokken kullanılır; özdeşlik eşlemesi varsayılır.
    pub(super) fn mark_dirty_with_wrap_count(&mut self, wrap_row_count: usize) {
        self.needs_rebuild = true;
        self.cached_wrap_row_count = wrap_row_count;
    }

    /// Toplam görünür gösterim satırı sayısını döndürür.
    pub fn display_row_count(&self) -> usize {
        if self.folded.is_empty() {
            return self.cached_wrap_row_count;
        }
        self.visible_wrap_rows.len()
    }

    /// sarma_satırı değerini gösterim_satırı değerine dönüştürür.
    /// sarma_satırı katlama nedeniyle gizliyse None döndürür.
    pub fn wrap_row_to_display_row(&self, wrap_row: usize) -> Option<usize> {
        if self.folded.is_empty() {
            return if wrap_row < self.cached_wrap_row_count {
                Some(wrap_row)
            } else {
                None
            };
        }
        self.wrap_row_to_display_row
            .get(wrap_row)
            .copied()
            .flatten()
    }

    /// gösterim_satırı değerini sarma_satırı değerine dönüştürür.
    pub fn display_row_to_wrap_row(&self, display_row: usize) -> Option<usize> {
        if self.folded.is_empty() {
            return if display_row < self.cached_wrap_row_count {
                Some(display_row)
            } else {
                None
            };
        }
        self.visible_wrap_rows.get(display_row).copied()
    }

    /// Verilen sarma_satırı için en yakın görünür gösterim_satırı değerini bulur.
    pub fn nearest_visible_display_row(&self, wrap_row: usize) -> usize {
        if self.folded.is_empty() {
            return wrap_row.min(self.cached_wrap_row_count.saturating_sub(1));
        }

        if let Some(dr) = self.wrap_row_to_display_row(wrap_row) {
            return dr;
        }

        match self.visible_wrap_rows.binary_search(&wrap_row) {
            Ok(idx) => idx,
            Err(insert_pos) => insert_pos.saturating_sub(1),
        }
    }

    /// tree-sitter/LSP kaynaklı katlama adaylarını tamamen değiştirerek ayarlar.
    pub fn set_candidates(&mut self, mut candidates: Vec<FoldRange>) {
        // Sort and deduplicate by start_line
        candidates.sort_by_key(|r| r.start_line);
        candidates.dedup_by_key(|r| r.start_line);
        self.candidates = candidates;

        // Remove any folded ranges that are no longer in candidates
        self.folded.retain(|fold| {
            self.candidates
                .iter()
                .any(|c| c.start_line == fold.start_line)
        });
    }

    /// Düzenlenen bölgeden çıkarılan yeni adayları mevcut adaylara ekler.
    ///
    /// [edit_start_line, edit_end_line] aralığındaki adayları `new_candidates` ile değiştirir,
    /// düzenleme aralığı dışındaki adayları korur.
    pub fn merge_candidates_for_edit(
        &mut self,
        edit_start_line: usize,
        edit_end_line: usize,
        new_candidates: Vec<FoldRange>,
    ) {
        // Remove old candidates within the edit range (already done by adjust_folds_for_edit)
        // But do it again in case adjust wasn't called or range differs
        self.candidates
            .retain(|c| c.start_line < edit_start_line || c.start_line > edit_end_line);

        // Add new candidates
        self.candidates.extend(new_candidates);
        self.candidates.sort_by_key(|r| r.start_line);
        self.candidates.dedup_by_key(|r| r.start_line);
    }

    /// Verilen start_line değerindeki katlamayı ayarlar; satır adaylar içinde olmalıdır.
    pub fn set_folded(&mut self, start_line: usize, folded: bool) {
        if folded {
            // Find the candidate range for this start_line
            if let Some(candidate) = self.candidates.iter().find(|c| c.start_line == start_line) {
                // Add to folded if not already present
                if !self.folded.iter().any(|f| f.start_line == start_line) {
                    self.folded.push(*candidate);
                    self.folded.sort_by_key(|r| r.start_line);
                    self.needs_rebuild = true;
                }
            }
        } else {
            // Remove from folded
            self.folded.retain(|f| f.start_line != start_line);
            self.needs_rebuild = true;
        }
    }

    /// Verilen start_line değerindeki katlamayı açıp kapatır.
    pub fn toggle_fold(&mut self, start_line: usize) {
        let is_folded = self.is_folded_at(start_line);
        self.set_folded(start_line, !is_folded);
    }

    /// Bir satırın şu anda katlanmış olup olmadığını kontrol eder.
    pub fn is_folded_at(&self, start_line: usize) -> bool {
        self.folded.iter().any(|f| f.start_line == start_line)
    }

    /// Bir satırın katlama adayı olup olmadığını kontrol eder.
    pub fn is_fold_candidate(&self, start_line: usize) -> bool {
        self.candidates.iter().any(|c| c.start_line == start_line)
    }

    /// Tüm katlama adaylarını döndürür.
    #[inline]
    pub fn fold_candidates(&self) -> &[FoldRange] {
        &self.candidates
    }

    /// Şu anda katlanmış tüm aralıkları döndürür.
    #[inline]
    pub fn folded_ranges(&self) -> &[FoldRange] {
        &self.folded
    }

    /// Tüm katlamaları temizler.
    #[inline]
    pub fn clear_folds(&mut self) {
        self.folded.clear();
    }

    /// Metin düzenlemesinden sonra katlamaları ve adayları ayarlar.
    ///
    /// - Düzenlenen satır aralığıyla çakışan katlamalar ve adaylar kaldırılır
    /// - Düzenlemeden sonraki katlamalar ve adaylar line_delta kadar kaydırılır
    ///
    /// Bu, her tuş vuruşunda maliyetli tam ağaç dolaşımını önler.
    pub fn adjust_folds_for_edit(
        &mut self,
        edit_start_line: usize,
        edit_end_line: usize,
        line_delta: isize,
    ) {
        // Adjust folded ranges
        if !self.folded.is_empty() {
            self.folded.retain(|fold| {
                !(fold.start_line <= edit_end_line && fold.end_line >= edit_start_line)
            });

            if line_delta != 0 {
                for fold in &mut self.folded {
                    if fold.start_line > edit_end_line {
                        fold.start_line = (fold.start_line as isize + line_delta).max(0) as usize;
                        fold.end_line = (fold.end_line as isize + line_delta).max(0) as usize;
                    }
                }
            }
        }

        // Adjust candidates the same way
        if !self.candidates.is_empty() {
            self.candidates
                .retain(|c| !(c.start_line <= edit_end_line && c.end_line >= edit_start_line));

            if line_delta != 0 {
                for c in &mut self.candidates {
                    if c.start_line > edit_end_line {
                        c.start_line = (c.start_line as isize + line_delta).max(0) as usize;
                        c.end_line = (c.end_line as isize + line_delta).max(0) as usize;
                    }
                }
            }
        }

        self.needs_rebuild = true;
    }

    /// wrap_map veya katlama durumu değiştikten sonra katlama eşlemesini yeniden oluşturur.
    ///
    /// Sarma satırlarını gösterim satırlarına yansıtan temel algoritmadır.
    pub fn rebuild(&mut self, wrap_map: &WrapMap) {
        let wrap_row_count = wrap_map.wrap_row_count();

        // Performance optimization: skip rebuild if nothing changed
        if !self.needs_rebuild && wrap_row_count == self.cached_wrap_row_count {
            return;
        }

        self.cached_wrap_row_count = wrap_row_count;

        self.visible_wrap_rows.clear();
        self.wrap_row_to_display_row = vec![None; wrap_row_count];

        if self.folded.is_empty() {
            // Fast path: no folds, all wrap rows are visible
            self.visible_wrap_rows = (0..wrap_row_count).collect();
            for (display_row, &wrap_row) in self.visible_wrap_rows.iter().enumerate() {
                self.wrap_row_to_display_row[wrap_row] = Some(display_row);
            }
            self.needs_rebuild = false;
            return;
        }

        // Build set of hidden wrap_row ranges from folded buffer lines
        let mut hidden_ranges = Vec::new();
        for fold in &self.folded {
            // Hide wrap rows from (start_line + 1) to (end_line - 1) (inclusive)
            // Both the first line and last line of the fold remain visible
            let hide_start_line = fold.start_line + 1;
            let hide_end_line = fold.end_line.saturating_sub(1);

            if hide_start_line > hide_end_line {
                continue; // No middle lines to hide (0 or 1 lines between start and end)
            }

            // Get wrap_row ranges for the hidden buffer lines
            let start_wrap_row = wrap_map.buffer_line_to_first_wrap_row(hide_start_line);
            let end_wrap_row = if hide_end_line + 1 < wrap_map.buffer_line_count() {
                wrap_map.buffer_line_to_first_wrap_row(hide_end_line + 1)
            } else {
                wrap_row_count
            };

            if start_wrap_row < end_wrap_row {
                hidden_ranges.push(start_wrap_row..end_wrap_row);
            }
        }

        // Merge overlapping hidden ranges
        hidden_ranges.sort_by_key(|r| r.start);
        let mut merged_hidden = Vec::new();
        for range in hidden_ranges {
            if let Some(last) = merged_hidden.last_mut() {
                if range.start <= *last {
                    // Overlapping or adjacent, merge
                    *last = (*last).max(range.end);
                } else {
                    merged_hidden.push(range.start);
                    merged_hidden.push(range.end);
                }
            } else {
                merged_hidden.push(range.start);
                merged_hidden.push(range.end);
            }
        }

        // Scan all wrap rows and filter out hidden ones
        let mut display_row = 0;
        let mut hidden_iter = merged_hidden.chunks_exact(2);
        let mut current_hidden = hidden_iter.next();

        for wrap_row in 0..wrap_row_count {
            // Check if wrap_row is in current hidden range
            let is_hidden = if let Some(&[start, end]) = current_hidden {
                if wrap_row >= end {
                    current_hidden = hidden_iter.next();
                    if let Some(&[new_start, new_end]) = current_hidden {
                        wrap_row >= new_start && wrap_row < new_end
                    } else {
                        false
                    }
                } else {
                    wrap_row >= start && wrap_row < end
                }
            } else {
                false
            };

            if !is_hidden {
                self.visible_wrap_rows.push(wrap_row);
                self.wrap_row_to_display_row[wrap_row] = Some(display_row);
                display_row += 1;
            }
        }

        self.needs_rebuild = false;
    }
}
