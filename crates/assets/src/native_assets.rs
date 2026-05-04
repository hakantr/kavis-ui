use anyhow::anyhow;
use kavis_ui::{AssetSource, Result, SharedString};
use std::borrow::Cow;

/// RustEmbed kullanan yerel uygulama
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
pub struct Varliklar;

impl Varliklar {
    /// Yeni bir Varliklar örneği oluşturur. endpoint parametresi yerel derlemelerde yok sayılır.
    pub fn new(_endpoint: impl Into<SharedString>) -> Self {
        Self
    }
}

impl AssetSource for Varliklar {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
