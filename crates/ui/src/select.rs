use gpui::{
    AnyElement, App, AppContext, Bounds, ClickEvent, Context, DismissEvent, Edges, ElementId,
    Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, KeyBinding,
    Length, ParentElement, Pixels, Render, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Subscription, Task, WeakEntity, Window, anchored, deferred, div,
    prelude::FluentBuilder, px, rems,
};
use rust_i18n::t;

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, EtkinTema, IndexPath,
    OgeUzantisi as _, Secilebilir, Simge, SimgeAdi, StilBoyutlandirma, StilUzantisi,
    actions::{Cancel, Confirm, SelectDown, SelectUp},
    global_state::KureselDurum,
    h_flex,
    input::{clear_button, input_style},
    list::{Liste, ListeDurumu, ListeTemsilcisi},
    v_flex,
};

const CONTEXT: &str = "Secim";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectUp, Some(CONTEXT)),
        KeyBinding::new("down", SelectDown, Some(CONTEXT)),
        KeyBinding::new("enter", Confirm { secondary: false }, Some(CONTEXT)),
        KeyBinding::new(
            "secondary-enter",
            Confirm { secondary: true },
            Some(CONTEXT),
        ),
        KeyBinding::new("escape", Cancel, Some(CONTEXT)),
    ])
}

/// Seçim içinde gösterilebilen öğeler için özellik.
pub trait SecimOgesi: Clone {
    type Value: Clone;
    fn title(&self) -> SharedString;
    /// Customize gösterim başlık için kullanılır seçili öğe içinde Secim girdi.
    ///
    /// None döndürülürse başlık kullanılır.
    fn display_title(&self) -> Option<AnyElement> {
        None
    }
    /// Seçim açılır menüsündeki öğeyi çizer; varsayılan olarak başlığı çizer.
    fn render(&self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.title().into_element()
    }
    /// değer öğe döndürür.
    fn value(&self) -> &Self::Value;
    /// Öğenin arama sorgusuyla eşleşip eşleşmediğini kontrol eder. Varsayılan olarak başlıkla eşleşir.
    fn matches(&self, query: &str) -> bool {
        self.title().to_lowercase().contains(&query.to_lowercase())
    }
}

impl SecimOgesi for String {
    type Value = Self;

    fn title(&self) -> SharedString {
        SharedString::from(self.to_string())
    }

    fn value(&self) -> &Self::Value {
        &self
    }
}

impl SecimOgesi for SharedString {
    type Value = Self;

    fn title(&self) -> SharedString {
        SharedString::from(self.to_string())
    }

    fn value(&self) -> &Self::Value {
        &self
    }
}

impl SecimOgesi for &'static str {
    type Value = Self;

    fn title(&self) -> SharedString {
        SharedString::from(self.to_string())
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

pub trait SecimTemsilcisi: Sized {
    type Item: SecimOgesi;

    /// sayı bölümler içinde [`Secim`] döndürür.
    fn sections_count(&self, _: &App) -> usize {
        1
    }

    /// bölüm başlık öğe için verilen bölüm indeks döndürür.
    fn section(&self, _section: usize) -> Option<AnyElement> {
        return None;
    }

    /// sayı öğeler içinde verilen bölüm döndürür.
    fn items_count(&self, section: usize) -> usize;

    /// Verilen indeks yolundaki öğeyi döndürür; yalnızca bölüm ve satır kullanılır.
    fn item(&self, ix: IndexPath) -> Option<&Self::Item>;

    /// Verilen değere sahip öğenin indeksini döndürür; bulunamazsa None döndürür.
    fn position<V>(&self, _value: &V) -> Option<IndexPath>
    where
        Self::Item: SecimOgesi<Value = V>,
        V: PartialEq;

    fn perform_search(
        &mut self,
        _query: &str,
        _window: &mut Window,
        _: &mut Context<SecimDurumu<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }
}

impl<T: SecimOgesi> SecimTemsilcisi for Vec<T> {
    type Item = T;

    fn items_count(&self, _: usize) -> usize {
        self.len()
    }

    fn item(&self, ix: IndexPath) -> Option<&Self::Item> {
        self.as_slice().get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<IndexPath>
    where
        Self::Item: SecimOgesi<Value = V>,
        V: PartialEq,
    {
        self.iter()
            .position(|v| v.value() == value)
            .map(|ix| IndexPath::default().row(ix))
    }
}

struct SelectListDelegate<D: SecimTemsilcisi + 'static> {
    delegate: D,
    state: WeakEntity<SecimDurumu<D>>,
    selected_index: Option<IndexPath>,
}

impl<D> ListeTemsilcisi for SelectListDelegate<D>
where
    D: SecimTemsilcisi + 'static,
{
    type Item = SelectListItem;

    fn sections_count(&self, cx: &App) -> usize {
        self.delegate.sections_count(cx)
    }

    fn items_count(&self, section: usize, _: &App) -> usize {
        self.delegate.items_count(section)
    }

    fn render_section_header(
        &mut self,
        section: usize,
        _: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> Option<impl IntoElement> {
        let state = self.state.upgrade()?.read(cx);
        let Some(item) = self.delegate.section(section) else {
            return None;
        };

        return Some(
            div()
                .py_0p5()
                .px_2()
                .list_size(state.options.size)
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(item),
        );
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        window: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> Option<Self::Item> {
        let selected = self
            .selected_index
            .map_or(false, |selected_index| selected_index == ix);
        let size = self
            .state
            .upgrade()
            .map_or(BilesenBoyutu::Orta, |state| state.read(cx).options.size);

        if let Some(item) = self.delegate.item(ix) {
            let list_item = SelectListItem::new(ix.row)
                .selected(selected)
                .with_size(size)
                .child(div().whitespace_nowrap().child(item.render(window, cx)));
            Some(list_item)
        } else {
            None
        }
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        let state = self.state.clone();
        let final_selected_index = state
            .read_with(cx, |this, _| this.final_selected_index)
            .ok()
            .flatten();

        // If the selected index is not the final selected index, we need to restore it.
        let need_restore = if final_selected_index != self.selected_index {
            self.selected_index = final_selected_index;
            true
        } else {
            false
        };

        cx.defer_in(window, move |this, window, cx| {
            if need_restore {
                this.set_selected_index(final_selected_index, window, cx);
            }

            _ = state.update(cx, |this, cx| {
                this.set_open(false, cx);
                this.focus(window, cx);
            });
        });
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        let selected_index = self.selected_index;
        let selected_value = selected_index
            .and_then(|ix| self.delegate.item(ix))
            .map(|item| item.value().clone());
        let state = self.state.clone();

        cx.defer_in(window, move |_, window, cx| {
            _ = state.update(cx, |this, cx| {
                cx.emit(SecimOlayi::Confirm(selected_value.clone()));
                this.final_selected_index = selected_index;
                this.selected_value = selected_value;
                this.set_open(false, cx);
                this.focus(window, cx);
            });
        });
    }

    fn perform_search(
        &mut self,
        query: &str,
        window: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> Task<()> {
        self.state.upgrade().map_or(Task::ready(()), |state| {
            state.update(cx, |_, cx| self.delegate.perform_search(query, window, cx))
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _: &mut Window,
        _: &mut Context<ListeDurumu<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn render_empty(
        &mut self,
        window: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> impl IntoElement {
        if let Some(empty) = self
            .state
            .upgrade()
            .and_then(|state| state.read(cx).empty.as_ref())
        {
            empty(window, cx).into_any_element()
        } else {
            h_flex()
                .justify_center()
                .py_6()
                .text_color(cx.theme().muted_foreground.opacity(0.6))
                .child(Simge::new(SimgeAdi::Inbox).size(px(28.)))
                .into_any_element()
        }
    }
}

/// Olaylar yayılan ile [`SecimDurumu`].
pub enum SecimOlayi<D: SecimTemsilcisi + 'static> {
    Confirm(Option<<D::Item as SecimOgesi>::Value>),
}

struct SelectOptions {
    style: StyleRefinement,
    size: BilesenBoyutu,
    icon: Option<Simge>,
    cleanable: bool,
    placeholder: Option<SharedString>,
    title_prefix: Option<SharedString>,
    search_placeholder: Option<SharedString>,
    empty: Option<AnyElement>,
    menu_width: Length,
    menu_max_h: Length,
    disabled: bool,
    appearance: bool,
}

impl Default for SelectOptions {
    fn default() -> Self {
        Self {
            style: StyleRefinement::default(),
            size: BilesenBoyutu::default(),
            icon: None,
            cleanable: false,
            placeholder: None,
            title_prefix: None,
            empty: None,
            menu_width: Length::Auto,
            menu_max_h: rems(20.).into(),
            disabled: false,
            appearance: true,
            search_placeholder: None,
        }
    }
}

/// durum [`Secim`].
pub struct SecimDurumu<D: SecimTemsilcisi + 'static> {
    focus_handle: FocusHandle,
    options: SelectOptions,
    searchable: bool,
    list: Entity<ListeDurumu<SelectListDelegate<D>>>,
    empty: Option<Box<dyn Fn(&Window, &App) -> AnyElement>>,
    /// Store sınırlar girdi
    bounds: Bounds<Pixels>,
    open: bool,
    selected_value: Option<<D::Item as SecimOgesi>::Value>,
    final_selected_index: Option<IndexPath>,
    _subscriptions: Vec<Subscription>,
}

/// Bir Secim öğe.
#[derive(IntoElement)]
pub struct Secim<D: SecimTemsilcisi + 'static> {
    id: ElementId,
    state: Entity<SecimDurumu<D>>,
    options: SelectOptions,
}

/// Seçim öğeleri için yerleşik aranabilir vektör.
#[derive(Debug, Clone)]
pub struct SearchableVec<T> {
    items: Vec<T>,
    matched_items: Vec<T>,
}

impl<T: Clone> SearchableVec<T> {
    pub fn push(&mut self, item: T) {
        self.items.push(item.clone());
        self.matched_items.push(item);
    }
}

impl<T: Clone> SearchableVec<T> {
    pub fn new(items: impl Into<Vec<T>>) -> Self {
        let items = items.into();
        Self {
            items: items.clone(),
            matched_items: items,
        }
    }
}

impl<T: SecimOgesi> From<Vec<T>> for SearchableVec<T> {
    fn from(items: Vec<T>) -> Self {
        Self {
            items: items.clone(),
            matched_items: items,
        }
    }
}

impl<I: SecimOgesi> SecimTemsilcisi for SearchableVec<I> {
    type Item = I;

    fn items_count(&self, _: usize) -> usize {
        self.matched_items.len()
    }

    fn item(&self, ix: IndexPath) -> Option<&Self::Item> {
        self.matched_items.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<IndexPath>
    where
        Self::Item: SecimOgesi<Value = V>,
        V: PartialEq,
    {
        for (ix, item) in self.matched_items.iter().enumerate() {
            if item.value() == value {
                return Some(IndexPath::default().row(ix));
            }
        }

        None
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _: &mut Context<SecimDurumu<Self>>,
    ) -> Task<()> {
        self.matched_items = self
            .items
            .iter()
            .filter(|item| item.matches(query))
            .cloned()
            .collect();

        Task::ready(())
    }
}

impl<I: SecimOgesi> SecimTemsilcisi for SearchableVec<SecimGrubu<I>> {
    type Item = I;

    fn sections_count(&self, _: &App) -> usize {
        self.matched_items.len()
    }

    fn items_count(&self, section: usize) -> usize {
        self.matched_items
            .get(section)
            .map_or(0, |group| group.items.len())
    }

    fn section(&self, section: usize) -> Option<AnyElement> {
        Some(
            self.matched_items
                .get(section)?
                .title
                .clone()
                .into_any_element(),
        )
    }

    fn item(&self, ix: IndexPath) -> Option<&Self::Item> {
        let section = self.matched_items.get(ix.section)?;

        section.items.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<IndexPath>
    where
        Self::Item: SecimOgesi<Value = V>,
        V: PartialEq,
    {
        for (ix, group) in self.matched_items.iter().enumerate() {
            for (row_ix, item) in group.items.iter().enumerate() {
                if item.value() == value {
                    return Some(IndexPath::default().section(ix).row(row_ix));
                }
            }
        }

        None
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _: &mut Context<SecimDurumu<Self>>,
    ) -> Task<()> {
        self.matched_items = self
            .items
            .iter()
            .filter(|item| item.matches(&query))
            .cloned()
            .map(|mut item| {
                item.items.retain(|item| item.matches(&query));
                item
            })
            .collect();

        Task::ready(())
    }
}

/// Bir grup seçim öğeler ile bir başlık.
#[derive(Debug, Clone)]
pub struct SecimGrubu<I: SecimOgesi> {
    pub title: SharedString,
    pub items: Vec<I>,
}

impl<I> SecimGrubu<I>
where
    I: SecimOgesi,
{
    /// Yeni bir SecimGrubu ile verilen başlık oluşturur.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            items: vec![],
        }
    }

    /// bir öğe için grup. ekler.
    pub fn item(mut self, item: I) -> Self {
        self.items.push(item);
        self
    }

    /// Birden çok öğeler için grup ekler.
    pub fn items(mut self, items: impl IntoIterator<Item = I>) -> Self {
        self.items.extend(items);
        self
    }

    fn matches(&self, query: &str) -> bool {
        self.title.to_lowercase().contains(&query.to_lowercase())
            || self.items.iter().any(|item| item.matches(query))
    }
}

impl<D> SecimDurumu<D>
where
    D: SecimTemsilcisi + 'static,
{
    /// Yeni bir Secim durum oluşturur.
    pub fn new(
        delegate: D,
        selected_index: Option<IndexPath>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let delegate = SelectListDelegate {
            delegate,
            state: cx.entity().downgrade(),
            selected_index,
        };

        let list = cx.new(|cx| ListeDurumu::new(delegate, window, cx).reset_on_cancel(false));
        let list_focus_handle = list.read(cx).focus_handle.clone();
        let list_search_focus_handle = list.read(cx).query_input.focus_handle(cx);

        let _subscriptions = vec![
            cx.on_blur(&list_focus_handle, window, Self::on_blur),
            cx.on_blur(&list_search_focus_handle, window, Self::on_blur),
            cx.on_blur(&focus_handle, window, Self::on_blur),
        ];

        let mut this = Self {
            focus_handle,
            options: SelectOptions::default(),
            searchable: false,
            list,
            selected_value: None,
            open: false,
            bounds: Bounds::default(),
            empty: None,
            final_selected_index: None,
            _subscriptions,
        };
        this.set_selected_index(selected_index, window, cx);
        this
    }

    /// Açılır menünün aranabilir olup olmadığını ayarlar. Varsayılan `false`.
    ///
    /// `true` olduğunda açılır menünün üstünde bir arama girdisi gösterilir.
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// seçili indeks için seçim ayarlar.
    pub fn set_selected_index(
        &mut self,
        selected_index: Option<IndexPath>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.list.update(cx, |list, cx| {
            list._set_selected_index(selected_index, window, cx);
        });
        self.final_selected_index = selected_index;
        self.update_selected_value(window, cx);
    }

    /// seçili değer için seçim ayarlar.
    ///
    /// Bu yöntem konumu temsilciden alır ve seçili indeksi ayarlar.
    ///
    /// Değer bulunamazsa None ayarlanır.
    pub fn set_selected_value(
        &mut self,
        selected_value: &<D::Item as SecimOgesi>::Value,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        <<D as SecimTemsilcisi>::Item as SecimOgesi>::Value: PartialEq,
    {
        let delegate = self.list.read(cx).delegate();
        let selected_index = delegate.delegate.position(selected_value);
        self.set_selected_index(selected_index, window, cx);
    }

    /// öğeler için seçim durum ayarlar.
    pub fn set_items(&mut self, items: D, _: &mut Window, cx: &mut Context<Self>)
    where
        D: SecimTemsilcisi + 'static,
    {
        self.list.update(cx, |list, _| {
            list.delegate_mut().delegate = items;
        });
    }

    /// seçili indeks seçim döndürür.
    pub fn selected_index(&self, cx: &App) -> Option<IndexPath> {
        self.list.read(cx).selected_index()
    }

    /// seçili değer seçim döndürür.
    pub fn selected_value(&self) -> Option<&<D::Item as SecimOgesi>::Value> {
        self.selected_value.as_ref()
    }

    /// Seçim girdisine odaklanır.
    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.focus_handle.focus(window, cx);
    }

    fn update_selected_value(&mut self, _: &Window, cx: &App) {
        self.selected_value = self
            .selected_index(cx)
            .and_then(|ix| self.list.read(cx).delegate().delegate.item(ix))
            .map(|item| item.value().clone());
    }

    fn on_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // When the select and dropdown menu are both not focused, close the dropdown menu.
        if self.list.read(cx).is_focused(window, cx) || self.focus_handle.is_focused(window) {
            return;
        }

        // If the selected index is not the final selected index, we need to restore it.
        let final_selected_index = self.final_selected_index;
        let selected_index = self.selected_index(cx);
        if final_selected_index != selected_index {
            self.list.update(cx, |list, cx| {
                list.set_selected_index(self.final_selected_index, window, cx);
            });
        }

        self.set_open(false, cx);
        cx.notify();
    }

    fn up(&mut self, _: &SelectUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            self.set_open(true, cx);
        }

        self.list.focus_handle(cx).focus(window, cx);
        cx.propagate();
    }

    fn down(&mut self, _: &SelectDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            self.set_open(true, cx);
        }

        self.list.focus_handle(cx).focus(window, cx);
        cx.propagate();
    }

    fn enter(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        // Propagate the event to the parent view, for example to the IletisimKutusu to support ENTER to confirm.
        cx.propagate();

        if !self.open {
            self.set_open(true, cx);
            cx.notify();
        }

        self.list.focus_handle(cx).focus(window, cx);
    }

    fn toggle_menu(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        cx.stop_propagation();

        self.set_open(!self.open, cx);
        if self.open {
            self.list.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }

    fn escape(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            cx.propagate();
            return;
        }

        cx.stop_propagation();
        self.set_open(false, cx);
        self.focus(window, cx);
        cx.notify();
    }

    fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.open = open;
        if self.open {
            KureselDurum::global_mut(cx).register_deferred_popover(&self.focus_handle)
        } else {
            KureselDurum::global_mut(cx).unregister_deferred_popover(&self.focus_handle)
        }
        cx.notify();
    }

    fn clean(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        cx.stop_propagation();
        self.set_selected_index(None, window, cx);
        cx.emit(SecimOlayi::Confirm(None));
    }

    /// başlık öğe için seçim girdi döndürür.
    fn display_title(&mut self, _: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let default_title = div().text_color(cx.theme().muted_foreground).child(
            self.options
                .placeholder
                .clone()
                .unwrap_or_else(|| t!("Secim.placeholder").into()),
        );

        let Some(selected_index) = &self.selected_index(cx) else {
            return default_title;
        };

        let Some(title) = self
            .list
            .read(cx)
            .delegate()
            .delegate
            .item(*selected_index)
            .map(|item| {
                if let Some(el) = item.display_title() {
                    el
                } else {
                    if let Some(prefix) = self.options.title_prefix.as_ref() {
                        format!("{}{}", prefix, item.title()).into_any_element()
                    } else {
                        item.title().into_any_element()
                    }
                }
            })
        else {
            return default_title;
        };

        div()
            .when(self.options.disabled, |this| {
                this.text_color(cx.theme().muted_foreground)
            })
            .child(title)
    }
}

impl<D> Render for SecimDurumu<D>
where
    D: SecimTemsilcisi + 'static,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let searchable = self.searchable;
        let is_focused = self.focus_handle.is_focused(window);
        let show_clean = self.options.cleanable && self.selected_index(cx).is_some();
        let bounds = self.bounds;
        let allow_open = !(self.open || self.options.disabled);
        let outline_visible = self.open || is_focused && !self.options.disabled;
        let popup_radius = cx.theme().radius.min(px(8.));

        let (bg, fg) = input_style(self.options.disabled, cx);

        self.list
            .update(cx, |list, cx| list.set_searchable(searchable, cx));

        div()
            .size_full()
            .relative()
            .child(
                div()
                    .id("input")
                    .relative()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_1()
                    .border_color(cx.theme().transparent)
                    .when(self.options.appearance, |this| {
                        this.bg(bg)
                            .text_color(fg)
                            .when(self.options.disabled, |this| this.opacity(0.5))
                            .border_color(cx.theme().input)
                            .rounded(cx.theme().radius)
                            .when(cx.theme().shadow, |this| this.shadow_xs())
                    })
                    .map(|this| {
                        if self.options.disabled {
                            this.shadow_none()
                        } else {
                            this
                        }
                    })
                    .overflow_hidden()
                    .input_size(self.options.size)
                    .input_text_size(self.options.size)
                    .refine_style(&self.options.style)
                    .when(outline_visible, |this| this.focused_border(cx))
                    .when(allow_open, |this| {
                        this.on_click(cx.listener(Self::toggle_menu))
                    })
                    .child(
                        h_flex()
                            .id("inner")
                            .w_full()
                            .items_center()
                            .justify_between()
                            .gap_1()
                            .child(
                                div()
                                    .id("title")
                                    .w_full()
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .truncate()
                                    .child(self.display_title(window, cx)),
                            )
                            .when(show_clean, |this| {
                                this.child(clear_button(cx).map(|this| {
                                    if self.options.disabled {
                                        this.disabled(true)
                                    } else {
                                        this.on_click(cx.listener(Self::clean))
                                    }
                                }))
                            })
                            .when(!show_clean, |this| {
                                let icon = match self.options.icon.clone() {
                                    Some(icon) => icon,
                                    None => Simge::new(SimgeAdi::ChevronDown),
                                };

                                this.child(icon.xsmall().text_color(cx.theme().muted_foreground))
                            }),
                    )
                    .on_prepaint({
                        let state = cx.entity();
                        move |bounds, _, cx| state.update(cx, |r, _| r.bounds = bounds)
                    }),
            )
            .when(self.open, |this| {
                this.child(
                    deferred(
                        anchored().snap_to_window_with_margin(px(8.)).child(
                            div()
                                .occlude()
                                .map(|this| match self.options.menu_width {
                                    Length::Auto => this.w(bounds.size.width + px(2.)),
                                    Length::Definite(w) => this.w(w),
                                })
                                .child(
                                    v_flex()
                                        .occlude()
                                        .mt_1p5()
                                        .bg(cx.theme().background)
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded(popup_radius)
                                        .shadow_md()
                                        .child(
                                            Liste::new(&self.list)
                                                .when_some(
                                                    self.options.search_placeholder.clone(),
                                                    |this, placeholder| {
                                                        this.search_placeholder(placeholder)
                                                    },
                                                )
                                                .with_size(self.options.size)
                                                .max_h(self.options.menu_max_h)
                                                .paddings(Edges::all(px(4.))),
                                        ),
                                )
                                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                                    this.escape(&Cancel, window, cx);
                                })),
                        ),
                    )
                    .with_priority(1),
                )
            })
    }
}

impl<D> Secim<D>
where
    D: SecimTemsilcisi + 'static,
{
    pub fn new(state: &Entity<SecimDurumu<D>>) -> Self {
        Self {
            id: ("select", state.entity_id()).into(),
            state: state.clone(),
            options: SelectOptions::default(),
        }
    }

    /// genişlik açılır menü ayarlar. Varsayılan: Length::Auto
    pub fn menu_width(mut self, width: impl Into<Length>) -> Self {
        self.options.menu_width = width.into();
        self
    }

    /// maksimum yükseklik açılır menü ayarlar. Varsayılan: 20rem
    pub fn menu_max_h(mut self, max_h: impl Into<Length>) -> Self {
        self.options.menu_max_h = max_h.into();
        self
    }

    /// Seçim değeri boşken gösterilecek yer tutucuyu ayarlar.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.options.placeholder = Some(placeholder.into());
        self
    }

    /// Seçim girdisinin sağ simgesini ayarlar; varsayılan ok simgesinin yerine kullanılır.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.options.icon = Some(icon.into());
        self
    }

    /// başlık ön ek için seçim ayarlar.
    ///
    /// e.g.: Country: United States
    ///
    /// Etiketi `Country: ` olarak ayarlamalısınız.
    pub fn title_prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.options.title_prefix = Some(prefix.into());
        self
    }

    /// Girdi alanı boş değilken temizleme düğmesinin gösterilip gösterilmeyeceğini ayarlar. Varsayılan false.
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.options.cleanable = cleanable;
        self
    }

    /// Arama girdisinin yer tutucu metnini ayarlar.
    pub fn search_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.options.search_placeholder = Some(placeholder.into());
        self
    }

    /// devre dışı bırakır durum için seçim ayarlar.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.options.disabled = disabled;
        self
    }

    /// Seçim listesi boşken gösterilecek öğeyi ayarlar.
    pub fn empty(mut self, el: impl IntoElement) -> Self {
        self.options.empty = Some(el.into_any_element());
        self
    }

    /// Seçim görünümünü ayarlar; false ise seçim girdisinde kenarlık ve arka plan olmaz.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.options.appearance = appearance;
        self
    }
}

impl<D> Boyutlandirilabilir for Secim<D>
where
    D: SecimTemsilcisi + 'static,
{
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.options.size = size.into();
        self
    }
}

impl<D> EventEmitter<SecimOlayi<D>> for SecimDurumu<D> where D: SecimTemsilcisi + 'static {}
impl<D> EventEmitter<DismissEvent> for SecimDurumu<D> where D: SecimTemsilcisi + 'static {}
impl<D> Focusable for SecimDurumu<D>
where
    D: SecimTemsilcisi,
{
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.open {
            self.list.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl<D> Styled for Secim<D>
where
    D: SecimTemsilcisi,
{
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.options.style
    }
}

impl<D> RenderOnce for Secim<D>
where
    D: SecimTemsilcisi + 'static,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.options.disabled;
        let focus_handle = self.state.focus_handle(cx);
        // If the size has change, set size to self.list, to change the QueryInput size.
        self.state.update(cx, |this, _| {
            this.options = self.options;
        });

        div()
            .id(self.id.clone())
            .key_context(CONTEXT)
            .when(!disabled, |this| {
                this.track_focus(&focus_handle.tab_stop(true))
            })
            .on_action(window.listener_for(&self.state, SecimDurumu::up))
            .on_action(window.listener_for(&self.state, SecimDurumu::down))
            .on_action(window.listener_for(&self.state, SecimDurumu::enter))
            .on_action(window.listener_for(&self.state, SecimDurumu::escape))
            .size_full()
            .child(self.state)
    }
}

#[derive(IntoElement)]
struct SelectListItem {
    id: ElementId,
    size: BilesenBoyutu,
    style: StyleRefinement,
    selected: bool,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl SelectListItem {
    pub fn new(ix: usize) -> Self {
        Self {
            id: ("select-item", ix).into(),
            size: BilesenBoyutu::default(),
            style: StyleRefinement::default(),
            selected: false,
            disabled: false,
            children: Vec::new(),
        }
    }
}

impl ParentElement for SelectListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl DevreDisiBirakilabilir for SelectListItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Secilebilir for SelectListItem {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Boyutlandirilabilir for SelectListItem {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for SelectListItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectListItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .relative()
            .gap_x_1()
            .py_1()
            .px_2()
            .rounded(cx.theme().radius)
            .text_base()
            .text_color(cx.theme().foreground)
            .relative()
            .items_center()
            .justify_between()
            .input_text_size(self.size)
            .list_size(self.size)
            .refine_style(&self.style)
            .when(!self.disabled, |this| {
                this.when(!self.selected, |this| {
                    this.hover(|this| this.bg(cx.theme().accent.alpha(0.7)))
                })
            })
            .when(self.selected, |this| this.bg(cx.theme().accent))
            .when(self.disabled, |this| {
                this.text_color(cx.theme().muted_foreground)
            })
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .gap_x_1()
                    .child(div().w_full().children(self.children)),
            )
    }
}
