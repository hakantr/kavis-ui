pub(crate) mod cache;
mod delegate;
mod list;
mod list_item;
mod loading;
mod separator_item;

pub use delegate::*;
pub use list::*;
pub use list_item::*;
use schemars::JsonSchema;
pub use separator_item::*;
use serde::{Deserialize, Serialize};

/// Ayarlar için Liste.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListeAyarlari {
    /// Olup olmadığını için kullanım etkin vurgulama stil üzerinde ListeOgesi, varsayılan
    pub active_highlight: bool,
}

impl Default for ListeAyarlari {
    fn default() -> Self {
        Self {
            active_highlight: true,
        }
    }
}
