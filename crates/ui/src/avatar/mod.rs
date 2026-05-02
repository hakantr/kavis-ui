mod avatar;
mod avatar_group;

pub use avatar::*;
pub use avatar_group::*;

use crate::{Simge, Size, StyledExt as _};
use gpui::{Div, Img, IntoElement, Pixels, Styled, px, rems};

/// boyut avatar temelli üzerinde verilen [`Size`] döndürür.
pub(super) fn avatar_size(size: Size) -> Pixels {
    match size {
        Size::Large => px(80.),
        Size::Medium => px(48.),
        Size::Small => px(24.),
        Size::XSmall => px(16.),
        Size::Size(size) => size,
    }
}

/// uzantı için ekler `avatar_size` yöntem için `IntoElement` için uygular avatar boyut için öğe.
pub(super) trait AvatarSized: IntoElement + Styled {
    fn avatar_size(self, size: Size) -> Self {
        self.size(avatar_size(size))
    }

    fn avatar_text_size(self, size: Size) -> Self {
        match size {
            Size::Large => self.text_3xl().font_semibold(),
            Size::Medium => self.text_sm(),
            Size::Small => self.text_xs(),
            Size::XSmall => self.text_size(rems(0.65)),
            Size::Size(size) => self.size(size * 0.5),
        }
    }
}
impl AvatarSized for Div {}
impl AvatarSized for Simge {}
impl AvatarSized for Img {}
