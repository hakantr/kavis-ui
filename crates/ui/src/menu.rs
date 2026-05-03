use gpui::App;

mod app_menu_bar;
mod context_menu;
mod dropdown_menu;
mod menu_item;
mod popup_menu;
mod uygulama_menusu;

pub use app_menu_bar::AppMenuBar;
pub use context_menu::{ContextMenu, ContextMenuExt, ContextMenuState};
pub use dropdown_menu::DropdownMenu;
pub use popup_menu::{PopupMenu, PopupMenuItem};
pub use uygulama_menusu::{
    UygulamaMenuOgesi, UygulamaMenusu, uygulama_menu_cubugu_gerekli_mi,
    uygulama_menu_cubugu_olustur, uygulama_menu_cubugu_olustur_baglam,
    uygulama_menu_cubugu_olustur_zorla, uygulama_menu_cubugu_olustur_zorla_baglam,
    uygulama_menulerini_kaydet,
};

/// [`AppMenuBar`] icin Turkce ad.
pub type UygulamaMenuCubugu = AppMenuBar;

pub(crate) fn init(cx: &mut App) {
    app_menu_bar::init(cx);
    popup_menu::init(cx);
}
