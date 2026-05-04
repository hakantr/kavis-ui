use std::fmt::{Debug, Display};

use crate::ham_gpui::ElementId;

/// Bir listede bölüm indeksi, satır indeksi ve sütun indeksinden oluşan indeks yolunu temsil eder.
///
/// Bölüm, satır ve sütun için varsayılan değerler 0 olarak ayarlanır.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IndexPath {
    /// bölüm indeks.
    pub section: usize,
    /// öğe indeks içinde bölüm.
    pub row: usize,
    /// sütun indeks.
    pub column: usize,
}

impl From<IndexPath> for ElementId {
    fn from(path: IndexPath) -> Self {
        ElementId::Name(format!("index-path({},{},{})", path.section, path.row, path.column).into())
    }
}

impl Display for IndexPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IndexPath(section: {}, row: {}, column: {})",
            self.section, self.row, self.column
        )
    }
}

impl IndexPath {
    /// Yeni bir indeks yol ile belirtilen bölüm ve satır oluşturur.
    ///
    /// `section` varsayılan olarak 0 ayarlanır.
    /// `column` varsayılan olarak 0 ayarlanır.
    pub fn new(row: usize) -> Self {
        IndexPath {
            section: 0,
            row,
            ..Default::default()
        }
    }

    /// bölüm için indeks yol ayarlar.
    pub fn section(mut self, section: usize) -> Self {
        self.section = section;
        self
    }

    /// satır için indeks yol ayarlar.
    pub fn row(mut self, row: usize) -> Self {
        self.row = row;
        self
    }

    /// sütun için indeks yol ayarlar.
    pub fn column(mut self, column: usize) -> Self {
        self.column = column;
        self
    }

    /// Bu indeks yolunun verilen indeks yoluyla aynı bölüm ve satıra sahip olup olmadığını kontrol eder.
    pub fn eq_row(&self, index: IndexPath) -> bool {
        self.section == index.section && self.row == index.row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_element_id() {
        let index_path = IndexPath::new(2).section(1).column(3);
        let element_id: ElementId = index_path.into();
        assert_eq!(element_id.to_string(), "index-path(1,2,3)");
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", IndexPath::new(2).section(1).column(3)),
            "IndexPath(section: 1, row: 2, column: 3)"
        );
    }

    #[test]
    fn test_index_path() {
        let mut index_path = IndexPath::default();
        assert_eq!(index_path.section, 0);
        assert_eq!(index_path.row, 0);
        assert_eq!(index_path.column, 0);

        index_path = index_path.section(1).row(2).column(3);
        assert_eq!(index_path.section, 1);
        assert_eq!(index_path.row, 2);
        assert_eq!(index_path.column, 3);
    }
}
