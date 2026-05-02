use std::path::PathBuf;

use autocorrect::ignorer::Ignorer;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, KeyBinding, ParentElement, Render,
    Styled, Window, actions, prelude::FluentBuilder as _, px,
};

use kavis_ui::{
    EtkinTema as _, SimgeAdi,
    button::Dugme,
    dock::PanelControl,
    h_flex,
    label::Etiket,
    list::ListeOgesi,
    tree::{AgacDurumu, AgacOgesi, tree},
    v_flex,
};
use rand::prelude::IndexedRandom as _;

use crate::{Story, section};

actions!(story, [Rename, OpenFile, Delete]);

const CONTEXT: &str = "TreeStory";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("enter", Rename, Some(CONTEXT))]);
}

pub struct TreeStory {
    tree_state: Entity<AgacDurumu>,
    items: Vec<AgacOgesi>,
}

fn build_file_items(ignorer: &Ignorer, root: &PathBuf, path: &PathBuf) -> Vec<AgacOgesi> {
    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path.strip_prefix(root).unwrap_or(&path);
            if ignorer.is_ignored(&relative_path.to_string_lossy())
                || relative_path.ends_with(".git")
            {
                continue;
            }
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let id = path.to_string_lossy().to_string();
            if path.is_dir() {
                let children = build_file_items(ignorer, &root, &path);
                items.push(AgacOgesi::new(id, file_name).children(children));
            } else {
                items.push(AgacOgesi::new(id, file_name));
            }
        }
    }
    items.sort_by(|a, b| {
        b.is_folder()
            .cmp(&a.is_folder())
            .then(a.label.cmp(&b.label))
    });
    items
}

impl TreeStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn load_files(state: Entity<AgacDurumu>, path: PathBuf, cx: &mut Context<Self>) {
        cx.spawn(async move |weak_self, cx| {
            let ignorer = Ignorer::new(&path.to_string_lossy());
            let items = build_file_items(&ignorer, &path, &path);
            _ = state.update(cx, |state, cx| {
                state.set_items(items.clone(), cx);
            });

            _ = weak_self.update(cx, |this, cx| {
                this.items = items;
                cx.notify();
            })
        })
        .detach();
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let tree_state = cx.new(|cx| AgacDurumu::new(cx));

        Self::load_files(tree_state.clone(), PathBuf::from("./"), cx);

        Self {
            tree_state,
            items: Vec::new(),
        }
    }

    fn on_action_rename(&mut self, _: &Rename, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if let Some(entry) = self.tree_state.read(cx).selected_entry() {
            let item = entry.item();
            println!("Öğe yeniden adlandırılıyor: {} ({})", item.label, item.id);
        }
    }

    fn on_action_open(&mut self, _: &OpenFile, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if let Some(entry) = self.tree_state.read(cx).selected_entry() {
            let item = entry.item();
            println!("Öğe açılıyor: {} ({})", item.label, item.id);
        }
    }

    fn on_action_delete(&mut self, _: &Delete, _: &mut Window, cx: &mut gpui::Context<Self>) {
        if let Some(entry) = self.tree_state.read(cx).selected_entry() {
            let item = entry.item();
            println!("Öğe siliniyor: {} ({})", item.label, item.id);
        }
    }
}

impl Story for TreeStory {
    fn title() -> &'static str {
        "Agac"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelControl> {
        None
    }
}

impl Render for TreeStory {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let view = cx.entity();
        v_flex()
            .w_full()
            .gap_3()
            .id("tree-story")
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::on_action_rename))
            .on_action(cx.listener(Self::on_action_open))
            .on_action(cx.listener(Self::on_action_delete))
            .child(
                h_flex().gap_3().child(
                    Dugme::new("select-item")
                        .outline()
                        .label("Öğe Seç")
                        .on_click(cx.listener(|this, _, _, cx| {
                            let mut rng = rand::rng();
                            if let Some(random_item) = this.items.choose(&mut rng) {
                                this.tree_state.update(cx, |state, cx| {
                                    state.set_selected_item(Some(random_item), cx);
                                });
                            }
                        })),
                ),
            )
            .child(
                section("File tree")
                    .sub_title("Yeniden adlandırmak için `enter` tuşuna basın. Bağlam menüsü için sağ tıklayın.")
                    .max_w_md()
                    .child(
                        v_flex()
                            .w_full()
                            .gap_4()
                            .child(
                                tree(
                                    &self.tree_state,
                                    move |ix, entry, _selected, _window, cx| {
                                        view.update(cx, |_, cx| {
                                            let item = entry.item();
                                            let icon = if !entry.is_folder() {
                                                SimgeAdi::File
                                            } else if entry.is_expanded() {
                                                SimgeAdi::FolderOpen
                                            } else {
                                                SimgeAdi::Folder
                                            };

                                            ListeOgesi::new(ix)
                                                .w_full()
                                                .rounded(cx.theme().radius)
                                                .px_3()
                                                .pl(px(16.) * entry.depth() + px(12.))
                                                .child(
                                                    h_flex()
                                                        .gap_2()
                                                        .child(icon)
                                                        .child(item.label.clone()),
                                                )
                                                .on_click(cx.listener({
                                                    let item = item.clone();
                                                    move |_, _, _window, _| {
                                                        println!(
                                                            "Öğeye tıklandı: {} ({})",
                                                            item.label, item.id
                                                        );
                                                    }
                                                }))
                                        })
                                    },
                                )
                                .context_menu(|_ix, entry, menu, _window, _cx| {
                                    let is_folder = entry.is_folder();
                                    menu.when(!is_folder, |m| m.menu("Aç", Box::new(OpenFile)))
                                        .menu("Yeniden Adlandır", Box::new(Rename))
                                        .separator()
                                        .menu("Sil", Box::new(Delete))
                                })
                                .p_1()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded(cx.theme().radius)
                                .h(px(540.)),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .gap_3()
                                    .children(
                                        self.tree_state
                                            .read(cx)
                                            .selected_index()
                                            .map(|ix| format!("Seçili İndeks: {}", ix)),
                                    )
                                    .children(self.tree_state.read(cx).selected_item().map(
                                        |item| Etiket::new("Seçili:").secondary(item.id.clone()),
                                    )),
                            ),
                    ),
            )
    }
}
