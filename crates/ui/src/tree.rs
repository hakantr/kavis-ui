use std::{cell::RefCell, ops::Range, rc::Rc};

use gpui::{
    App, Context, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement, KeyBinding,
    ListSizingBehavior, MouseButton, ParentElement, Render, RenderOnce, SharedString,
    StyleRefinement, Styled, UniformListScrollHandle, Window, div, prelude::FluentBuilder as _,
    uniform_list,
};

use crate::{
    Secilebilir as _, StilUzantisi,
    actions::{Confirm, SelectDown, SelectLeft, SelectRight, SelectUp},
    list::ListeOgesi,
    menu::{AcilirMenu, BaglamMenusuUzantisi as _},
    scroll::KaydirilabilirOge,
};

const CONTEXT: &str = "Agac";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", SelectUp, Some(CONTEXT)),
        KeyBinding::new("down", SelectDown, Some(CONTEXT)),
        KeyBinding::new("left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("right", SelectRight, Some(CONTEXT)),
    ]);
}

/// Bir [`Agac`]. oluşturur.
///
/// # Argümanlar
///
/// * `durum` - paylaşılan durum managing ağaç öğeler.
/// * `render_item` - Her ağaç öğesini çizmek için kullanılan kapanış.
///
/// ```ignore
/// let state = cx.new(|_| {
///     AgacDurumu::new().items(vec![
///         AgacOgesi::new("src")
///             .child(AgacOgesi::new("lib.rs"),
///         AgacOgesi::new("Cargo.toml"),
///         AgacOgesi::new("README.md"),
///     ])
/// });
///
/// tree(&state, |ix, entry, selected, window, cx| {
///     let item = entry.item();
///     ListeOgesi::new(ix).pl(px(16.) * entry.depth()).child(item.label.clone())
/// })
/// ```
pub fn tree<R>(state: &Entity<AgacDurumu>, render_item: R) -> Agac
where
    R: Fn(usize, &AgacGirdisi, bool, &mut Window, &mut App) -> ListeOgesi + 'static,
{
    Agac::new(state, render_item)
}

struct TreeItemState {
    expanded: bool,
    disabled: bool,
}

/// Bir ağaç öğe ile bir etiket, alt öğeler, ve bir genişletilmiş durum.
#[derive(Clone)]
pub struct AgacOgesi {
    pub id: SharedString,
    pub label: SharedString,
    pub children: Vec<AgacOgesi>,
    state: Rc<RefCell<TreeItemState>>,
}

/// Bir flat representation bir ağaç öğe ile onun depth.
#[derive(Clone)]
pub struct AgacGirdisi {
    item: AgacOgesi,
    depth: usize,
}

impl AgacGirdisi {
    /// kaynak ağaç öğe döndürür.
    #[inline]
    pub fn item(&self) -> &AgacOgesi {
        &self.item
    }

    /// depth bu öğe içinde ağaç.
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    fn is_root(&self) -> bool {
        self.depth == 0
    }

    /// Bu öğenin alt öğelere sahip bir klasör olup olmadığını belirtir.
    #[inline]
    pub fn is_folder(&self) -> bool {
        self.item.is_folder()
    }

    /// Öğe genişletilmişse true döndürür.
    #[inline]
    pub fn is_expanded(&self) -> bool {
        self.item.is_expanded()
    }

    #[inline]
    pub fn is_disabled(&self) -> bool {
        self.item.is_disabled()
    }
}

impl AgacOgesi {
    /// Yeni bir ağaç öğe ile verilen etiket oluşturur.
    ///
    /// - `id`, bu öğeyi benzersiz tanımlar; daha sonra seçim veya başka amaçlar için kullanılabilir.
    /// - `etiket`, bu öğe için gösterilecek metindir.
    ///
    /// Örneğin `id` tam dosya yolu, `etiket` dosya adı olabilir.
    ///
    /// ```ignore
    /// AgacOgesi::new("src/ui/button.rs", "button.rs")
    /// ```
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            state: Rc::new(RefCell::new(TreeItemState {
                expanded: false,
                disabled: false,
            })),
        }
    }

    /// Bir alt öğe için bu ağaç öğe ekler.
    pub fn child(mut self, child: AgacOgesi) -> Self {
        self.children.push(child);
        self
    }

    /// Birden çok alt öğeler için bu ağaç öğe ekler.
    pub fn children(mut self, children: impl IntoIterator<Item = AgacOgesi>) -> Self {
        self.children.extend(children);
        self
    }

    /// genişletilmiş durum için bu ağaç öğe ayarlar.
    pub fn expanded(self, expanded: bool) -> Self {
        self.state.borrow_mut().expanded = expanded;
        self
    }

    /// devre dışı durum için bu ağaç öğe ayarlar.
    pub fn disabled(self, disabled: bool) -> Self {
        self.state.borrow_mut().disabled = disabled;
        self
    }

    /// Bu öğenin alt öğelere sahip bir klasör olup olmadığını belirtir.
    #[inline]
    pub fn is_folder(&self) -> bool {
        self.children.len() > 0
    }

    /// Öğe devre dışıysa true döndürür.
    pub fn is_disabled(&self) -> bool {
        self.state.borrow().disabled
    }

    /// Öğe genişletilmişse true döndürür.
    #[inline]
    pub fn is_expanded(&self) -> bool {
        self.state.borrow().expanded
    }

    fn find_ancestors(&self, target_id: &SharedString) -> Option<Vec<AgacOgesi>> {
        if self.id == *target_id {
            return Some(vec![]);
        }

        for child in &self.children {
            if let Some(mut path) = child.find_ancestors(target_id) {
                path.push(self.clone());
                return Some(path);
            }
        }

        None
    }
}

/// durum için managing ağaç öğeler.
pub struct AgacDurumu {
    focus_handle: FocusHandle,
    entries: Vec<AgacGirdisi>,
    scroll_handle: UniformListScrollHandle,
    selected_ix: Option<usize>,
    right_clicked_ix: Option<usize>,
    render_item: Rc<dyn Fn(usize, &AgacGirdisi, bool, &mut Window, &mut App) -> ListeOgesi>,
    context_menu_builder: Option<
        Rc<
            dyn Fn(
                usize,
                &AgacGirdisi,
                AcilirMenu,
                &mut Window,
                &mut Context<AgacDurumu>,
            ) -> AcilirMenu,
        >,
    >,
}

impl AgacDurumu {
    /// Yeni bir boş ağaç durum oluşturur.
    pub fn new(cx: &mut App) -> Self {
        Self {
            selected_ix: None,
            right_clicked_ix: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::default(),
            entries: Vec::new(),
            render_item: Rc::new(|_, _, _, _, _| ListeOgesi::new(0)),
            context_menu_builder: None,
        }
    }

    /// ağaç öğeler ayarlar.
    pub fn items(mut self, items: impl Into<Vec<AgacOgesi>>) -> Self {
        let items = items.into();
        self.entries.clear();
        for item in items.into_iter() {
            self.add_entry(item, 0);
        }
        self
    }

    /// ağaç öğeler ayarlar.
    pub fn set_items(&mut self, items: impl Into<Vec<AgacOgesi>>, cx: &mut Context<Self>) {
        let items = items.into();
        self.entries.clear();
        for item in items.into_iter() {
            self.add_entry(item, 0);
        }
        self.selected_ix = None;
        self.right_clicked_ix = None;
        cx.notify();
    }

    /// şu anda seçili indeks, ise herhangi bir döndürür.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_ix
    }

    /// Seçili indeksi ayarlar; seçimi temizlemek için `None` kullanılır.
    pub fn set_selected_index(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_ix = ix;
        cx.notify();
    }

    /// Ağaç öğesiyle seçili indeksi ayarlar; seçimi temizlemek için `None` kullanılır.
    pub fn set_selected_item(&mut self, item: Option<&AgacOgesi>, cx: &mut Context<Self>) {
        if let Some(item) = item {
            let ix = self
                .entries
                .iter()
                .position(|entry| entry.item.id == item.id);
            if ix.is_some() {
                self.selected_ix = ix;
            } else {
                self.expand_ancestors(item.id.clone());
                self.selected_ix = self
                    .entries
                    .iter()
                    .position(|entry| entry.item.id == item.id);
            }
        } else {
            self.selected_ix = None;
        }
        cx.notify();
    }

    /// şu anda seçili ağaç öğe, ise herhangi bir döndürür.
    pub fn selected_item(&self) -> Option<&AgacOgesi> {
        self.selected_ix
            .and_then(|ix| self.entries.get(ix).map(|entry| &entry.item))
    }

    pub fn scroll_to_item(&mut self, ix: usize, strategy: gpui::ScrollStrategy) {
        self.scroll_handle.scroll_to_item(ix, strategy);
    }

    /// şu anda seçili entry, ise herhangi bir döndürür.
    pub fn selected_entry(&self) -> Option<&AgacGirdisi> {
        self.selected_ix.and_then(|ix| self.entries.get(ix))
    }

    fn expand_ancestors(&mut self, target_id: SharedString) {
        let mut ancestors = Vec::new();

        for entry in &self.entries {
            if let Some(found_ancestors) = entry.item.find_ancestors(&target_id) {
                ancestors = found_ancestors;
                break;
            }
        }

        if ancestors.is_empty() {
            return;
        }

        for ancestor in ancestors {
            ancestor.state.borrow_mut().expanded = true;
        }

        self.rebuild_entries();
    }

    fn add_entry(&mut self, item: AgacOgesi, depth: usize) {
        self.entries.push(AgacGirdisi {
            item: item.clone(),
            depth,
        });
        if item.is_expanded() {
            for child in &item.children {
                self.add_entry(child.clone(), depth + 1);
            }
        }
    }

    fn toggle_expand(&mut self, ix: usize) {
        let Some(entry) = self.entries.get_mut(ix) else {
            return;
        };
        if !entry.is_folder() {
            return;
        }

        entry.item.state.borrow_mut().expanded = !entry.is_expanded();
        self.right_clicked_ix = None;
        self.rebuild_entries();
    }

    fn rebuild_entries(&mut self) {
        let root_items: Vec<AgacOgesi> = self
            .entries
            .iter()
            .filter(|e| e.is_root())
            .map(|e| e.item.clone())
            .collect();
        self.entries.clear();
        for item in root_items.into_iter() {
            self.add_entry(item, 0);
        }
    }

    pub fn focus(&mut self, window: &mut Window, cx: &mut App) {
        self.focus_handle.focus(window, cx);
    }

    fn on_action_confirm(&mut self, _: &Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_ix) = self.selected_ix {
            if let Some(entry) = self.entries.get(selected_ix) {
                if entry.is_folder() {
                    self.toggle_expand(selected_ix);
                    cx.notify();
                }
            }
        }
    }

    fn on_action_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_ix) = self.selected_ix {
            if let Some(entry) = self.entries.get(selected_ix) {
                if entry.is_folder() && entry.is_expanded() {
                    self.toggle_expand(selected_ix);
                    cx.notify();
                }
            }
        }
    }

    fn on_action_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_ix) = self.selected_ix {
            if let Some(entry) = self.entries.get(selected_ix) {
                if entry.is_folder() && !entry.is_expanded() {
                    self.toggle_expand(selected_ix);
                    cx.notify();
                }
            }
        }
    }

    fn on_action_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        let mut selected_ix = self.selected_ix.unwrap_or(0);

        if selected_ix > 0 {
            selected_ix = selected_ix - 1;
        } else {
            selected_ix = self.entries.len().saturating_sub(1);
        }

        self.selected_ix = Some(selected_ix);
        self.scroll_handle
            .scroll_to_item(selected_ix, gpui::ScrollStrategy::Top);
        cx.notify();
    }

    fn on_action_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        let mut selected_ix = self.selected_ix.unwrap_or(0);
        if selected_ix + 1 < self.entries.len() {
            selected_ix = selected_ix + 1;
        } else {
            selected_ix = 0;
        }

        self.selected_ix = Some(selected_ix);
        self.scroll_handle
            .scroll_to_item(selected_ix, gpui::ScrollStrategy::Bottom);
        cx.notify();
    }

    fn on_entry_click(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_ix = Some(ix);
        self.toggle_expand(ix);
        cx.notify();
    }
}

impl Render for AgacDurumu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let render_item = self.render_item.clone();
        let state = cx.entity().clone();

        div()
            .id("tree-state")
            .size_full()
            .relative()
            .baglam_menusu({
                let state = state.clone();
                move |menu, window, cx: &mut Context<AcilirMenu>| {
                    if state.read(cx).context_menu_builder.is_none() {
                        return menu;
                    }

                    let (ix, entry) = {
                        let state = state.read(cx);
                        let entry = state
                            .right_clicked_ix
                            .and_then(|ix| state.entries.get(ix).cloned());
                        (state.right_clicked_ix, entry)
                    };

                    if let (Some(ix), Some(entry)) = (ix, entry) {
                        state.update(cx, |state, cx| {
                            if let Some(build) = state.context_menu_builder.clone() {
                                build(ix, &entry, menu, window, cx)
                            } else {
                                menu
                            }
                        })
                    } else {
                        menu
                    }
                }
            })
            .child(
                uniform_list("entries", self.entries.len(), {
                    cx.processor(move |state, visible_range: Range<usize>, window, cx| {
                        let mut items = Vec::with_capacity(visible_range.len());
                        for ix in visible_range {
                            let entry = &state.entries[ix];
                            let selected = Some(ix) == state.selected_ix;
                            let right_clicked = Some(ix) == state.right_clicked_ix;
                            let item = (render_item)(ix, entry, selected, window, cx);

                            let el = div()
                                .id(ix)
                                .child(
                                    item.disabled(entry.item().is_disabled())
                                        .selected(selected)
                                        .secondary_selected(right_clicked),
                                )
                                .when(!entry.item().is_disabled(), |this| {
                                    this.on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener({
                                            move |this, _, window, cx| {
                                                this.on_entry_click(ix, window, cx);
                                            }
                                        }),
                                    )
                                    .on_mouse_down(
                                        MouseButton::Right,
                                        cx.listener(move |this, _, _, cx| {
                                            this.right_clicked_ix = Some(ix);
                                            cx.notify();
                                        }),
                                    )
                                });

                            items.push(el)
                        }

                        items
                    })
                })
                .flex_grow()
                .size_full()
                .track_scroll(&self.scroll_handle)
                .with_sizing_behavior(ListSizingBehavior::Auto)
                .into_any_element(),
            )
    }
}

/// Hiyerarşik veri gösteren ağaç görünümü öğesi.
#[derive(IntoElement)]
pub struct Agac {
    id: ElementId,
    state: Entity<AgacDurumu>,
    style: StyleRefinement,
    render_item: Rc<dyn Fn(usize, &AgacGirdisi, bool, &mut Window, &mut App) -> ListeOgesi>,
    context_menu_builder: Option<
        Rc<
            dyn Fn(
                usize,
                &AgacGirdisi,
                AcilirMenu,
                &mut Window,
                &mut Context<AgacDurumu>,
            ) -> AcilirMenu,
        >,
    >,
}

impl Agac {
    pub fn new<R>(state: &Entity<AgacDurumu>, render_item: R) -> Self
    where
        R: Fn(usize, &AgacGirdisi, bool, &mut Window, &mut App) -> ListeOgesi + 'static,
    {
        Self {
            id: ElementId::Name(format!("tree-{}", state.entity_id()).into()),
            state: state.clone(),
            style: StyleRefinement::default(),
            render_item: Rc::new(move |ix, item, selected, window, app| {
                render_item(ix, item, selected, window, app)
            }),
            context_menu_builder: None,
        }
    }

    /// Bir bağlam menü için ağaç ekler.
    ///
    /// kapanış receives:
    /// - `ix`: indeks sağ-tıklandığında entry
    /// - `entry`: sağ-tıklandığında ağaç entry
    /// - `menu`: açılır pencere menü oluşturucu
    pub fn baglam_menusu<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, &AgacGirdisi, AcilirMenu, &mut Window, &mut Context<AgacDurumu>) -> AcilirMenu
            + 'static,
    {
        self.context_menu_builder = Some(Rc::new(f));
        self
    }
}

impl Styled for Agac {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Agac {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.state.read(cx).focus_handle.clone();
        let scroll_handle = self.state.read(cx).scroll_handle.clone();

        self.state.update(cx, |state, _| {
            state.render_item = self.render_item;
            state.context_menu_builder = self.context_menu_builder;
        });

        div()
            .id(self.id)
            .key_context(CONTEXT)
            .track_focus(&focus_handle)
            .on_action(window.listener_for(&self.state, AgacDurumu::on_action_confirm))
            .on_action(window.listener_for(&self.state, AgacDurumu::on_action_left))
            .on_action(window.listener_for(&self.state, AgacDurumu::on_action_right))
            .on_action(window.listener_for(&self.state, AgacDurumu::on_action_up))
            .on_action(window.listener_for(&self.state, AgacDurumu::on_action_down))
            .size_full()
            .child(self.state)
            .refine_style(&self.style)
            .vertical_scrollbar(&scroll_handle)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::AgacDurumu;
    use gpui::AppContext as _;

    fn assert_entries(entries: &Vec<super::AgacGirdisi>, expected: &str) {
        let actual: Vec<String> = entries
            .iter()
            .map(|e| {
                let mut s = String::new();
                s.push_str(&"    ".repeat(e.depth));
                s.push_str(e.item().label.as_str());
                s
            })
            .collect();
        let actual = actual.join("\n");
        assert_eq!(actual.trim(), expected.trim());
    }

    #[gpui::test]
    fn test_tree_entry(cx: &mut gpui::TestAppContext) {
        use super::AgacOgesi;

        let items = vec![
            AgacOgesi::new("src", "src")
                .expanded(true)
                .child(
                    AgacOgesi::new("src/ui", "ui")
                        .expanded(true)
                        .child(AgacOgesi::new("src/ui/button.rs", "button.rs"))
                        .child(AgacOgesi::new("src/ui/icon.rs", "icon.rs"))
                        .child(AgacOgesi::new("src/ui/mod.rs", "mod.rs")),
                )
                .child(AgacOgesi::new("src/lib.rs", "lib.rs")),
            AgacOgesi::new("Cargo.toml", "Cargo.toml"),
            AgacOgesi::new("Cargo.lock", "Cargo.lock").disabled(true),
            AgacOgesi::new("README.md", "README.md"),
        ];

        let state = cx.new(|cx| AgacDurumu::new(cx).items(items));
        state.update(cx, |state, _| {
            assert_entries(
                &state.entries,
                indoc! {
                    r#"
                src
                    ui
                        button.rs
                        icon.rs
                        mod.rs
                    lib.rs
                Cargo.toml
                Cargo.lock
                README.md
                "#
                },
            );

            let entry = state.entries.get(0).unwrap();
            assert_eq!(entry.depth(), 0);
            assert_eq!(entry.is_root(), true);
            assert_eq!(entry.is_folder(), true);
            assert_eq!(entry.is_expanded(), true);

            let entry = state.entries.get(1).unwrap();
            assert_eq!(entry.depth(), 1);
            assert_eq!(entry.is_root(), false);
            assert_eq!(entry.is_folder(), true);
            assert_eq!(entry.is_expanded(), true);
            assert_eq!(entry.item().label.as_str(), "ui");

            state.toggle_expand(1);
            let entry = state.entries.get(1).unwrap();
            assert_eq!(entry.is_expanded(), false);
            assert_entries(
                &state.entries,
                indoc! {
                    r#"
                src
                    ui
                    lib.rs
                Cargo.toml
                Cargo.lock
                README.md
                "#
                },
            );
        })
    }
}
