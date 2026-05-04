use crate::ham_gpui::{
    App, Entity, InteractiveElement as _, IntoElement, ListAlignment, ListState as ListeDurumu,
    ParentElement as _, SharedString, StyleRefinement, Styled, Window, div, list,
    prelude::FluentBuilder as _, px,
};
use rust_i18n::t;

use crate::{
    Boyutlandirilabilir, EtkinTema, Simge, SimgeAdi, StilUzantisi,
    button::{Dugme, DugmeVaryantlari},
    h_flex,
    label::Etiket,
    scroll::KaydirilabilirOge,
    setting::{AyarGrubu, RenderOptions, settings::AyarlarDurumu},
    v_flex,
};

/// Birden çok ayar grubu içerebilen ayar sayfası.
#[derive(Clone)]
pub struct AyarSayfasi {
    pub(super) icon: Option<Simge>,
    resettable: bool,
    pub(super) default_open: bool,
    pub(super) title: SharedString,
    pub(super) description: Option<SharedString>,
    pub(super) groups: Vec<AyarGrubu>,
    pub(super) header_style: StyleRefinement,
}

impl AyarSayfasi {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            icon: None,
            resettable: true,
            default_open: false,
            title: title.into(),
            description: None,
            groups: Vec::new(),
            header_style: StyleRefinement::default(),
        }
    }

    /// Ayar sayfasının başlığını ayarlar.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Ayar sayfasının simgesini ayarlar.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Ayar sayfasının açıklamasını ayarlar, varsayılan None değeridir.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Ayar sayfasının varsayılan açık durumunu ayarlar, varsayılan false değeridir.
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Ayar sayfasının sıfırlanabilir olup olmadığını ayarlar, varsayılan true değeridir.
    ///
    /// True ise ve bu sayfadaki öğeler değiştiyse sıfırlama düğmesi görünür.
    pub fn resettable(mut self, resettable: bool) -> Self {
        self.resettable = resettable;
        self
    }

    /// Sayfaya bir ayar grubu ekler.
    pub fn group(mut self, group: AyarGrubu) -> Self {
        self.groups.push(group);
        self
    }

    /// Sayfaya birden çok ayar grubu ekler.
    pub fn groups(mut self, groups: impl IntoIterator<Item = AyarGrubu>) -> Self {
        self.groups.extend(groups);
        self
    }

    /// Ayar sayfası başlığı için stil iyileştirmesini ayarlar.
    pub fn header_style(mut self, style: &StyleRefinement) -> Self {
        self.header_style = style.clone();
        self
    }

    fn is_resettable(&self, cx: &App) -> bool {
        self.resettable && self.groups.iter().any(|group| group.is_resettable(cx))
    }

    fn reset_all(&self, window: &mut Window, cx: &mut App) {
        for group in &self.groups {
            group.reset(window, cx);
        }
    }

    pub(super) fn render(
        &self,
        ix: usize,
        state: &Entity<AyarlarDurumu>,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let search_input = state.read(cx).search_input.clone();
        let query = search_input.read(cx).value();
        let groups = self
            .groups
            .iter()
            .filter(|group| group.is_match(&query, cx))
            .cloned()
            .collect::<Vec<_>>();
        let groups_count = groups.len();

        let list_state = window
            .use_keyed_state(
                SharedString::from(format!("list-state:{}", ix)),
                cx,
                |_, _| ListeDurumu::new(groups_count, ListAlignment::Top, px(100.)),
            )
            .read(cx)
            .clone();

        if list_state.item_count() != groups_count {
            list_state.reset(groups_count);
        }

        let deferred_scroll_group_ix = state.read(cx).deferred_scroll_group_ix;
        if let Some(ix) = deferred_scroll_group_ix {
            state.update(cx, |state, _| {
                state.deferred_scroll_group_ix = None;
            });
            list_state.scroll_to_reveal_item(ix);
        }

        v_flex()
            .id(ix)
            .size_full()
            .child(
                v_flex()
                    .p_4()
                    .gap_3()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .refine_style(&self.header_style)
                    .child(h_flex().justify_between().child(self.title.clone()).when(
                        self.is_resettable(cx),
                        |this| {
                            this.child(
                                Dugme::new("reset")
                                    .icon(SimgeAdi::Undo2)
                                    .ghost()
                                    .small()
                                    .tooltip(t!("Ayarlar.Reset All"))
                                    .on_click({
                                        let page = self.clone();
                                        move |_, window, cx| {
                                            page.reset_all(window, cx);
                                        }
                                    }),
                            )
                        },
                    ))
                    .when_some(self.description.clone(), |this, description| {
                        this.child(
                            Etiket::new(description)
                                .text_sm()
                                .text_color(cx.theme().muted_foreground),
                        )
                    }),
            )
            .child(
                div()
                    .px_4()
                    .relative()
                    .flex_1()
                    .w_full()
                    .child(
                        list(list_state.clone(), {
                            let query = query.clone();
                            let options = *options;
                            move |group_ix, window, cx| {
                                let group = groups[group_ix].clone();
                                group
                                    .py_4()
                                    .render(
                                        &query,
                                        &RenderOptions {
                                            page_ix: ix,
                                            group_ix,
                                            ..options
                                        },
                                        window,
                                        cx,
                                    )
                                    .into_any_element()
                            }
                        })
                        .size_full(),
                    )
                    .vertical_scrollbar(&list_state),
            )
    }
}
