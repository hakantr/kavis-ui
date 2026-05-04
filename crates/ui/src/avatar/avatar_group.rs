use crate::ham_gpui::{
    Div, InteractiveElement, Interactivity, IntoElement, ParentElement as _, RenderOnce,
    StyleRefinement, Styled, div, prelude::FluentBuilder as _,
};

use crate::{BilesenBoyutu, Boyutlandirilabilir, EtkinTema, StilUzantisi as _, avatar::Avatar};

/// Bir grouped avatars göstermek için içinde bir kompakt yerleşim.
#[derive(IntoElement)]
pub struct AvatarGroup {
    base: Div,
    style: StyleRefinement,
    avatars: Vec<Avatar>,
    size: BilesenBoyutu,
    limit: usize,
    ellipsis: bool,
}

impl AvatarGroup {
    /// Yeni bir AvatarGroup oluşturur.
    pub fn new() -> Self {
        Self {
            base: div(),
            style: StyleRefinement::default(),
            avatars: Vec::new(),
            size: BilesenBoyutu::default(),
            limit: 3,
            ellipsis: false,
        }
    }

    /// Bir alt avatar için grup ekler.
    pub fn child(mut self, avatar: Avatar) -> Self {
        self.avatars.push(avatar);
        self
    }

    /// Birden çok alt avatars için grup ekler.
    pub fn children(mut self, avatars: impl IntoIterator<Item = Avatar>) -> Self {
        self.avatars.extend(avatars);
        self
    }

    /// "Daha fazla" avatarı gösterilmeden önce gösterilecek en fazla avatar sayısını ayarlar.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Sınıra ulaşıldığında üç nokta gösterilip gösterilmeyeceğini ayarlar. Varsayılan: false.
    pub fn ellipsis(mut self) -> Self {
        self.ellipsis = true;
        self
    }
}

impl Boyutlandirilabilir for AvatarGroup {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for AvatarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for AvatarGroup {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _: &mut crate::ham_gpui::Window, cx: &mut crate::ham_gpui::App) -> impl IntoElement {
        let item_ml = -super::avatar_size(self.size) * 0.3;
        let avatars_len = self.avatars.len();

        self.base
            .h_flex()
            .flex_row_reverse()
            .refine_style(&self.style)
            .children(if self.ellipsis && avatars_len > self.limit {
                Some(
                    Avatar::new()
                        .name("⋯")
                        .bg(cx.theme().secondary)
                        .text_color(cx.theme().muted_foreground)
                        .with_size(self.size)
                        .ml_1(),
                )
            } else {
                None
            })
            .children(
                self.avatars
                    .into_iter()
                    .take(self.limit)
                    .enumerate()
                    .rev()
                    .map(|(ix, item)| {
                        item.with_size(self.size)
                            .when(ix > 0, |this| this.ml(item_ml))
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::ham_gpui::test]
    fn test_avatar_group_builder(_cx: &mut crate::ham_gpui::TestAppContext) {
        let group = AvatarGroup::new()
            .child(Avatar::new().name("Alice"))
            .child(Avatar::new().name("Bob"))
            .child(Avatar::new().name("Charlie"))
            .child(Avatar::new().name("David"))
            .large()
            .limit(3)
            .ellipsis();

        assert_eq!(group.avatars.len(), 4);
        assert_eq!(group.size, BilesenBoyutu::Buyuk);
        assert_eq!(group.limit, 3);
        assert!(group.ellipsis);
    }
}
