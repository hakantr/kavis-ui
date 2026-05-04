use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, Secilebilir as _,
    button::{Dugme, DugmeGrubu},
    pagination::Sayfalama,
    v_flex,
};

use crate::section;

pub struct PaginationStory {
    basic_page: usize,
    many_pages_page: usize,
    compact_page: usize,
    focus_handle: FocusHandle,
    size: BilesenBoyutu,
}

impl super::Story for PaginationStory {
    fn title() -> &'static str {
        "Sayfalama"
    }

    fn description() -> &'static str {
        "Sayfalama with page navigation, next and previous links."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl PaginationStory {
    pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            basic_page: 5,
            many_pages_page: 1,
            compact_page: 3,
            focus_handle: cx.focus_handle(),
            size: BilesenBoyutu::default(),
        })
    }

    fn set_size(&mut self, size: BilesenBoyutu, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Focusable for PaginationStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PaginationStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        v_flex()
            .gap_6()
            .child(
                DugmeGrubu::new("toggle-size")
                    .outline()
                    .compact()
                    .child(
                        Dugme::new("xsmall")
                            .label("XSmall")
                            .selected(self.size == BilesenBoyutu::CokKucuk),
                    )
                    .child(
                        Dugme::new("small")
                            .label("Small")
                            .selected(self.size == BilesenBoyutu::Kucuk),
                    )
                    .child(
                        Dugme::new("medium")
                            .label("Medium")
                            .selected(self.size == BilesenBoyutu::Orta),
                    )
                    .child(
                        Dugme::new("large")
                            .label("Large")
                            .selected(self.size == BilesenBoyutu::Buyuk),
                    )
                    .on_click(cx.listener(|this, selecteds: &Vec<usize>, window, cx| {
                        let size = match selecteds[0] {
                            0 => BilesenBoyutu::CokKucuk,
                            1 => BilesenBoyutu::Kucuk,
                            2 => BilesenBoyutu::Orta,
                            3 => BilesenBoyutu::Buyuk,
                            _ => BilesenBoyutu::Orta,
                        };
                        this.set_size(size, window, cx);
                    })),
            )
            .child(
                section("Basic").child(
                    Sayfalama::new("basic-pagination")
                        .current_page(self.basic_page)
                        .total_pages(10)
                        .with_size(self.size)
                        .on_click({
                            let entity = entity.clone();
                            move |page, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.basic_page = *page;
                                    cx.notify();
                                });
                            }
                        }),
                ),
            )
            .child(
                section("Sayfalama with 10 visible pages").child(
                    Sayfalama::new("many-pages-pagination")
                        .current_page(self.many_pages_page)
                        .total_pages(50)
                        .visible_pages(10)
                        .with_size(self.size)
                        .on_click({
                            let entity = entity.clone();
                            move |page, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.many_pages_page = *page;
                                    cx.notify();
                                });
                            }
                        }),
                ),
            )
            .child(
                section("Compact Style").child(
                    Sayfalama::new("compact-pagination")
                        .compact()
                        .current_page(self.compact_page)
                        .total_pages(10)
                        .with_size(self.size)
                        .on_click({
                            let entity = entity.clone();
                            move |page, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.compact_page = *page;
                                    cx.notify();
                                });
                            }
                        }),
                ),
            )
            .child(
                section("Disabled").child(
                    Sayfalama::new("disabled-pagination")
                        .current_page(4)
                        .total_pages(10)
                        .with_size(self.size)
                        .disabled(true)
                        .on_click(|_, _, _| {}),
                ),
            )
    }
}
