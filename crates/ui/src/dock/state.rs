use crate::ham_gpui::{
    App, AppContext, Axis, Bounds, Entity, Pixels, WeakEntity, Window, point, px, size,
};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};

use super::{Panel, PanelRegistry, Yerlesim, YerlesimAlani, YerlesimKonumu, YerlesimOgesi};

/// YerlesimAlani serileştirme ve seriden çıkarma işlemleri için kullanılır.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct YerlesimAlaniDurumu {
    /// Kalıcı durumun geçerli sürümle uyumlu olup olmadığını işaretlemek için kullanılır.
    /// Örneğin panel yapısını büyük ölçüde değiştirdiğimiz zamanlarda sürümü
    /// karşılaştırarak durumu kullanıp kullanmayacağımıza karar verebiliriz.
    #[serde(default)]
    pub version: Option<usize>,
    pub center: PanelState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_dock: Option<YerlesimDurumu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_dock: Option<YerlesimDurumu>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_dock: Option<YerlesimDurumu>,
}

/// Yerlesim serileştirme ve seriden çıkarma işlemleri için kullanılır.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YerlesimDurumu {
    panel: PanelState,
    placement: YerlesimKonumu,
    size: Pixels,
    open: bool,
}

impl YerlesimDurumu {
    pub fn new(dock: Entity<Yerlesim>, cx: &App) -> Self {
        let dock = dock.read(cx);

        Self {
            placement: dock.placement,
            size: dock.size,
            open: dock.open,
            panel: dock.panel.view().dump(cx),
        }
    }

    /// YerlesimDurumu için Yerlesim dönüştürür.
    pub fn to_dock(
        &self,
        dock_area: WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Yerlesim> {
        let item = self.panel.to_item(dock_area.clone(), window, cx);
        cx.new(|cx| {
            Yerlesim::from_state(
                dock_area.clone(),
                self.placement,
                self.size,
                item,
                self.open,
                window,
                cx,
            )
        })
    }
}

/// İçin kullanılır serialize ve deserialize DockerItem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PanelState {
    pub panel_name: String,
    pub children: Vec<PanelState>,
    pub info: PanelInfo,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileMeta {
    pub bounds: Bounds<Pixels>,
    pub z_index: usize,
}

impl Default for TileMeta {
    fn default() -> Self {
        Self {
            bounds: Bounds {
                origin: point(px(10.), px(10.)),
                size: size(px(200.), px(200.)),
            },
            z_index: 0,
        }
    }
}

impl From<Bounds<Pixels>> for TileMeta {
    fn from(bounds: Bounds<Pixels>) -> Self {
        Self { bounds, z_index: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PanelInfo {
    #[serde(rename = "stack")]
    Stack {
        sizes: Vec<Pixels>,
        axis: usize, // 0 for horizontal, 1 for vertical
    },
    #[serde(rename = "tabs")]
    Tabs { active_index: usize },
    #[serde(rename = "panel")]
    Panel(serde_json::Value),
    #[serde(rename = "tiles")]
    Tiles { metas: Vec<TileMeta> },
}

impl PanelInfo {
    pub fn stack(sizes: Vec<Pixels>, axis: Axis) -> Self {
        Self::Stack {
            sizes,
            axis: if axis == Axis::Horizontal { 0 } else { 1 },
        }
    }

    pub fn tabs(active_index: usize) -> Self {
        Self::Tabs { active_index }
    }

    pub fn panel(info: serde_json::Value) -> Self {
        Self::Panel(info)
    }

    pub fn tiles(metas: Vec<TileMeta>) -> Self {
        Self::Tiles { metas }
    }

    pub fn axis(&self) -> Option<Axis> {
        match self {
            Self::Stack { axis, .. } => Some(if *axis == 0 {
                Axis::Horizontal
            } else {
                Axis::Vertical
            }),
            _ => None,
        }
    }

    pub fn sizes(&self) -> Option<&Vec<Pixels>> {
        match self {
            Self::Stack { sizes, .. } => Some(sizes),
            _ => None,
        }
    }

    pub fn active_index(&self) -> Option<usize> {
        match self {
            Self::Tabs { active_index } => Some(*active_index),
            _ => None,
        }
    }
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            panel_name: "".to_string(),
            children: Vec::new(),
            info: PanelInfo::Panel(serde_json::Value::Null),
        }
    }
}

impl PanelState {
    pub fn new<P: Panel>(panel: &P) -> Self {
        Self {
            panel_name: panel.panel_name().to_string(),
            ..Default::default()
        }
    }

    pub fn add_child(&mut self, panel: PanelState) {
        self.children.push(panel);
    }

    pub fn to_item(
        &self,
        dock_area: WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut App,
    ) -> YerlesimOgesi {
        let info = self.info.clone();

        let items: Vec<YerlesimOgesi> = self
            .children
            .iter()
            .map(|child| child.to_item(dock_area.clone(), window, cx))
            .collect();

        match info {
            PanelInfo::Stack { sizes, axis } => {
                let axis = if axis == 0 {
                    Axis::Horizontal
                } else {
                    Axis::Vertical
                };
                let sizes = sizes.iter().map(|s| Some(*s)).collect_vec();
                YerlesimOgesi::split_with_sizes(axis, items, sizes, &dock_area, window, cx)
            }
            PanelInfo::Tabs { active_index } => {
                if items.len() == 1 {
                    return items[0].clone();
                }

                let items = items
                    .iter()
                    .flat_map(|item| match item {
                        YerlesimOgesi::Tabs { items, .. } => items.clone(),
                        _ => {
                            // ignore invalid panels in tabs
                            vec![]
                        }
                    })
                    .collect_vec();

                YerlesimOgesi::tabs(items, &dock_area, window, cx).active_index(active_index, cx)
            }
            PanelInfo::Panel(_) => {
                let view = PanelRegistry::build_panel(
                    &self.panel_name,
                    dock_area.clone(),
                    self,
                    &info,
                    window,
                    cx,
                );
                YerlesimOgesi::tabs(vec![view.into()], &dock_area, window, cx)
            }
            PanelInfo::Tiles { metas } => {
                YerlesimOgesi::tiles(items, metas, &dock_area, window, cx)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ham_gpui::px;

    use super::*;
    #[test]
    fn test_deserialize_item_state() {
        let json = include_str!("../fixtures/layout.json");
        let state: YerlesimAlaniDurumu = serde_json::from_str(json).unwrap();
        assert_eq!(state.version, None);
        assert_eq!(state.center.panel_name, "StackPanel");
        assert_eq!(state.center.children.len(), 2);
        assert_eq!(state.center.children[0].panel_name, "TabPanel");
        assert_eq!(state.center.children[1].children.len(), 1);
        assert_eq!(
            state.center.children[1].children[0].panel_name,
            "StoryContainer"
        );
        assert_eq!(state.center.children[1].panel_name, "TabPanel");

        let left_dock = state.left_dock.unwrap();
        assert_eq!(left_dock.open, true);
        assert_eq!(left_dock.size, px(350.0));
        assert_eq!(left_dock.placement, YerlesimKonumu::Left);
        assert_eq!(left_dock.panel.panel_name, "TabPanel");
        assert_eq!(left_dock.panel.children.len(), 1);
        assert_eq!(left_dock.panel.children[0].panel_name, "StoryContainer");

        let bottom_dock = state.bottom_dock.unwrap();
        assert_eq!(bottom_dock.open, true);
        assert_eq!(bottom_dock.size, px(200.0));
        assert_eq!(bottom_dock.panel.panel_name, "TabPanel");
        assert_eq!(bottom_dock.panel.children.len(), 2);
        assert_eq!(bottom_dock.panel.children[0].panel_name, "StoryContainer");

        let right_dock = state.right_dock.unwrap();
        assert_eq!(right_dock.open, true);
        assert_eq!(right_dock.size, px(320.0));
        assert_eq!(right_dock.panel.panel_name, "TabPanel");
        assert_eq!(right_dock.panel.children.len(), 1);
        assert_eq!(right_dock.panel.children[0].panel_name, "StoryContainer");
    }
}
