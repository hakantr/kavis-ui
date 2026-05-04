use gpui::{App, Styled};

use crate::{
    Boyutlandirilabilir as _, EtkinTema as _, Simge, SimgeAdi,
    button::{Dugme, DugmeVaryantlari as _},
};

#[inline]
pub(crate) fn clear_button(cx: &App) -> Dugme {
    Dugme::new("clean")
        .icon(Simge::new(SimgeAdi::CircleX))
        .ghost()
        .xsmall()
        .tab_stop(false)
        .text_color(cx.theme().muted_foreground)
}
