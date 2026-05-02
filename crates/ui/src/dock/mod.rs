mod dock;
mod invalid_panel;
mod panel;
mod stack_panel;
mod state;
mod tab_panel;
mod tiles;

use anyhow::Result;
use gpui::{
    AnyElement, AnyView, App, AppContext, Axis, Bounds, Context, Edges, Entity, EntityId,
    EventEmitter, InteractiveElement as _, IntoElement, ParentElement as _, Pixels, Render,
    SharedString, Styled, Subscription, WeakEntity, Window, actions, div, prelude::FluentBuilder,
};
use std::sync::Arc;

pub use dock::*;
pub use panel::*;
pub use stack_panel::*;
pub use state::*;
pub use tab_panel::*;
pub use tiles::*;

use crate::ElementExt;

pub(crate) fn init(cx: &mut App) {
    PanelRegistry::init(cx);
}

actions!(dock, [ToggleZoom, ClosePanel]);

pub enum YerlesimOlayi {
    /// yerleşim yerleşim alanı sahip değişmiş, subscribers bu için save yerleşim.
    ///
    /// Yerleşim alanı düzeni her değiştiğinde bu olay yayılır.
    /// Çok sık yayılabileceği için gerekirse olayı geciktirebilirsiniz.
    LayoutChanged,

    /// Sürüklenen öğe bırakma olayı.
    DragDrop(AnyDrag),
}

/// Ana yerleşim alanı.
pub struct YerlesimAlani {
    id: SharedString,
    /// Özel varsayılan yerleşim için kullanılan sürüm; [`panel`](panel) içindeki
    /// `panel_version` değerine benzer.
    version: Option<usize>,
    pub(crate) bounds: Bounds<Pixels>,

    /// Yerleşim alanının merkez görünümü.
    center: YerlesimOgesi,
    /// Yerleşim alanının sol dock'u.
    left_dock: Option<Entity<Yerlesim>>,
    /// Yerleşim alanının alt dock'u.
    bottom_dock: Option<Entity<Yerlesim>>,
    /// Yerleşim alanının sağ dock'u.
    right_dock: Option<Entity<Yerlesim>>,

    /// Her açıp kapatma düğmesinin gösterileceği [`SekmePaneli`](SekmePaneli) entity_id değeri.
    toggle_button_panels: Edges<Option<EntityId>>,

    /// Açıp kapatma düğmesinin gösterilip gösterilmeyeceği.
    toggle_button_visible: bool,
    /// Varsa yerleşim alanının üst yakınlaştırma görünümü.
    zoom_view: Option<AnyView>,

    /// Panel yerleşimini kilitler, ancak yeniden boyutlandırmaya izin verir.
    locked: bool,

    /// Panel stili. Varsayılan [`PanelStyle::varsayılan`](PanelStyle::varsayılan).
    pub(crate) panel_style: PanelStyle,

    _subscriptions: Vec<Subscription>,
}

/// YerlesimOgesi, yerleşim alanı düzenini temsil eden bir ağaç yapısıdır.
#[derive(Clone)]
pub enum YerlesimOgesi {
    /// Bölme yerleşim
    Split {
        axis: Axis,
        /// Kendi boyutu; yalnızca bölme panelleri oluşturmak için kullanılır.
        size: Option<Pixels>,
        items: Vec<YerlesimOgesi>,
        /// Öğelerin boyutları.
        sizes: Vec<Option<Pixels>>,
        view: Entity<StackPanel>,
    },
    /// Sekme yerleşim
    Tabs {
        /// Kendi boyutu; yalnızca bölme panelleri oluşturmak için kullanılır.
        size: Option<Pixels>,
        items: Vec<Arc<dyn PanelView>>,
        active_ix: usize,
        view: Entity<SekmePaneli>,
    },
    /// Panel yerleşimi.
    Panel {
        /// Kendi boyutu; yalnızca bölme panelleri oluşturmak için kullanılır.
        size: Option<Pixels>,
        view: Arc<dyn PanelView>,
    },
    /// Tiles yerleşimi.
    Tiles {
        /// Kendi boyutu; yalnızca bölme panelleri oluşturmak için kullanılır.
        size: Option<Pixels>,
        items: Vec<TileItem>,
        view: Entity<Tiles>,
    },
}

impl std::fmt::Debug for YerlesimOgesi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YerlesimOgesi::Split {
                axis, items, sizes, ..
            } => f
                .debug_struct("Split")
                .field("axis", axis)
                .field("items", &items.len())
                .field("sizes", sizes)
                .finish(),
            YerlesimOgesi::Tabs {
                items, active_ix, ..
            } => f
                .debug_struct("Tabs")
                .field("items", &items.len())
                .field("active_ix", active_ix)
                .finish(),
            YerlesimOgesi::Panel { .. } => f.debug_struct("Panel").finish(),
            YerlesimOgesi::Tiles { .. } => f.debug_struct("Tiles").finish(),
        }
    }
}

impl YerlesimOgesi {
    /// boyut YerlesimOgesi döndürür.
    fn get_size(&self) -> Option<Pixels> {
        match self {
            Self::Split { size, .. } => *size,
            Self::Tabs { size, .. } => *size,
            Self::Panel { size, .. } => *size,
            Self::Tiles { size, .. } => *size,
        }
    }

    /// boyut için YerlesimOgesi ayarlar.
    pub fn size(mut self, new_size: impl Into<Pixels>) -> Self {
        let new_size: Option<Pixels> = Some(new_size.into());
        match self {
            Self::Split { ref mut size, .. } => *size = new_size,
            Self::Tabs { ref mut size, .. } => *size = new_size,
            Self::Tiles { ref mut size, .. } => *size = new_size,
            Self::Panel { ref mut size, .. } => *size = new_size,
        }
        self
    }

    /// etkin indeks için YerlesimOgesi, yalnızca geçerli için [`YerlesimOgesi::Tabs`] ayarlar.
    pub fn active_index(mut self, new_active_ix: usize, cx: &mut App) -> Self {
        debug_assert!(
            matches!(self, Self::Tabs { .. }),
            "active_ix can only be set for YerlesimOgesi::Tabs"
        );

        if let Self::Tabs {
            ref mut active_ix,
            ref mut view,
            ..
        } = self
        {
            *active_ix = new_active_ix;
            view.update(cx, |tab_panel, _| {
                tab_panel.active_ix = new_active_ix;
            });
        }
        self
    }

    /// Verilen bölme yerleşimiyle YerlesimOgesi::Bölme oluşturur.
    pub fn split(
        axis: Axis,
        items: Vec<YerlesimOgesi>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let sizes = items.iter().map(|item| item.get_size()).collect();
        Self::split_with_sizes(axis, items, sizes, dock_area, window, cx)
    }

    /// Dikey bölme yerleşimiyle YerlesimOgesi oluşturur.
    pub fn v_split(
        items: Vec<YerlesimOgesi>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self::split(Axis::Vertical, items, dock_area, window, cx)
    }

    /// Yatay bölme yerleşimiyle YerlesimOgesi oluşturur.
    pub fn h_split(
        items: Vec<YerlesimOgesi>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self::split(Axis::Horizontal, items, dock_area, window, cx)
    }

    /// Her panel öğesinin belirtilen boyuta sahip olduğu bölme yerleşimiyle YerlesimOgesi oluşturur.
    ///
    /// `öğeler` ve `sizes` aynı uzunlukta olmalıdır.
    /// `sizes` içinde `None` olan indekslerde panel otomatik boyuta sahip olur.
    pub fn split_with_sizes(
        axis: Axis,
        items: Vec<YerlesimOgesi>,
        sizes: Vec<Option<Pixels>>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut items = items;
        let stack_panel = cx.new(|cx| {
            let mut stack_panel = StackPanel::new(axis, window, cx);
            for (i, item) in items.iter_mut().enumerate() {
                let view = item.view();
                let size = sizes.get(i).copied().flatten();
                stack_panel.add_panel(view.clone(), size, dock_area.clone(), window, cx)
            }

            for (i, item) in items.iter().enumerate() {
                let view = item.view();
                let size = sizes.get(i).copied().flatten();
                stack_panel.add_panel(view.clone(), size, dock_area.clone(), window, cx)
            }

            stack_panel
        });

        window.defer(cx, {
            let stack_panel = stack_panel.clone();
            let dock_area = dock_area.clone();
            move |window, cx| {
                _ = dock_area.update(cx, |this, cx| {
                    this.subscribe_panel(&stack_panel, window, cx);
                });
            }
        });

        Self::Split {
            axis,
            size: None,
            items,
            sizes,
            view: stack_panel,
        }
    }

    /// Panel yerleşimiyle YerlesimOgesi oluşturur.
    pub fn panel(panel: Arc<dyn PanelView>) -> Self {
        Self::Panel {
            size: None,
            view: panel,
        }
    }

    /// Tiles yerleşimiyle YerlesimOgesi oluşturur.
    ///
    /// Öğeler ve metalar aynı uzunlukta olmalıdır.
    pub fn tiles(
        items: Vec<YerlesimOgesi>,
        metas: Vec<impl Into<TileMeta> + Copy>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        assert!(items.len() == metas.len());

        let tile_panel = cx.new(|cx| {
            let mut tiles = Tiles::new(window, cx);
            for (ix, item) in items.clone().into_iter().enumerate() {
                match item {
                    YerlesimOgesi::Tabs { view, .. } => {
                        let meta: TileMeta = metas[ix].into();
                        let tile_item =
                            TileItem::new(Arc::new(view), meta.bounds).z_index(meta.z_index);
                        tiles.add_item(tile_item, dock_area, window, cx);
                    }
                    YerlesimOgesi::Panel { view, .. } => {
                        let meta: TileMeta = metas[ix].into();
                        let tile_item =
                            TileItem::new(view.clone(), meta.bounds).z_index(meta.z_index);
                        tiles.add_item(tile_item, dock_area, window, cx);
                    }
                    _ => {
                        // Ignore non-tabs items
                    }
                }
            }
            tiles
        });

        window.defer(cx, {
            let tile_panel = tile_panel.clone();
            let dock_area = dock_area.clone();
            move |window, cx| {
                _ = dock_area.update(cx, |this, cx| {
                    this.subscribe_panel(&tile_panel, window, cx);
                    this.subscribe_tiles_item_drop(&tile_panel, window, cx);
                });
            }
        });

        Self::Tiles {
            size: None,
            items: tile_panel.read(cx).panels.clone(),
            view: tile_panel,
        }
    }

    /// Öğelerin sekme olarak gösterildiği tabs yerleşimiyle YerlesimOgesi oluşturur.
    ///
    /// `active_ix` etkin sekme indeksidir; `None` ise ilk sekme etkin olur.
    pub fn tabs(
        items: Vec<Arc<dyn PanelView>>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut new_items: Vec<Arc<dyn PanelView>> = vec![];
        for item in items.into_iter() {
            new_items.push(item)
        }
        Self::new_tabs(new_items, None, dock_area, window, cx)
    }

    pub fn tab<P: Panel>(
        item: Entity<P>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self::new_tabs(vec![Arc::new(item.clone())], None, dock_area, window, cx)
    }

    fn new_tabs(
        items: Vec<Arc<dyn PanelView>>,
        active_ix: Option<usize>,
        dock_area: &WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let active_ix = active_ix.unwrap_or(0);
        let tab_panel = cx.new(|cx| {
            let mut tab_panel = SekmePaneli::new(None, dock_area.clone(), window, cx);
            for item in items.iter() {
                tab_panel.add_panel(item.clone(), window, cx)
            }
            tab_panel.active_ix = active_ix;
            tab_panel
        });

        Self::Tabs {
            size: None,
            items,
            active_ix,
            view: tab_panel,
        }
    }

    /// Yerleşim alanı öğesinin görünümünü döndürür.
    pub fn view(&self) -> Arc<dyn PanelView> {
        match self {
            Self::Split { view, .. } => Arc::new(view.clone()),
            Self::Tabs { view, .. } => Arc::new(view.clone()),
            Self::Tiles { view, .. } => Arc::new(view.clone()),
            Self::Panel { view, .. } => view.clone(),
        }
    }

    /// Yerleşim alanı öğesi içinde mevcut paneli bulur.
    pub fn find_panel(&self, panel: Arc<dyn PanelView>) -> Option<Arc<dyn PanelView>> {
        match self {
            Self::Split { items, .. } => {
                items.iter().find_map(|item| item.find_panel(panel.clone()))
            }
            Self::Tabs { items, .. } => items.iter().find(|item| *item == &panel).cloned(),
            Self::Panel { view, .. } => Some(view.clone()),
            Self::Tiles { items, .. } => items.iter().find_map(|item| {
                if &item.panel == &panel {
                    Some(item.panel.clone())
                } else {
                    None
                }
            }),
        }
    }

    /// Yerleşim alanı öğesine bir panel ekler.
    pub fn add_panel(
        &mut self,
        panel: Arc<dyn PanelView>,
        dock_area: &WeakEntity<YerlesimAlani>,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        match self {
            Self::Tabs { view, items, .. } => {
                items.push(panel.clone());
                view.update(cx, |tab_panel, cx| {
                    tab_panel.add_panel(panel, window, cx);
                });
            }
            Self::Split { view, items, .. } => {
                // Iter items to add panel to the first tabs
                for item in items.into_iter() {
                    if let YerlesimOgesi::Tabs { view, .. } = item {
                        view.update(cx, |tab_panel, cx| {
                            tab_panel.add_panel(panel.clone(), window, cx);
                        });
                        return;
                    }
                }

                // Unable to find tabs, create new tabs
                let new_item = Self::tabs(vec![panel.clone()], dock_area, window, cx);
                items.push(new_item.clone());
                view.update(cx, |stack_panel, cx| {
                    stack_panel.add_panel(new_item.view(), None, dock_area.clone(), window, cx);
                });
            }
            Self::Tiles { view, items, .. } => {
                let tile_item = TileItem::new(
                    Arc::new(cx.new(|cx| {
                        let mut tab_panel = SekmePaneli::new(None, dock_area.clone(), window, cx);
                        tab_panel.add_panel(panel.clone(), window, cx);
                        tab_panel
                    })),
                    bounds.unwrap_or_else(|| TileMeta::default().bounds),
                );

                items.push(tile_item.clone());
                view.update(cx, |tiles, cx| {
                    tiles.add_item(tile_item, dock_area, window, cx);
                });
            }
            Self::Panel { .. } => {}
        }
    }

    /// Yerleşim alanı öğesinden bir panel kaldırır.
    pub fn remove_panel(&self, panel: Arc<dyn PanelView>, window: &mut Window, cx: &mut App) {
        match self {
            YerlesimOgesi::Tabs { view, .. } => {
                view.update(cx, |tab_panel, cx| {
                    tab_panel.remove_panel(panel, window, cx);
                });
            }
            YerlesimOgesi::Split { items, view, .. } => {
                // For each child item, set collapsed state
                for item in items {
                    item.remove_panel(panel.clone(), window, cx);
                }
                view.update(cx, |split, cx| {
                    split.remove_panel(panel, window, cx);
                });
            }
            YerlesimOgesi::Tiles { view, .. } => {
                view.update(cx, |tiles, cx| {
                    tiles.remove(panel, window, cx);
                });
            }
            YerlesimOgesi::Panel { .. } => {}
        }
    }

    pub fn set_collapsed(&self, collapsed: bool, window: &mut Window, cx: &mut App) {
        match self {
            YerlesimOgesi::Tabs { view, .. } => {
                view.update(cx, |tab_panel, cx| {
                    tab_panel.set_collapsed(collapsed, window, cx);
                });
            }
            YerlesimOgesi::Split { items, .. } => {
                // For each child item, set collapsed state
                for item in items {
                    item.set_collapsed(collapsed, window, cx);
                }
            }
            YerlesimOgesi::Tiles { .. } => {}
            YerlesimOgesi::Panel { view, .. } => view.set_active(!collapsed, window, cx),
        }
    }

    /// Sol ve üst taraftaki en uç SekmePaneli bulmak için özyinelemeli gezer.
    pub(crate) fn left_top_tab_panel(&self, cx: &App) -> Option<Entity<SekmePaneli>> {
        match self {
            YerlesimOgesi::Tabs { view, .. } => Some(view.clone()),
            YerlesimOgesi::Split { view, .. } => view.read(cx).left_top_tab_panel(true, cx),
            YerlesimOgesi::Tiles { .. } => None,
            YerlesimOgesi::Panel { .. } => None,
        }
    }

    /// Sağ ve üst taraftaki en uç SekmePaneli bulmak için özyinelemeli gezer.
    pub(crate) fn right_top_tab_panel(&self, cx: &App) -> Option<Entity<SekmePaneli>> {
        match self {
            YerlesimOgesi::Tabs { view, .. } => Some(view.clone()),
            YerlesimOgesi::Split { view, .. } => view.read(cx).right_top_tab_panel(true, cx),
            YerlesimOgesi::Tiles { .. } => None,
            YerlesimOgesi::Panel { .. } => None,
        }
    }
}

impl YerlesimAlani {
    pub fn new(
        id: impl Into<SharedString>,
        version: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let stack_panel = cx.new(|cx| StackPanel::new(Axis::Horizontal, window, cx));

        let dock_item = YerlesimOgesi::Split {
            axis: Axis::Horizontal,
            size: None,
            items: vec![],
            sizes: vec![],
            view: stack_panel.clone(),
        };

        let mut this = Self {
            id: id.into(),
            version,
            bounds: Bounds::default(),
            center: dock_item,
            left_dock: None,
            right_dock: None,
            bottom_dock: None,
            zoom_view: None,
            toggle_button_panels: Edges::default(),
            toggle_button_visible: true,
            locked: false,
            panel_style: PanelStyle::default(),
            _subscriptions: vec![],
        };

        this.subscribe_panel(&stack_panel, window, cx);

        this
    }

    /// Yerleşim alanı sınırlarını döndürür.
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    /// Tiles öğesindeki sürüklenen öğe bırakma olayına abone olur.
    fn subscribe_tiles_item_drop(
        &mut self,
        tile_panel: &Entity<Tiles>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self._subscriptions
            .push(cx.subscribe(tile_panel, move |_, _, evt: &DragDrop, cx| {
                let item = evt.0.clone();
                cx.emit(YerlesimOlayi::DragDrop(item));
            }));
    }

    /// Yerleşim alanı panel stilini ayarlar.
    pub fn panel_style(mut self, style: PanelStyle) -> Self {
        self.panel_style = style;
        self
    }

    /// Yerleşim alanı sürümünü ayarlar.
    pub fn set_version(&mut self, version: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.version = Some(version);
        cx.notify();
    }

    /// Merkez yerleşim alanı öğesini döndürür.
    pub fn center(&self) -> &YerlesimOgesi {
        &self.center
    }

    /// sol yerleşim alanı öğe döndürür.
    pub fn left_dock(&self) -> Option<&Entity<Yerlesim>> {
        self.left_dock.as_ref()
    }

    /// alt yerleşim alanı öğe döndürür.
    pub fn bottom_dock(&self) -> Option<&Entity<Yerlesim>> {
        self.bottom_dock.as_ref()
    }

    /// sağ yerleşim alanı öğe döndürür.
    pub fn right_dock(&self) -> Option<&Entity<Yerlesim>> {
        self.right_dock.as_ref()
    }

    /// Kaldırır sol yerleşim alanı.
    pub fn remove_left_dock(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.left_dock = None;
    }

    /// Kaldırır alt yerleşim alanı.
    pub fn remove_bottom_dock(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.bottom_dock = None;
    }

    /// Kaldırır sağ yerleşim alanı.
    pub fn remove_right_dock(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.right_dock = None;
    }

    /// Merkez yerleşim alanını YerlesimOgesi olarak verir.
    ///
    /// Merkez YerlesimAlani çizimi için kullanılır.
    pub fn set_center(
        &mut self,
        center: YerlesimOgesi,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.subscribe_item(&center, window, cx);
        self.center = center;
        self.update_toggle_button_tab_panels(window, cx);
        cx.notify();
    }

    pub fn set_left_dock(
        &mut self,
        panel: YerlesimOgesi,
        size: Option<Pixels>,
        open: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.subscribe_item(&panel, window, cx);
        let weak_self = cx.entity().downgrade();
        self.left_dock = Some(cx.new(|cx| {
            let mut dock = Yerlesim::left(weak_self.clone(), window, cx);
            if let Some(size) = size {
                dock.set_size(size, window, cx);
            }
            dock.set_panel(panel, window, cx);
            dock.set_open(open, window, cx);
            dock
        }));
        self.update_toggle_button_tab_panels(window, cx);
    }

    pub fn set_bottom_dock(
        &mut self,
        panel: YerlesimOgesi,
        size: Option<Pixels>,
        open: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.subscribe_item(&panel, window, cx);
        let weak_self = cx.entity().downgrade();
        self.bottom_dock = Some(cx.new(|cx| {
            let mut dock = Yerlesim::bottom(weak_self.clone(), window, cx);
            if let Some(size) = size {
                dock.set_size(size, window, cx);
            }
            dock.set_panel(panel, window, cx);
            dock.set_open(open, window, cx);
            dock
        }));
        self.update_toggle_button_tab_panels(window, cx);
    }

    pub fn set_right_dock(
        &mut self,
        panel: YerlesimOgesi,
        size: Option<Pixels>,
        open: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.subscribe_item(&panel, window, cx);
        let weak_self = cx.entity().downgrade();
        self.right_dock = Some(cx.new(|cx| {
            let mut dock = Yerlesim::right(weak_self.clone(), window, cx);
            if let Some(size) = size {
                dock.set_size(size, window, cx);
            }
            dock.set_panel(panel, window, cx);
            dock.set_open(open, window, cx);
            dock
        }));
        self.update_toggle_button_tab_panels(window, cx);
    }

    /// Yerleşim alanının kilitli durumunu ayarlar. Kilitliyse yerleşim alanı bölünemez
    /// veya taşınamaz, ancak paneller yeniden boyutlandırılabilir.
    pub fn set_locked(&mut self, locked: bool, _window: &mut Window, _cx: &mut App) {
        self.locked = locked;
    }

    /// Yerleşim alanının kilitli olup olmadığını belirler.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Verilen yerleşimde bir yerleşim alanı olup olmadığını belirler.
    pub fn has_dock(&self, placement: YerlesimKonumu) -> bool {
        match placement {
            YerlesimKonumu::Left => self.left_dock.is_some(),
            YerlesimKonumu::Bottom => self.bottom_dock.is_some(),
            YerlesimKonumu::Right => self.right_dock.is_some(),
            YerlesimKonumu::Center => false,
        }
    }

    /// Verilen yerleşimdeki yerleşim alanının açık olup olmadığını belirler.
    pub fn is_dock_open(&self, placement: YerlesimKonumu, cx: &App) -> bool {
        match placement {
            YerlesimKonumu::Left => self
                .left_dock
                .as_ref()
                .map(|dock| dock.read(cx).is_open())
                .unwrap_or(false),
            YerlesimKonumu::Bottom => self
                .bottom_dock
                .as_ref()
                .map(|dock| dock.read(cx).is_open())
                .unwrap_or(false),
            YerlesimKonumu::Right => self
                .right_dock
                .as_ref()
                .map(|dock| dock.read(cx).is_open())
                .unwrap_or(false),
            YerlesimKonumu::Center => false,
        }
    }

    /// Verilen yerleşimdeki yerleşim alanını açık veya kapalı olarak ayarlar.
    ///
    /// Yalnızca sol, alt ve sağ yerleşim alanı açılıp kapatılabilir.
    pub fn set_dock_collapsible(
        &mut self,
        collapsible_edges: Edges<bool>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(left_dock) = self.left_dock.as_ref() {
            left_dock.update(cx, |dock, cx| {
                dock.set_collapsible(collapsible_edges.left, window, cx);
            });
        }

        if let Some(bottom_dock) = self.bottom_dock.as_ref() {
            bottom_dock.update(cx, |dock, cx| {
                dock.set_collapsible(collapsible_edges.bottom, window, cx);
            });
        }

        if let Some(right_dock) = self.right_dock.as_ref() {
            right_dock.update(cx, |dock, cx| {
                dock.set_collapsible(collapsible_edges.right, window, cx);
            });
        }
    }

    /// Verilen yerleşimdeki yerleşim alanının daraltılabilir olup olmadığını belirler.
    pub fn is_dock_collapsible(&self, placement: YerlesimKonumu, cx: &App) -> bool {
        match placement {
            YerlesimKonumu::Left => self
                .left_dock
                .as_ref()
                .map(|dock| dock.read(cx).collapsible)
                .unwrap_or(false),
            YerlesimKonumu::Bottom => self
                .bottom_dock
                .as_ref()
                .map(|dock| dock.read(cx).collapsible)
                .unwrap_or(false),
            YerlesimKonumu::Right => self
                .right_dock
                .as_ref()
                .map(|dock| dock.read(cx).collapsible)
                .unwrap_or(false),
            YerlesimKonumu::Center => false,
        }
    }

    /// Verilen yerleşimdeki yerleşim alanını açıp kapatır.
    pub fn toggle_dock(
        &self,
        placement: YerlesimKonumu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dock = match placement {
            YerlesimKonumu::Left => &self.left_dock,
            YerlesimKonumu::Bottom => &self.bottom_dock,
            YerlesimKonumu::Right => &self.right_dock,
            YerlesimKonumu::Center => return,
        };

        if let Some(dock) = dock {
            dock.update(cx, |view, cx| {
                view.toggle_open(window, cx);
            })
        }
    }

    /// Açıp kapatma düğmesi görünürlüğünü ayarlar.
    pub fn set_toggle_button_visible(&mut self, visible: bool, _: &mut Context<Self>) {
        self.toggle_button_visible = visible;
    }

    /// Verilen yerleşimdeki yerleşim alanına bir panel öğesi ekler.
    pub fn add_panel(
        &mut self,
        panel: Arc<dyn PanelView>,
        placement: YerlesimKonumu,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let weak_self = cx.entity().downgrade();
        match placement {
            YerlesimKonumu::Left => {
                if let Some(dock) = self.left_dock.as_ref() {
                    dock.update(cx, |dock, cx| dock.add_panel(panel, window, cx))
                } else {
                    self.set_left_dock(
                        YerlesimOgesi::tabs(vec![panel], &weak_self, window, cx),
                        None,
                        true,
                        window,
                        cx,
                    );
                }
            }
            YerlesimKonumu::Bottom => {
                if let Some(dock) = self.bottom_dock.as_ref() {
                    dock.update(cx, |dock, cx| dock.add_panel(panel, window, cx))
                } else {
                    self.set_bottom_dock(
                        YerlesimOgesi::tabs(vec![panel], &weak_self, window, cx),
                        None,
                        true,
                        window,
                        cx,
                    );
                }
            }
            YerlesimKonumu::Right => {
                if let Some(dock) = self.right_dock.as_ref() {
                    dock.update(cx, |dock, cx| dock.add_panel(panel, window, cx))
                } else {
                    self.set_right_dock(
                        YerlesimOgesi::tabs(vec![panel], &weak_self, window, cx),
                        None,
                        true,
                        window,
                        cx,
                    );
                }
            }
            YerlesimKonumu::Center => {
                self.center
                    .add_panel(panel, &cx.entity().downgrade(), bounds, window, cx);
            }
        }
    }

    /// Verilen yerleşim konumundaki paneli YerlesimAlani içinden kaldırır.
    pub fn remove_panel(
        &mut self,
        panel: Arc<dyn PanelView>,
        placement: YerlesimKonumu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match placement {
            YerlesimKonumu::Left => {
                if let Some(dock) = self.left_dock.as_mut() {
                    dock.update(cx, |dock, cx| {
                        dock.remove_panel(panel, window, cx);
                    });
                }
            }
            YerlesimKonumu::Right => {
                if let Some(dock) = self.right_dock.as_mut() {
                    dock.update(cx, |dock, cx| {
                        dock.remove_panel(panel, window, cx);
                    });
                }
            }
            YerlesimKonumu::Bottom => {
                if let Some(dock) = self.bottom_dock.as_mut() {
                    dock.update(cx, |dock, cx| {
                        dock.remove_panel(panel, window, cx);
                    });
                }
            }
            YerlesimKonumu::Center => {
                self.center.remove_panel(panel, window, cx);
            }
        }
        cx.notify();
    }

    /// Bir paneli tüm dock'lardan kaldırır.
    pub fn remove_panel_from_all_docks(
        &mut self,
        panel: Arc<dyn PanelView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remove_panel(panel.clone(), YerlesimKonumu::Center, window, cx);
        self.remove_panel(panel.clone(), YerlesimKonumu::Left, window, cx);
        self.remove_panel(panel.clone(), YerlesimKonumu::Right, window, cx);
        self.remove_panel(panel.clone(), YerlesimKonumu::Bottom, window, cx);
    }

    /// YerlesimAlani durumunu YerlesimAlaniDurumu değerinden yükler.
    ///
    /// Bakınız ayrıca [DockeArea::dump].
    pub fn load(
        &mut self,
        state: YerlesimAlaniDurumu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.version = state.version;
        let weak_self = cx.entity().downgrade();

        if let Some(left_dock_state) = state.left_dock {
            self.left_dock = Some(left_dock_state.to_dock(weak_self.clone(), window, cx));
        }

        if let Some(right_dock_state) = state.right_dock {
            self.right_dock = Some(right_dock_state.to_dock(weak_self.clone(), window, cx));
        }

        if let Some(bottom_dock_state) = state.bottom_dock {
            self.bottom_dock = Some(bottom_dock_state.to_dock(weak_self.clone(), window, cx));
        }

        self.center = state.center.to_item(weak_self, window, cx);
        self.update_toggle_button_tab_panels(window, cx);
        Ok(())
    }

    /// Yerleşim alanı panel düzenini PanelState olarak döker.
    ///
    /// Bakınız ayrıca [YerlesimAlani::load].
    pub fn dump(&self, cx: &App) -> YerlesimAlaniDurumu {
        let root = self.center.view();
        let center = root.dump(cx);

        let left_dock = self
            .left_dock
            .as_ref()
            .map(|dock| YerlesimDurumu::new(dock.clone(), cx));
        let right_dock = self
            .right_dock
            .as_ref()
            .map(|dock| YerlesimDurumu::new(dock.clone(), cx));
        let bottom_dock = self
            .bottom_dock
            .as_ref()
            .map(|dock| YerlesimDurumu::new(dock.clone(), cx));

        YerlesimAlaniDurumu {
            version: self.version,
            center,
            left_dock,
            right_dock,
            bottom_dock,
        }
    }

    /// Paneller üzerindeki olaylara abone olur.
    #[allow(clippy::only_used_in_recursion)]
    fn subscribe_item(
        &mut self,
        item: &YerlesimOgesi,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match item {
            YerlesimOgesi::Split { items, view, .. } => {
                for item in items {
                    self.subscribe_item(item, window, cx);
                }

                self._subscriptions.push(cx.subscribe_in(
                    view,
                    window,
                    move |_, _, event, window, cx| match event {
                        PanelEvent::LayoutChanged => {
                            cx.spawn_in(window, async move |view, window| {
                                _ = view.update_in(window, |view, window, cx| {
                                    view.update_toggle_button_tab_panels(window, cx)
                                });
                            })
                            .detach();
                            cx.emit(YerlesimOlayi::LayoutChanged);
                        }
                        _ => {}
                    },
                ));
            }
            YerlesimOgesi::Tabs { .. } => {
                // We subscribe to the tab panel event in StackPanel's insert_panel
            }
            YerlesimOgesi::Tiles { .. } => {
                // We subscribe to the tab panel event in Tiles's [`add_item`](Tiles::add_item)
            }
            YerlesimOgesi::Panel { .. } => {
                // Not supported
            }
        }
    }

    /// Panel üzerindeki yakınlaştırma olayına abone olur.
    pub(crate) fn subscribe_panel<P: Panel>(
        &mut self,
        view: &Entity<P>,
        window: &mut Window,
        cx: &mut Context<YerlesimAlani>,
    ) {
        let subscription =
            cx.subscribe_in(
                view,
                window,
                move |_, panel, event, window, cx| match event {
                    PanelEvent::ZoomIn => {
                        let panel = panel.clone();
                        cx.spawn_in(window, async move |view, window| {
                            _ = view.update_in(window, |view, window, cx| {
                                view.set_zoomed_in(panel, window, cx);
                                cx.notify();
                            });
                        })
                        .detach();
                    }
                    PanelEvent::ZoomOut => cx
                        .spawn_in(window, async move |view, window| {
                            _ = view.update_in(window, |view, window, cx| {
                                view.set_zoomed_out(window, cx);
                            });
                        })
                        .detach(),
                    PanelEvent::LayoutChanged => {
                        cx.spawn_in(window, async move |view, window| {
                            _ = view.update_in(window, |view, window, cx| {
                                view.update_toggle_button_tab_panels(window, cx)
                            });
                        })
                        .detach();
                        cx.emit(YerlesimOlayi::LayoutChanged);
                    }
                },
            );

        self._subscriptions.push(subscription);
    }

    /// ID yerleşim alanı alan döndürür.
    pub fn id(&self) -> SharedString {
        self.id.clone()
    }

    pub fn set_zoomed_in<P: Panel>(
        &mut self,
        panel: Entity<P>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.zoom_view = Some(panel.into());
        cx.notify();
    }

    pub fn set_zoomed_out(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.zoom_view = None;
        cx.notify();
    }

    fn render_items(&self, _window: &mut Window, _cx: &mut Context<Self>) -> AnyElement {
        match &self.center {
            YerlesimOgesi::Split { view, .. } => view.clone().into_any_element(),
            YerlesimOgesi::Tabs { view, .. } => view.clone().into_any_element(),
            YerlesimOgesi::Tiles { view, .. } => view.clone().into_any_element(),
            YerlesimOgesi::Panel { view, .. } => view.clone().view().into_any_element(),
        }
    }

    pub fn update_toggle_button_tab_panels(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        // Left toggle button
        self.toggle_button_panels.left = self
            .center
            .left_top_tab_panel(cx)
            .map(|view| view.entity_id());

        // Right toggle button
        self.toggle_button_panels.right = self
            .center
            .right_top_tab_panel(cx)
            .map(|view| view.entity_id());

        // Bottom toggle button
        self.toggle_button_panels.bottom = self
            .bottom_dock
            .as_ref()
            .and_then(|dock| dock.read(cx).panel.left_top_tab_panel(cx))
            .map(|view| view.entity_id());
    }
}
impl EventEmitter<YerlesimOlayi> for YerlesimAlani {}
impl Render for YerlesimAlani {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();

        div()
            .id("dock-area")
            .relative()
            .size_full()
            .overflow_hidden()
            .on_prepaint(move |bounds, _, cx| view.update(cx, |r, _| r.bounds = bounds))
            .map(|this| {
                if let Some(zoom_view) = self.zoom_view.clone() {
                    this.child(zoom_view)
                } else {
                    match &self.center {
                        YerlesimOgesi::Tiles { view, .. } => {
                            // render tiles
                            this.child(view.clone())
                        }
                        _ => {
                            // render dock
                            this.child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h_full()
                                    // Left dock
                                    .when_some(self.left_dock.clone(), |this, dock| {
                                        this.child(div().flex().flex_none().child(dock))
                                    })
                                    // Center
                                    .child(
                                        div()
                                            .flex()
                                            .flex_1()
                                            .flex_col()
                                            .overflow_hidden()
                                            // Top center
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .overflow_hidden()
                                                    .child(self.render_items(window, cx)),
                                            )
                                            // Bottom Yerlesim
                                            .when_some(self.bottom_dock.clone(), |this, dock| {
                                                this.child(dock)
                                            }),
                                    )
                                    // Right Yerlesim
                                    .when_some(self.right_dock.clone(), |this, dock| {
                                        this.child(div().flex().flex_none().child(dock))
                                    }),
                            )
                        }
                    }
                }
            })
    }
}
