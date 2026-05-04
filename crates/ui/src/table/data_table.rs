use crate::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema,
    actions::{
        Cancel, SelectDown, SelectFirst, SelectLast, SelectNextColumn, SelectPageDown,
        SelectPageUp, SelectPrevColumn, SelectUp,
    },
    table::{TabloDurumu, TabloTemsilcisi},
};
use crate::ham_gpui::{
    App, Edges, Entity, Focusable, InteractiveElement, IntoElement, KeyBinding, ParentElement,
    RenderOnce, Styled, Window, div, prelude::FluentBuilder,
};

const CONTEXT: &'static str = "VeriTablosu";
pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", Cancel, Some(CONTEXT)),
        KeyBinding::new("up", SelectUp, Some(CONTEXT)),
        KeyBinding::new("down", SelectDown, Some(CONTEXT)),
        KeyBinding::new("left", SelectPrevColumn, Some(CONTEXT)),
        KeyBinding::new("right", SelectNextColumn, Some(CONTEXT)),
        KeyBinding::new("home", SelectFirst, Some(CONTEXT)),
        KeyBinding::new("end", SelectLast, Some(CONTEXT)),
        KeyBinding::new("pageup", SelectPageUp, Some(CONTEXT)),
        KeyBinding::new("pagedown", SelectPageDown, Some(CONTEXT)),
        KeyBinding::new("tab", SelectNextColumn, Some(CONTEXT)),
        KeyBinding::new("shift-tab", SelectPrevColumn, Some(CONTEXT)),
    ]);
}

pub(super) struct TabloSecenekleri {
    pub(super) scrollbar_visible: Edges<bool>,
    /// stripe stil tablo ayarlar.
    pub(super) stripe: bool,
    /// kullanmak için kenarlık stil tablo ayarlar.
    pub(super) bordered: bool,
    /// hücre boyut tablo.
    pub(super) size: BilesenBoyutu,
}

impl Default for TabloSecenekleri {
    fn default() -> Self {
        Self {
            scrollbar_visible: Edges::all(true),
            stripe: false,
            bordered: true,
            size: BilesenBoyutu::default(),
        }
    }
}

/// Bir tablo öğe ile destek için satır, sütun, ve hücre seçim.
///
/// # Özellikler
///
/// - **Çoklu Seçim Modları**: Satır, sütun ve hücre seçimini destekler
/// - **Hücre Seçimi**: Klavye gezinmesiyle tek tek hücreleri tıklayarak seçme
/// - **Sanal Kaydırma**: Büyük veri kümelerini verimli çizer
/// - **Yeniden Boyutlandırılabilir Sütunlar**: Sütun kenarlarını sürükleyerek yeniden boyutlandırma sağlar
/// - **Taşınabilir Sütunlar**: Sütun başlıklarını sürükleyerek yeniden sıralama sağlar
/// - **Sabit Sütunlar**: Sütunları sol tarafa sabitler
/// - **Sıralanabilir Sütunlar**: Sütun başlıklarına tıklayarak sıralama sağlar
/// - **Context Menus**: Right-tıklama destek için satırlar ve hücreler
///
/// # Hücre Seçim Modu
///
/// [`TabloDurumu::cell_selectable()`] ile hücre seçimi etkin olduğunda:
/// - Click üzerinde hücreler için seçim them
/// - Tüm satırları seçmek için solda bir satır seçici sütun görünür
/// - Klavye gezinmesi (ok tuşları, Sekme, Home, Bitiş, PageUp, PageDown) hücre düzeyinde çalışır
/// - Sağ tıklama ve çift tıklama olayları desteklenir.
///
/// Bakınız [`TabloDurumu`] için daha fazla details üzerinde hücre seçim.
///
/// # Örnek
///
/// ```rust,ignore
/// let table_state = cx.new(|cx| {
///     TabloDurumu::new(delegate, cx)
///         .cell_selectable(true)
///         .row_selectable(true)
/// });
///
/// VeriTablosu::new(&table_state)
///     .stripe(true)
///     .bordered(true)
/// ```
#[derive(IntoElement)]
pub struct VeriTablosu<D: TabloTemsilcisi> {
    state: Entity<TabloDurumu<D>>,
    options: TabloSecenekleri,
}

impl<D> VeriTablosu<D>
where
    D: TabloTemsilcisi,
{
    /// Yeni bir VeriTablosu öğe ile verilen [`TabloDurumu`] oluşturur.
    pub fn new(state: &Entity<TabloDurumu<D>>) -> Self {
        Self {
            state: state.clone(),
            options: TabloSecenekleri::default(),
        }
    }

    /// kullanmak için stripe stil tablo, varsayılan için false ayarlar.
    pub fn stripe(mut self, stripe: bool) -> Self {
        self.options.stripe = stripe;
        self
    }

    /// kullanmak için kenarlık stil tablo, varsayılan için true ayarlar.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.options.bordered = bordered;
        self
    }

    /// kaydırma çubuğu görünürlük ayarlar.
    pub fn scrollbar_visible(mut self, vertical: bool, horizontal: bool) -> Self {
        self.options.scrollbar_visible = Edges {
            right: vertical,
            bottom: horizontal,
            ..Default::default()
        };
        self
    }
}

impl<D> Boyutlandirilabilir for VeriTablosu<D>
where
    D: TabloTemsilcisi,
{
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.options.size = size.into();
        self
    }
}

impl<D> RenderOnce for VeriTablosu<D>
where
    D: TabloTemsilcisi,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bordered = self.options.bordered;
        let focus_handle = self.state.focus_handle(cx);
        self.state.update(cx, |state, _| {
            state.options = self.options;
        });

        div()
            .id("table")
            .size_full()
            .key_context(CONTEXT)
            .track_focus(&focus_handle)
            .on_action(window.listener_for(&self.state, TabloDurumu::action_cancel))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_next))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_prev))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_next_col))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_prev_col))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_first_column))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_last_column))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_page_up))
            .on_action(window.listener_for(&self.state, TabloDurumu::action_select_page_down))
            .bg(cx.theme().table)
            .when(bordered, |this| {
                this.rounded(cx.theme().radius)
                    .border_1()
                    .border_color(cx.theme().border)
            })
            .child(self.state)
    }
}
