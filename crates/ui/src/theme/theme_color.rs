use std::sync::Arc;

use crate::{TemaModu, theme::DEFAULT_THEME_COLORS};

use gpui::Hsla;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Tema renkler kullanılır throughout UI bileşenler.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct TemaRengi {
    /// İçin kullanılır accents böyle olarak üzerine gelme arka plan üzerinde MenuItem, ListeOgesi, etc.
    pub accent: Hsla,
    /// İçin kullanılır accent metin renk.
    pub accent_foreground: Hsla,
    /// Akordeon arka plan renk.
    pub accordion: Hsla,
    /// Akordeon üzerine gelme arka plan renk.
    pub accordion_hover: Hsla,
    /// varsayılan arka plan renk.
    pub background: Hsla,
    /// varsayılan kenarlık renk
    pub border: Hsla,
    /// Dugme birincil arka plan renk, yedek için `primary`.
    pub button_primary: Hsla,
    /// Dugme birincil etkin arka plan renk, yedek için `primary_active`.
    pub button_primary_active: Hsla,
    /// Dugme birincil metin renk, yedek için `primary_foreground`.
    pub button_primary_foreground: Hsla,
    /// Dugme birincil üzerine gelme arka plan renk, yedek için `primary_hover`.
    pub button_primary_hover: Hsla,
    /// arka plan renk için GrupKutusu.
    pub group_box: Hsla,
    /// metin renk için GrupKutusu.
    pub group_box_foreground: Hsla,
    /// girdi caret renk (Blinking imleç).
    pub caret: Hsla,
    /// Chart 1 renk.
    pub chart_1: Hsla,
    /// Chart 2 renk.
    pub chart_2: Hsla,
    /// Chart 3 renk.
    pub chart_3: Hsla,
    /// Chart 4 renk.
    pub chart_4: Hsla,
    /// Chart 5 renk.
    pub chart_5: Hsla,
    /// Bullish renk için candlestick charts (upward price movement).
    pub chart_bullish: Hsla,
    /// Bearish renk için candlestick charts (downward price movement).
    pub chart_bearish: Hsla,
    /// Danger arka plan renk.
    pub danger: Hsla,
    /// Danger etkin arka plan renk.
    pub danger_active: Hsla,
    /// Danger metin renk.
    pub danger_foreground: Hsla,
    /// Danger üzerine gelme arka plan renk.
    pub danger_hover: Hsla,
    /// açıklama Liste etiket arka plan renk.
    pub description_list_label: Hsla,
    /// açıklama Liste etiket ön plan renk.
    pub description_list_label_foreground: Hsla,
    /// Drag kenarlık renk.
    pub drag_border: Hsla,
    /// Drop hedef arka plan renk.
    pub drop_target: Hsla,
    /// varsayılan metin renk.
    pub foreground: Hsla,
    /// Info arka plan renk.
    pub info: Hsla,
    /// Info etkin arka plan renk.
    pub info_active: Hsla,
    /// Info metin renk.
    pub info_foreground: Hsla,
    /// Info üzerine gelme arka plan renk.
    pub info_hover: Hsla,
    /// Border renk için inputs böyle olarak girdi, Secim, etc.
    pub input: Hsla,
    /// Link metin renk.
    pub link: Hsla,
    /// Active link metin renk.
    pub link_active: Hsla,
    /// Hover link metin renk.
    pub link_hover: Hsla,
    /// arka plan renk için Liste ve ListeOgesi.
    pub list: Hsla,
    /// arka plan renk için etkin ListeOgesi.
    pub list_active: Hsla,
    /// Border renk için etkin ListeOgesi.
    pub list_active_border: Hsla,
    /// Stripe arka plan renk için even ListeOgesi.
    pub list_even: Hsla,
    /// arka plan renk için Liste başlık.
    pub list_head: Hsla,
    /// Hover arka plan renk için ListeOgesi.
    pub list_hover: Hsla,
    /// Muted backgrounds böyle olarak Iskelet ve Anahtar.
    pub muted: Hsla,
    /// Muted metin renk, olarak kullanılır içinde devre dışı metin.
    pub muted_foreground: Hsla,
    /// arka plan renk için AcilirKatman.
    pub popover: Hsla,
    /// metin renk için AcilirKatman.
    pub popover_foreground: Hsla,
    /// Primary arka plan renk.
    pub primary: Hsla,
    /// Active birincil arka plan renk.
    pub primary_active: Hsla,
    /// Primary metin renk.
    pub primary_foreground: Hsla,
    /// Hover birincil arka plan renk.
    pub primary_hover: Hsla,
    /// Ilerleme çubuk arka plan renk.
    pub progress_bar: Hsla,
    /// İçin kullanılır odak ring.
    pub ring: Hsla,
    /// KaydirmaCubugu arka plan renk.
    pub scrollbar: Hsla,
    /// KaydirmaCubugu tutamak arka plan renk.
    pub scrollbar_thumb: Hsla,
    /// KaydirmaCubugu tutamak üzerine gelme arka plan renk.
    pub scrollbar_thumb_hover: Hsla,
    /// Secondary arka plan renk.
    pub secondary: Hsla,
    /// Active ikincil arka plan renk.
    pub secondary_active: Hsla,
    /// Secondary metin renk, için kullanılır ikincil Dugme metin renk veya ikincil metin.
    pub secondary_foreground: Hsla,
    /// Hover ikincil arka plan renk.
    pub secondary_hover: Hsla,
    /// girdi seçim arka plan renk.
    pub selection: Hsla,
    /// YanCubuk arka plan renk.
    pub sidebar: Hsla,
    /// YanCubuk accent arka plan renk.
    pub sidebar_accent: Hsla,
    /// YanCubuk accent metin renk.
    pub sidebar_accent_foreground: Hsla,
    /// YanCubuk kenarlık renk.
    pub sidebar_border: Hsla,
    /// YanCubuk metin renk.
    pub sidebar_foreground: Hsla,
    /// YanCubuk birincil arka plan renk.
    pub sidebar_primary: Hsla,
    /// YanCubuk birincil metin renk.
    pub sidebar_primary_foreground: Hsla,
    /// Iskelet arka plan renk.
    pub skeleton: Hsla,
    /// Kaydirici çubuk arka plan renk.
    pub slider_bar: Hsla,
    /// Kaydirici tutamak arka plan renk.
    pub slider_thumb: Hsla,
    /// Success arka plan renk.
    pub success: Hsla,
    /// Success metin renk.
    pub success_foreground: Hsla,
    /// Success üzerine gelme arka plan renk.
    pub success_hover: Hsla,
    /// Success etkin arka plan renk.
    pub success_active: Hsla,
    /// Anahtar arka plan renk.
    pub switch: Hsla,
    /// Anahtar tutamak arka plan renk.
    pub switch_thumb: Hsla,
    /// Sekme arka plan renk.
    pub tab: Hsla,
    /// Sekme etkin arka plan renk.
    pub tab_active: Hsla,
    /// Sekme etkin metin renk.
    pub tab_active_foreground: Hsla,
    /// SekmeCubugu arka plan renk.
    pub tab_bar: Hsla,
    /// SekmeCubugu segmented arka plan renk.
    pub tab_bar_segmented: Hsla,
    /// Sekme metin renk.
    pub tab_foreground: Hsla,
    /// Tablo arka plan renk.
    pub table: Hsla,
    /// Tablo etkin öğe arka plan renk.
    pub table_active: Hsla,
    /// Tablo etkin öğe kenarlık renk.
    pub table_active_border: Hsla,
    /// Stripe arka plan renk için even TabloSatiri.
    pub table_even: Hsla,
    /// Tablo başlık arka plan renk.
    pub table_head: Hsla,
    /// Tablo başlık metin renk.
    pub table_head_foreground: Hsla,
    /// Tablo alt bilgi arka plan renk.
    pub table_foot: Hsla,
    /// Tablo alt bilgi metin renk.
    pub table_foot_foreground: Hsla,
    /// Tablo öğe üzerine gelme arka plan renk.
    pub table_hover: Hsla,
    /// Tablo satır kenarlık renk.
    pub table_row_border: Hsla,
    /// BaslikCubugu arka plan renk, kullanmak için Pencere başlık çubuk.
    pub title_bar: Hsla,
    /// BaslikCubugu kenarlık renk.
    pub title_bar_border: Hsla,
    /// arka plan renk için Tiles.
    pub tiles: Hsla,
    /// Warning arka plan renk.
    pub warning: Hsla,
    /// Warning etkin arka plan renk.
    pub warning_active: Hsla,
    /// Warning üzerine gelme arka plan renk.
    pub warning_hover: Hsla,
    /// Warning ön plan renk.
    pub warning_foreground: Hsla,
    /// Overlay arka plan renk.
    pub overlay: Hsla,
    /// Pencere kenarlık renk.
    ///
    /// # Platform özel:
    ///
    /// Bu yalnızca çalışır üzerinde Linux, diğer platforms biz olabilir't değişir pencere kenarlık renk.
    pub window_border: Hsla,

    /// taban red renk.
    pub red: Hsla,
    /// taban red açık renk.
    pub red_light: Hsla,
    /// taban green renk.
    pub green: Hsla,
    /// taban green açık renk.
    pub green_light: Hsla,
    /// taban blue renk.
    pub blue: Hsla,
    /// taban blue açık renk.
    pub blue_light: Hsla,
    /// taban yellow renk.
    pub yellow: Hsla,
    /// taban yellow açık renk.
    pub yellow_light: Hsla,
    /// taban magenta renk.
    pub magenta: Hsla,
    /// taban magenta açık renk.
    pub magenta_light: Hsla,
    /// taban cyan renk.
    pub cyan: Hsla,
    /// taban cyan açık renk.
    pub cyan_light: Hsla,
}

impl TemaRengi {
    /// varsayılan açık tema renkler döndürür.
    pub fn light() -> Arc<Self> {
        DEFAULT_THEME_COLORS[&TemaModu::Light].0.clone()
    }

    /// varsayılan koyu tema renkler döndürür.
    pub fn dark() -> Arc<Self> {
        DEFAULT_THEME_COLORS[&TemaModu::Dark].0.clone()
    }
}
