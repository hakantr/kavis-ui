use crate::{Simge, Sizable, Size, progress::DaireselIlerleme, spinner::DonerGosterge};
use gpui::{App, IntoElement, RenderOnce, Window, prelude::FluentBuilder};

/// Dugme simge olan olabilir bir Simge, DonerGosterge, veya Ilerleme kullanmak için `simge` yöntem Dugme.
#[doc(hidden)]
#[derive(IntoElement)]
pub struct DugmeSimgesi {
    icon: DugmeSimgesiVaryanti,
    loading_icon: Option<Simge>,
    loading: bool,
    size: Size,
}

impl<T> From<T> for DugmeSimgesi
where
    T: Into<DugmeSimgesiVaryanti>,
{
    fn from(icon: T) -> Self {
        DugmeSimgesi::new(icon)
    }
}

impl DugmeSimgesi {
    /// Yeni bir DugmeSimgesi ile verilen simge oluşturur.
    pub fn new(icon: impl Into<DugmeSimgesiVaryanti>) -> Self {
        Self {
            icon: icon.into(),
            loading_icon: None,
            loading: false,
            size: Size::Medium,
        }
    }

    pub(crate) fn loading_icon(mut self, icon: Option<Simge>) -> Self {
        self.loading_icon = icon;
        self
    }

    pub(crate) fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl Sizable for DugmeSimgesi {
    fn with_size(mut self, size: impl Into<crate::Size>) -> Self {
        self.size = size.into();
        self
    }
}

/// Dugme simge olan olabilir bir Simge, DonerGosterge, Ilerleme, veya DaireselIlerleme kullanmak için `simge` yöntem Dugme.
#[doc(hidden)]
#[derive(IntoElement)]
pub enum DugmeSimgesiVaryanti {
    Simge(Simge),
    DonerGosterge(DonerGosterge),
    Ilerleme(DaireselIlerleme),
}

impl<T> From<T> for DugmeSimgesiVaryanti
where
    T: Into<Simge>,
{
    fn from(icon: T) -> Self {
        Self::Simge(icon.into())
    }
}

impl From<DonerGosterge> for DugmeSimgesiVaryanti {
    fn from(spinner: DonerGosterge) -> Self {
        Self::DonerGosterge(spinner)
    }
}

impl From<DaireselIlerleme> for DugmeSimgesiVaryanti {
    fn from(progress: DaireselIlerleme) -> Self {
        Self::Ilerleme(progress)
    }
}

impl DugmeSimgesiVaryanti {
    /// ButtonIconKind bir Simge ise true döndürür.
    #[inline]
    pub(crate) fn is_spinner(&self) -> bool {
        matches!(self, Self::DonerGosterge(_))
    }

    /// ButtonIconKind bir Ilerleme veya DaireselIlerleme ise true döndürür.
    #[inline]
    pub(crate) fn is_progress(&self) -> bool {
        matches!(self, Self::Ilerleme(_))
    }
}

impl Sizable for DugmeSimgesiVaryanti {
    fn with_size(self, size: impl Into<crate::Size>) -> Self {
        match self {
            Self::Simge(icon) => Self::Simge(icon.with_size(size)),
            Self::DonerGosterge(spinner) => Self::DonerGosterge(spinner.with_size(size)),
            Self::Ilerleme(progress) => Self::Ilerleme(progress.with_size(size)),
        }
    }
}

impl RenderOnce for DugmeSimgesiVaryanti {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            Self::Simge(icon) => icon.into_any_element(),
            Self::DonerGosterge(spinner) => spinner.into_any_element(),
            Self::Ilerleme(progress) => progress.into_any_element(),
        }
    }
}

impl RenderOnce for DugmeSimgesi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        if self.loading {
            if self.icon.is_spinner() || self.icon.is_progress() {
                self.icon.with_size(self.size).into_any_element()
            } else {
                DonerGosterge::new()
                    .when_some(self.loading_icon, |this, icon| this.icon(icon))
                    .with_size(self.size)
                    .into_any_element()
            }
        } else {
            self.icon.with_size(self.size).into_any_element()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimgeAdi;

    #[gpui::test]
    fn test_button_icon_builder(_cx: &mut gpui::TestAppContext) {
        let custom_icon = Simge::new(SimgeAdi::Loader);
        let icon = DugmeSimgesi::new(SimgeAdi::Plus)
            .loading(true)
            .loading_icon(Some(custom_icon))
            .large();

        assert!(icon.loading);
        assert!(icon.loading_icon.is_some());
        assert_eq!(icon.size, Size::Large);
    }

    #[gpui::test]
    fn test_button_icon_variant_types(_cx: &mut gpui::TestAppContext) {
        // Test Simge variant
        let icon_variant = DugmeSimgesiVaryanti::Simge(Simge::new(SimgeAdi::Plus));
        assert!(!icon_variant.is_spinner());
        assert!(!icon_variant.is_progress());

        // Test DonerGosterge variant
        let spinner_variant = DugmeSimgesiVaryanti::DonerGosterge(DonerGosterge::new());
        assert!(spinner_variant.is_spinner());
        assert!(!spinner_variant.is_progress());

        // Test Ilerleme variant
        let progress_variant = DugmeSimgesiVaryanti::Ilerleme(DaireselIlerleme::new(75));
        assert!(!progress_variant.is_spinner());
        assert!(progress_variant.is_progress());
    }
}
