use gpui::{App, Styled};

use crate::{
    EtkinTema as _, Simge, SimgeAdi, Sizable as _,
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
