use gpui::{
    App, IntoElement, ParentElement as _, SharedString, StyleRefinement, Styled, Window,
    prelude::FluentBuilder as _,
};

use crate::{
    EtkinTema, StilUzantisi,
    group_box::{GrupKutusu, GrupKutusuVaryantlari},
    label::Etiket,
    setting::{AyarOgesi, RenderOptions},
    v_flex,
};

/// Birden çok ayar öğesi içerebilen ayar grubu.
#[derive(Clone)]
pub struct AyarGrubu {
    style: StyleRefinement,

    pub(super) title: Option<SharedString>,
    pub(super) description: Option<SharedString>,
    pub(super) items: Vec<AyarOgesi>,
}

impl Styled for AyarGrubu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl AyarGrubu {
    /// Yeni bir ayar grubu oluşturur.
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            title: None,
            description: None,
            items: Vec::new(),
        }
    }

    /// Ayar grubunun etiketini ayarlar, varsayılan None değeridir.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Ayar grubunun açıklamasını ayarlar, varsayılan None değeridir.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Gruba bir ayar öğesi ekler.
    pub fn item(mut self, item: AyarOgesi) -> Self {
        self.items.push(item);
        self
    }

    /// Gruba birden çok ayar öğesi ekler.
    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = AyarOgesi>,
    {
        self.items.extend(items);
        self
    }

    /// Gruptaki ayar öğelerinden herhangi biri verilen sorguyla eşleşirse true döndürür.
    pub(super) fn is_match(&self, query: &str, cx: &App) -> bool {
        self.items.iter().any(|item| item.is_match(query, cx))
    }

    pub(super) fn is_resettable(&self, cx: &App) -> bool {
        self.items.iter().any(|item| item.is_resettable(cx))
    }

    pub(crate) fn render(
        self,
        query: &str,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        GrupKutusu::new()
            .id(SharedString::from(format!("group-{}", options.group_ix)))
            .with_variant(options.group_variant)
            .when_some(self.title.clone(), |this, title| {
                this.title(v_flex().gap_1().child(title).when_some(
                    self.description.clone(),
                    |this, description| {
                        this.child(
                            Etiket::new(description)
                                .text_sm()
                                .text_color(cx.theme().muted_foreground),
                        )
                    },
                ))
            })
            .gap_4()
            .children(self.items.iter().enumerate().filter_map(|(item_ix, item)| {
                if item.is_match(&query, cx) {
                    Some(item.clone().render_item(
                        &RenderOptions {
                            item_ix,
                            ..*options
                        },
                        window,
                        cx,
                    ))
                } else {
                    None
                }
            }))
            .refine_style(&self.style)
    }

    pub(crate) fn reset(&self, window: &mut Window, cx: &mut App) {
        for item in &self.items {
            item.reset(window, cx);
        }
    }
}
