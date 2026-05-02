use std::ops::Range;

#[cfg(not(target_family = "wasm"))]
use tree_sitter::Node;
#[cfg(not(target_family = "wasm"))]
pub use tree_sitter::Tree as Agac;

#[cfg(target_family = "wasm")]
/// WASM üzerinde tree-sitter kullanılamadığı için Agac yerine kullanılan yer tutucu tip.
pub struct Agac;

#[cfg(not(target_family = "wasm"))]
/// Bir düğümün katlanabilir sayılması için gereken minimum satır aralığı.
const MIN_FOLD_LINES: usize = 2;

/// Katlanabilir bir kod bölgesini temsil eden katlama aralığı.
///
/// Katlama aralığı start_line ile end_line arasını kapsar (dahil).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FoldRange {
    /// Başlangıç satır (dahil)
    pub start_line: usize,
    /// Bitiş satır (dahil)
    pub end_line: usize,
}

impl FoldRange {
    pub fn new(start_line: usize, end_line: usize) -> Self {
        assert!(
            start_line <= end_line,
            "fold start_line must be <= end_line"
        );
        Self {
            start_line,
            end_line,
        }
    }
}

// ==================== Native Implementation (with tree-sitter) ====================

#[cfg(not(target_family = "wasm"))]
/// Adlandırılmış bir düğümün katlama adayı olup olmadığını kontrol eder.
///
/// Yapısal bir kestirim kullanır: MIN_FOLD_LINES veya daha fazla satıra yayılan her **adlandırılmış** düğüm
/// katlanabilir sayılır. tree-sitter kodu zaten semantik birimlere (fonksiyonlar,
/// sınıflar, bloklar vb.) ayırdığı için adlandırılmış düğümler doğal olarak anlamlı
/// katlanabilir bölgelere karşılık gelir; dil başına düğüm tipi listesi gerekmez.
fn is_foldable_node(node: &Node) -> bool {
    // Skip root node (e.g. `source_file`) and unnamed tokens
    if !node.is_named() || node.parent().is_none() {
        return false;
    }

    let start = node.start_position().row;
    let end = node.end_position().row;
    end.saturating_sub(start) >= MIN_FOLD_LINES
}

#[cfg(not(target_family = "wasm"))]
/// tree-sitter sözdizimi ağacından katlama aralıklarını çıkarır (tam dolaşım).
pub fn extract_fold_ranges(tree: &Agac) -> Vec<FoldRange> {
    let mut ranges = Vec::new();
    collect_foldable_nodes(tree.root_node(), &mut ranges);

    ranges.sort_by_key(|r| r.start_line);
    ranges.dedup_by_key(|r| r.start_line);
    ranges
}

#[cfg(not(target_family = "wasm"))]
/// Katlama aralıklarını yalnızca belirli bir bayt aralığından çıkarır (düzenlemelerden sonra artımlı güncelleme için).
///
/// Aralık dışında kalan alt ağaçları tamamen atlar; böylece işlem aralıktaki düğüm sayısı kadar olur
/// ve tüm ağaçtaki düğüm sayısına bağlı kalmaz.
pub fn extract_fold_ranges_in_range(tree: &Agac, byte_range: Range<usize>) -> Vec<FoldRange> {
    let mut ranges = Vec::new();
    collect_foldable_nodes_in_range(tree.root_node(), &byte_range, &mut ranges);

    ranges.sort_by_key(|r| r.start_line);
    ranges.dedup_by_key(|r| r.start_line);
    ranges
}

#[cfg(not(target_family = "wasm"))]
/// byte_range dışında kalan alt ağaçları atlayarak katlanabilir düğümleri özyinelemeli toplar.
fn collect_foldable_nodes_in_range(
    node: Node,
    byte_range: &Range<usize>,
    ranges: &mut Vec<FoldRange>,
) {
    if node.end_byte() <= byte_range.start || node.start_byte() >= byte_range.end {
        return;
    }

    if is_foldable_node(&node) {
        ranges.push(FoldRange {
            start_line: node.start_position().row,
            end_line: node.end_position().row,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_foldable_nodes_in_range(child, byte_range, ranges);
    }
}

#[cfg(not(target_family = "wasm"))]
/// Sözdizimi ağacından katlanabilir düğümleri özyinelemeli toplar (tam dolaşım).
fn collect_foldable_nodes(node: Node, ranges: &mut Vec<FoldRange>) {
    if is_foldable_node(&node) {
        ranges.push(FoldRange {
            start_line: node.start_position().row,
            end_line: node.end_position().row,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_foldable_nodes(child, ranges);
    }
}

// ==================== WASM Stub Implementation ====================

#[cfg(target_family = "wasm")]
/// Katlama aralıklarını çıkarır; WASM yer tutucusu boş sonuç döndürür çünkü tree-sitter yoktur.
pub fn extract_fold_ranges(_tree: &Agac) -> Vec<FoldRange> {
    Vec::new()
}

#[cfg(target_family = "wasm")]
/// Aralıktaki katlama aralıklarını çıkarır; WASM yer tutucusu boş sonuç döndürür çünkü tree-sitter yoktur.
pub fn extract_fold_ranges_in_range(_tree: &Agac, _byte_range: Range<usize>) -> Vec<FoldRange> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_range_ordering() {
        let mut ranges = vec![
            FoldRange {
                start_line: 10,
                end_line: 20,
            },
            FoldRange {
                start_line: 5,
                end_line: 15,
            },
            FoldRange {
                start_line: 5,
                end_line: 15,
            },
            FoldRange {
                start_line: 1,
                end_line: 30,
            },
        ];

        ranges.sort_by_key(|r| r.start_line);
        ranges.dedup_by_key(|r| r.start_line);

        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].start_line, 1);
        assert_eq!(ranges[1].start_line, 5);
        assert_eq!(ranges[2].start_line, 10);
    }
}
