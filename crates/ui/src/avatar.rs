mod avatar;
mod avatar_group;

pub use avatar::*;
pub use avatar_group::*;

use crate::{BilesenBoyutu, Simge, StilUzantisi as _};
use gpui::{Div, Img, IntoElement, Pixels, Styled, px, rems};

/// boyut avatar temelli üzerinde verilen [`BilesenBoyutu`] döndürür.
pub(super) fn avatar_size(size: BilesenBoyutu) -> Pixels {
    match size {
        BilesenBoyutu::Buyuk => px(80.),
        BilesenBoyutu::Orta => px(48.),
        BilesenBoyutu::Kucuk => px(24.),
        BilesenBoyutu::CokKucuk => px(16.),
        BilesenBoyutu::Ozel(size) => size,
    }
}

/// uzantı için ekler `avatar_size` yöntem için `IntoElement` için uygular avatar boyut için öğe.
pub(super) trait AvatarSized: IntoElement + Styled {
    fn avatar_size(self, size: BilesenBoyutu) -> Self {
        self.size(avatar_size(size))
    }

    fn avatar_text_size(self, size: BilesenBoyutu) -> Self {
        match size {
            BilesenBoyutu::Buyuk => self.text_3xl().font_semibold(),
            BilesenBoyutu::Orta => self.text_sm(),
            BilesenBoyutu::Kucuk => self.text_xs(),
            BilesenBoyutu::CokKucuk => self.text_size(rems(0.65)),
            BilesenBoyutu::Ozel(size) => self.size(size * 0.5),
        }
    }
}
impl AvatarSized for Div {}
impl AvatarSized for Simge {}
impl AvatarSized for Img {}
