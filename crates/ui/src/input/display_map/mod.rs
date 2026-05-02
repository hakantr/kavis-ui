/// Gösterim eşleme sistem için düzenleyici/girdi.
///
/// Bu modül katmanlı bir gösterim eşleme mimarisi uygular:
/// - **WrapMap**: Yumuşak sarmayı işler (tampon satırları -> sarma satırları)
/// - **FoldMap**: İşler katlama (sarma satırlar → gösterim satırlar)
/// - **DisplayMap**: Düzenleyici/girdi için genel cephe
///
/// Amaç, düzenleyicinin yalnızca bilmesi gereken temiz ve birleşik bir API sağlamaktır.
/// about `BufferPoint ↔ DisplayPoint` eşleme, olmadan worrying about dahili sarma/katlama complexity.
mod display_map;
mod fold_map;
#[cfg(not(target_family = "wasm"))]
mod folding;
#[cfg(target_family = "wasm")]
pub mod folding;
mod text_wrapper;
mod wrap_map;

// Re-export public API
pub use self::display_map::DisplayMap;
pub(crate) use self::text_wrapper::LineLayout;

// Re-export FoldRange and extract_fold_ranges
pub use folding::{FoldRange, extract_fold_ranges};

/// konum içinde tampon (mantıksal metin).
///
/// - `line`: 0-temelli mantıksal satır sayı (bölme ile `\n`)
/// - `col`: 0-temelli sütun ofset (bayt ofset)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferPoint {
    pub line: usize,
    pub col: usize,
}

impl BufferPoint {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// konum sonra yumuşak-sarma ama önce katlama (dahili).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct WrapPoint {
    pub row: usize,
    pub col: usize,
}

impl WrapPoint {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

/// Final gösterim konum (sonra yumuşak-sarma ve katlama).
///
/// - `row`: 0-temelli gösterim satır (son görünür satır)
/// - `col`: 0-temelli gösterim sütun
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayPoint {
    pub row: usize,
    pub col: usize,
}

impl DisplayPoint {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
