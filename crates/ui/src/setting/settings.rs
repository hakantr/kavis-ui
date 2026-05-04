use crate::{
    BilesenBoyutu, Boyutlandirilabilir, SimgeAdi, StilUzantisi,
    group_box::GrupKutusuVaryanti,
    input::{Input, InputState},
    resizable::{h_resizable, resizable_panel},
    setting::{AyarGrubu, AyarSayfasi},
    sidebar::{YanCubuk, YanCubukMenuOgesi, YanCubukMenusu},
};
use gpui::{
    App, AppContext as _, Axis, ElementId, Entity, IntoElement, ParentElement as _, Pixels,
    RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px, relative,
};
use rust_i18n::t;

/// ayarlar structure containing çoklu pages için uygulama ayarlar.
///
/// Ayar hiyerarşisi şöyledir:
///
/// ```ignore
/// Ayarlar
///   AyarSayfasi     <- The single active page displayed
///     AyarGrubu
///       AyarOgesi
///         Etiket
///         AyarAlani (e.g., Anahtar, Dropdown, Input)
/// ```
#[derive(IntoElement)]
pub struct Ayarlar {
    id: ElementId,
    pages: Vec<AyarSayfasi>,
    group_variant: GrupKutusuVaryanti,
    size: BilesenBoyutu,
    sidebar_width: Pixels,
    sidebar_style: StyleRefinement,
    default_selected_index: SecimIndeksi,
    header_style: StyleRefinement,
}

impl Ayarlar {
    /// Yeni bir ayarlar ile verilen ID oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pages: vec![],
            group_variant: GrupKutusuVaryanti::default(),
            size: BilesenBoyutu::default(),
            sidebar_width: px(250.0),
            sidebar_style: StyleRefinement::default(),
            default_selected_index: SecimIndeksi::default(),
            header_style: StyleRefinement::default(),
        }
    }

    /// Yan çubuğun genişliğini ayarlar. Varsayılan `250px` değeridir.
    pub fn sidebar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.sidebar_width = width.into();
        self
    }

    /// Bir sayfa için ayarlar ekler.
    pub fn page(mut self, page: AyarSayfasi) -> Self {
        self.pages.push(page);
        self
    }

    /// pages için ayarlar. ekler.
    pub fn pages(mut self, pages: impl IntoIterator<Item = AyarSayfasi>) -> Self {
        self.pages.extend(pages);
        self
    }

    /// varsayılan varyant için tüm ayar gruplar ayarlar.
    ///
    /// Tek tek geçersiz kılınmadıkça tüm ayar grupları bu varyantı kullanır.
    pub fn with_group_variant(mut self, variant: GrupKutusuVaryanti) -> Self {
        self.group_variant = variant;
        self
    }

    /// stil refinement için sidebar ayarlar.
    pub fn sidebar_style(mut self, style: &StyleRefinement) -> Self {
        self.sidebar_style = style.clone();
        self
    }

    /// varsayılan indeks sayfa olmak için seçili ayarlar.
    pub fn default_selected_index(mut self, index: SecimIndeksi) -> Self {
        self.default_selected_index = index;
        self
    }

    /// stil refinement için başlık ayarlar.
    pub fn header_style(mut self, style: &StyleRefinement) -> Self {
        self.header_style = style.clone();
        self
    }

    fn filtered_pages(&self, query: &str, cx: &App) -> Vec<AyarSayfasi> {
        self.pages
            .iter()
            .filter_map(|page| {
                let filtered_groups: Vec<AyarGrubu> = page
                    .groups
                    .iter()
                    .filter_map(|group| {
                        let mut group = group.clone();
                        group.items = group
                            .items
                            .iter()
                            .filter(|item| item.is_match(&query, cx))
                            .cloned()
                            .collect();
                        if group.items.is_empty() {
                            None
                        } else {
                            Some(group)
                        }
                    })
                    .collect();
                let mut page = page.clone();
                page.groups = filtered_groups;
                if page.groups.is_empty() {
                    None
                } else {
                    Some(page)
                }
            })
            .collect()
    }

    fn render_active_page(
        &self,
        state: &Entity<AyarlarDurumu>,
        pages: &Vec<AyarSayfasi>,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let selected_index = state.read(cx).selected_index;

        for (ix, page) in pages.into_iter().enumerate() {
            if selected_index.page_ix == ix {
                return page
                    .render(ix, state, &options, window, cx)
                    .into_any_element();
            }
        }

        return div().into_any_element();
    }

    fn render_sidebar(
        &self,
        state: &Entity<AyarlarDurumu>,
        pages: &Vec<AyarSayfasi>,
        _: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let selected_index = state.read(cx).selected_index;
        let search_input = state.read(cx).search_input.clone();

        YanCubuk::new("settings-sidebar")
            .w(relative(1.))
            .border_0()
            .refine_style(&self.sidebar_style)
            .collapsible(false)
            .collapsed(false)
            .header(
                div()
                    .w_full()
                    .refine_style(&self.header_style)
                    .child(Input::new(&search_input).prefix(SimgeAdi::Search)),
            )
            .child(YanCubukMenusu::new().children(pages.iter().enumerate().map(
                |(page_ix, page)| {
                    let is_page_active =
                        selected_index.page_ix == page_ix && selected_index.group_ix.is_none();
                    YanCubukMenuOgesi::new(page.title.clone())
                        .click_to_open(true)
                        .when_some(page.icon.clone(), |this, icon| this.icon(icon))
                        .default_open(page.default_open)
                        .active(is_page_active)
                        .on_click({
                            let state = state.clone();
                            move |_, _, cx| {
                                state.update(cx, |state, cx| {
                                    state.selected_index = SecimIndeksi {
                                        page_ix,
                                        ..Default::default()
                                    };
                                    cx.notify();
                                })
                            }
                        })
                        .when(page.groups.len() > 1, |this| {
                            this.children(
                                page.groups
                                    .iter()
                                    .filter(|g| g.title.is_some())
                                    .enumerate()
                                    .map(|(group_ix, group)| {
                                        let is_active = selected_index.page_ix == page_ix
                                            && selected_index.group_ix == Some(group_ix);
                                        let title = group.title.clone().unwrap_or_default();

                                        YanCubukMenuOgesi::new(title).active(is_active).on_click({
                                            let state = state.clone();
                                            move |_, _, cx| {
                                                state.update(cx, |state, cx| {
                                                    state.selected_index = SecimIndeksi {
                                                        page_ix,
                                                        group_ix: Some(group_ix),
                                                    };
                                                    state.deferred_scroll_group_ix = Some(group_ix);
                                                    cx.notify();
                                                })
                                            }
                                        })
                                    }),
                            )
                        })
                },
            )))
    }
}

impl Boyutlandirilabilir for Ayarlar {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

pub(super) struct AyarlarDurumu {
    pub(super) selected_index: SecimIndeksi,
    /// ayarlar ise, defer kaydırma için bu grup indeks sonra çizim.
    pub(super) deferred_scroll_group_ix: Option<usize>,
    pub(super) search_input: Entity<InputState>,
}

/// Ayar öğesini çizmek için seçenekler.
#[derive(Clone, Copy)]
pub struct RenderOptions {
    pub page_ix: usize,
    pub group_ix: usize,
    pub item_ix: usize,
    pub size: BilesenBoyutu,
    pub group_variant: GrupKutusuVaryanti,
    pub layout: Axis,
}

#[derive(Clone, Copy, Default)]
pub struct SecimIndeksi {
    pub page_ix: usize,
    pub group_ix: Option<usize>,
}

impl RenderOnce for Ayarlar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
            let search_input = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder(t!("Ayarlar.search_placeholder"))
                    .default_value("")
            });

            AyarlarDurumu {
                search_input,
                selected_index: self.default_selected_index,
                deferred_scroll_group_ix: None,
            }
        });

        let query = state.read(cx).search_input.read(cx).value();
        let filtered_pages = self.filtered_pages(&query, cx);
        let options = RenderOptions {
            page_ix: 0,
            group_ix: 0,
            item_ix: 0,
            size: self.size,
            group_variant: self.group_variant,
            layout: Axis::Horizontal,
        };

        h_resizable(self.id.clone())
            .child(
                resizable_panel()
                    .size(self.sidebar_width)
                    .child(self.render_sidebar(&state, &filtered_pages, window, cx)),
            )
            .child(resizable_panel().child(self.render_active_page(
                &state,
                &filtered_pages,
                &options,
                window,
                cx,
            )))
    }
}
