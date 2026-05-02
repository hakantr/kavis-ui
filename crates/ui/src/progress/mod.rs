mod progress;
mod progress_circle;

pub use progress::Ilerleme;
pub use progress_circle::DaireselIlerleme;

use std::cell::Cell;

/// Shared durum için ilerleme bileşenler.
///
/// Tracks animasyon "den" değer (`değer`) ve en son hedef
/// (`target`). `target`, hemen güncellenebilmek için `Cell` kullanır
/// sırasında çizer olmadan triggering bir re-çizer bildirim.
pub(crate) struct IlerlemeDurumu {
    /// "den" değer için geçerli animasyon; güncellenir ile async
    /// Animasyon tamamlandığında zamanlayıcı tarafından güncellenir.
    pub(crate) value: f32,
    /// en son animasyon hedef, güncellenir hemen aracılığıyla interior
    /// mutability böylece bayat timers her zaman read up-için-tarih hedef.
    target: Cell<f32>,
}

impl IlerlemeDurumu {
    pub(crate) fn new(value: f32) -> Self {
        Self {
            value,
            target: Cell::new(value),
        }
    }

    pub(crate) fn target(&self) -> f32 {
        self.target.get()
    }

    pub(crate) fn set_target(&self, value: f32) {
        self.target.set(value);
    }
}
