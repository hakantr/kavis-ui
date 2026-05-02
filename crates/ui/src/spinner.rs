use crate::{Simge, SimgeAdi, Sizable, Size};
use gpui::{
    Animation, AnimationExt as _, App, Hsla, IntoElement, ParentElement, RenderOnce, Styled as _,
    Transformation, Window, div, ease_in_out, percentage, prelude::FluentBuilder as _,
};
use instant::Duration;

/// Bir cycling yükleme spinner.
#[derive(IntoElement)]
pub struct DonerGosterge {
    size: Size,
    icon: Simge,
    speed: Duration,
    color: Option<Hsla>,
}

impl DonerGosterge {
    /// Yeni bir yükleme spinner oluşturur.
    pub fn new() -> Self {
        Self {
            size: Size::Medium,
            speed: Duration::from_secs_f64(0.8),
            icon: Simge::new(SimgeAdi::Loader),
            color: None,
        }
    }

    /// belirtilen simge için spinner ayarlar.
    ///
    /// Varsayılan [`SimgeAdi::Loader`] değeridir.
    ///
    /// Kullanılan simgenin yükleme spinner'ı için uygun olduğundan emin olun.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = icon.into();
        self
    }

    /// simge renk ayarlar.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Sizable for DonerGosterge {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for DonerGosterge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .child(
                self.icon
                    .with_size(self.size)
                    .when_some(self.color, |this, color| this.text_color(color))
                    .with_animation(
                        "circle",
                        Animation::new(self.speed).repeat().with_easing(ease_in_out),
                        |this, delta| this.transform(Transformation::rotate(percentage(delta))),
                    ),
            )
            .into_element()
    }
}
