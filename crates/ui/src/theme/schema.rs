use std::{rc::Rc, sync::Arc};

use crate::ham_gpui::{SharedString, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Renklendir, Tema, TemaModu, TemaRengi,
    highlighter::{VurguTemasi, VurguTemasiStili},
    try_parse_color,
};

/// Tema yapılandırmasını temsil eder.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct TemaKumesi {
    /// ad tema ayarlar.
    pub name: SharedString,
    /// author tema.
    pub author: Option<SharedString>,
    /// URL tema.
    pub url: Option<SharedString>,
    /// tema liste tema ayarlar.
    #[serde(rename = "themes")]
    pub themes: Vec<TemaYapilandirmasi>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct TemaYapilandirmasi {
    /// Bu temanın varsayılan tema olup olmadığını belirtir.
    pub is_default: bool,
    /// ad tema.
    pub name: SharedString,
    /// Tema modunu belirtir. Varsayılan açık moddur.
    pub mode: TemaModu,

    /// Taban font boyutu. Varsayılan 16dır.
    #[serde(rename = "font.size")]
    pub font_size: Option<f32>,
    /// Taban font ailesi. Varsayılan sistem fontudur: `.SystemUIFont`.
    #[serde(rename = "font.family")]
    pub font_family: Option<SharedString>,
    /// Monospace font ailesi. Varsayılan platforma özeldir:
    /// - macOS: `Menlo`
    /// - Windows: `Consolas`
    /// - Linux: `DejaVu Sans Mono`
    #[serde(rename = "mono_font.family")]
    pub mono_font_family: Option<SharedString>,
    /// Monospace font boyutu. Varsayılan 13tür.
    #[serde(rename = "mono_font.size")]
    pub mono_font_size: Option<f32>,

    /// Genel öğeler için kenarlık yarıçapı. Varsayılan 6dır.
    #[serde(rename = "radius")]
    pub radius: Option<usize>,
    /// Dialog ve Notification gibi büyük öğeler için kenarlık yarıçapı. Varsayılan 8dir.
    #[serde(rename = "radius.lg")]
    pub radius_lg: Option<usize>,
    /// Temada gölgelerin kullanılıp kullanılmayacağını ayarlar; örneğin girdi ve Düğme için. Varsayılan true değeridir.
    #[serde(rename = "shadow")]
    pub shadow: Option<bool>,

    /// renkler tema.
    pub colors: TemaYapilandirmaRenkleri,
    /// Vurgulama teması; bu bölüm Zed temasındaki `stil` bölümüyle uyumludur.
    ///
    /// https://github.com/zed-industries/zed/blob/f50041779dcfd7a76c8aec293361c60c53f02d51/assets/themes/ayu/ayu.json#L9
    pub highlight: Option<VurguTemasiStili>,
}

#[derive(Debug, Default, Clone, JsonSchema, Serialize, Deserialize)]
pub struct TemaYapilandirmaRenkleri {
    /// İçin kullanılır accents böyle olarak üzerine gelme arka plan üzerinde MenuItem, ListeOgesi, etc.
    #[serde(rename = "accent.background")]
    pub accent: Option<SharedString>,
    /// İçin kullanılır accent metin renk.
    #[serde(rename = "accent.foreground")]
    pub accent_foreground: Option<SharedString>,
    /// Akordeon arka plan renk.
    #[serde(rename = "accordion.background")]
    pub accordion: Option<SharedString>,
    /// Akordeon üzerine gelme arka plan renk.
    #[serde(rename = "accordion.hover.background")]
    pub accordion_hover: Option<SharedString>,
    /// varsayılan arka plan renk.
    #[serde(rename = "background")]
    pub background: Option<SharedString>,
    /// varsayılan kenarlık renk
    #[serde(rename = "border")]
    pub border: Option<SharedString>,
    /// Dugme birincil arka plan renk, yedek için `primary`.
    #[serde(rename = "button.primary.background")]
    pub button_primary: Option<SharedString>,
    /// Dugme birincil etkin arka plan renk, yedek için `primary_active`.
    #[serde(rename = "button.primary.active.background")]
    pub button_primary_active: Option<SharedString>,
    /// Dugme birincil metin renk, yedek için `primary_foreground`.
    #[serde(rename = "button.primary.foreground")]
    pub button_primary_foreground: Option<SharedString>,
    /// Dugme birincil üzerine gelme arka plan renk, yedek için `primary_hover`.
    #[serde(rename = "button.primary.hover.background")]
    pub button_primary_hover: Option<SharedString>,
    /// arka plan renk için GrupKutusu.
    #[serde(rename = "group_box.background")]
    pub group_box: Option<SharedString>,
    /// metin renk için GrupKutusu.
    #[serde(rename = "group_box.foreground")]
    pub group_box_foreground: Option<SharedString>,
    /// başlık metin renk için GrupKutusu.
    #[serde(rename = "group_box.title.foreground")]
    pub group_box_title_foreground: Option<SharedString>,
    /// girdi caret renk (Blinking imleç).
    #[serde(rename = "caret")]
    pub caret: Option<SharedString>,
    /// Chart 1 renk.
    #[serde(rename = "chart.1")]
    pub chart_1: Option<SharedString>,
    /// Chart 2 renk.
    #[serde(rename = "chart.2")]
    pub chart_2: Option<SharedString>,
    /// Chart 3 renk.
    #[serde(rename = "chart.3")]
    pub chart_3: Option<SharedString>,
    /// Chart 4 renk.
    #[serde(rename = "chart.4")]
    pub chart_4: Option<SharedString>,
    /// Chart 5 renk.
    #[serde(rename = "chart.5")]
    pub chart_5: Option<SharedString>,
    /// Bullish renk için candlestick charts (upward price movement).
    #[serde(rename = "chart_bullish")]
    pub chart_bullish: Option<SharedString>,
    /// Bearish renk için candlestick charts (downward price movement).
    #[serde(rename = "chart_bearish")]
    pub chart_bearish: Option<SharedString>,
    /// Danger arka plan renk.
    #[serde(rename = "danger.background")]
    pub danger: Option<SharedString>,
    /// Danger etkin arka plan renk.
    #[serde(rename = "danger.active.background")]
    pub danger_active: Option<SharedString>,
    /// Danger metin renk.
    #[serde(rename = "danger.foreground")]
    pub danger_foreground: Option<SharedString>,
    /// Danger üzerine gelme arka plan renk.
    #[serde(rename = "danger.hover.background")]
    pub danger_hover: Option<SharedString>,
    /// açıklama Liste etiket arka plan renk.
    #[serde(rename = "description_list.label.background")]
    pub description_list_label: Option<SharedString>,
    /// açıklama Liste etiket ön plan renk.
    #[serde(rename = "description_list.label.foreground")]
    pub description_list_label_foreground: Option<SharedString>,
    /// Drag kenarlık renk.
    #[serde(rename = "drag.border")]
    pub drag_border: Option<SharedString>,
    /// Drop hedef arka plan renk.
    #[serde(rename = "drop_target.background")]
    pub drop_target: Option<SharedString>,
    /// varsayılan metin renk.
    #[serde(rename = "foreground")]
    pub foreground: Option<SharedString>,
    /// Info arka plan renk.
    #[serde(rename = "info.background")]
    pub info: Option<SharedString>,
    /// Info etkin arka plan renk.
    #[serde(rename = "info.active.background")]
    pub info_active: Option<SharedString>,
    /// Info metin renk.
    #[serde(rename = "info.foreground")]
    pub info_foreground: Option<SharedString>,
    /// Info üzerine gelme arka plan renk.
    #[serde(rename = "info.hover.background")]
    pub info_hover: Option<SharedString>,
    /// Border renk için inputs böyle olarak girdi, Secim, etc.
    #[serde(rename = "input.border")]
    pub input: Option<SharedString>,
    /// Link metin renk.
    #[serde(rename = "link")]
    pub link: Option<SharedString>,
    /// Active link metin renk.
    #[serde(rename = "link.active")]
    pub link_active: Option<SharedString>,
    /// Hover link metin renk.
    #[serde(rename = "link.hover")]
    pub link_hover: Option<SharedString>,
    /// arka plan renk için Liste ve ListeOgesi.
    #[serde(rename = "list.background")]
    pub list: Option<SharedString>,
    /// arka plan renk için etkin ListeOgesi.
    #[serde(rename = "list.active.background")]
    pub list_active: Option<SharedString>,
    /// Border renk için etkin ListeOgesi.
    #[serde(rename = "list.active.border")]
    pub list_active_border: Option<SharedString>,
    /// Stripe arka plan renk için even ListeOgesi.
    #[serde(rename = "list.even.background")]
    pub list_even: Option<SharedString>,
    /// arka plan renk için Liste başlık.
    #[serde(rename = "list.head.background")]
    pub list_head: Option<SharedString>,
    /// Hover arka plan renk için ListeOgesi.
    #[serde(rename = "list.hover.background")]
    pub list_hover: Option<SharedString>,
    /// Muted backgrounds böyle olarak Iskelet ve Anahtar.
    #[serde(rename = "muted.background")]
    pub muted: Option<SharedString>,
    /// Muted metin renk, olarak kullanılır içinde devre dışı metin.
    #[serde(rename = "muted.foreground")]
    pub muted_foreground: Option<SharedString>,
    /// arka plan renk için AcilirKatman.
    #[serde(rename = "popover.background")]
    pub popover: Option<SharedString>,
    /// metin renk için AcilirKatman.
    #[serde(rename = "popover.foreground")]
    pub popover_foreground: Option<SharedString>,
    /// Primary arka plan renk.
    #[serde(rename = "primary.background")]
    pub primary: Option<SharedString>,
    /// Active birincil arka plan renk.
    #[serde(rename = "primary.active.background")]
    pub primary_active: Option<SharedString>,
    /// Primary metin renk.
    #[serde(rename = "primary.foreground")]
    pub primary_foreground: Option<SharedString>,
    /// Hover birincil arka plan renk.
    #[serde(rename = "primary.hover.background")]
    pub primary_hover: Option<SharedString>,
    /// Ilerleme çubuk arka plan renk.
    #[serde(rename = "progress.bar.background")]
    pub progress_bar: Option<SharedString>,
    /// İçin kullanılır odak ring.
    #[serde(rename = "ring")]
    pub ring: Option<SharedString>,
    /// KaydirmaCubugu arka plan renk.
    #[serde(rename = "scrollbar.background")]
    pub scrollbar: Option<SharedString>,
    /// KaydirmaCubugu tutamak arka plan renk.
    #[serde(rename = "scrollbar.thumb.background")]
    pub scrollbar_thumb: Option<SharedString>,
    /// KaydirmaCubugu tutamak üzerine gelme arka plan renk.
    #[serde(rename = "scrollbar.thumb.hover.background")]
    pub scrollbar_thumb_hover: Option<SharedString>,
    /// Secondary arka plan renk.
    #[serde(rename = "secondary.background")]
    pub secondary: Option<SharedString>,
    /// Active ikincil arka plan renk.
    #[serde(rename = "secondary.active.background")]
    pub secondary_active: Option<SharedString>,
    /// Secondary metin renk, için kullanılır ikincil Dugme metin renk veya ikincil metin.
    #[serde(rename = "secondary.foreground")]
    pub secondary_foreground: Option<SharedString>,
    /// Hover ikincil arka plan renk.
    #[serde(rename = "secondary.hover.background")]
    pub secondary_hover: Option<SharedString>,
    /// girdi seçim arka plan renk.
    #[serde(rename = "selection.background")]
    pub selection: Option<SharedString>,
    /// YanCubuk arka plan renk.
    #[serde(rename = "sidebar.background")]
    pub sidebar: Option<SharedString>,
    /// YanCubuk accent arka plan renk.
    #[serde(rename = "sidebar.accent.background")]
    pub sidebar_accent: Option<SharedString>,
    /// YanCubuk accent metin renk.
    #[serde(rename = "sidebar.accent.foreground")]
    pub sidebar_accent_foreground: Option<SharedString>,
    /// YanCubuk kenarlık renk.
    #[serde(rename = "sidebar.border")]
    pub sidebar_border: Option<SharedString>,
    /// YanCubuk metin renk.
    #[serde(rename = "sidebar.foreground")]
    pub sidebar_foreground: Option<SharedString>,
    /// YanCubuk birincil arka plan renk.
    #[serde(rename = "sidebar.primary.background")]
    pub sidebar_primary: Option<SharedString>,
    /// YanCubuk birincil metin renk.
    #[serde(rename = "sidebar.primary.foreground")]
    pub sidebar_primary_foreground: Option<SharedString>,
    /// Iskelet arka plan renk.
    #[serde(rename = "skeleton.background")]
    pub skeleton: Option<SharedString>,
    /// Kaydirici çubuk arka plan renk.
    #[serde(rename = "slider.background")]
    pub slider_bar: Option<SharedString>,
    /// Kaydirici tutamak arka plan renk.
    #[serde(rename = "slider.thumb.background")]
    pub slider_thumb: Option<SharedString>,
    /// Success arka plan renk.
    #[serde(rename = "success.background")]
    pub success: Option<SharedString>,
    /// Success metin renk.
    #[serde(rename = "success.foreground")]
    pub success_foreground: Option<SharedString>,
    /// Success üzerine gelme arka plan renk.
    #[serde(rename = "success.hover.background")]
    pub success_hover: Option<SharedString>,
    /// Success etkin arka plan renk.
    #[serde(rename = "success.active.background")]
    pub success_active: Option<SharedString>,
    /// Anahtar arka plan renk.
    #[serde(rename = "switch.background")]
    pub switch: Option<SharedString>,
    /// Anahtar tutamak arka plan renk.
    #[serde(rename = "switch.thumb.background")]
    pub switch_thumb: Option<SharedString>,
    /// Sekme arka plan renk.
    #[serde(rename = "tab.background")]
    pub tab: Option<SharedString>,
    /// Sekme etkin arka plan renk.
    #[serde(rename = "tab.active.background")]
    pub tab_active: Option<SharedString>,
    /// Sekme etkin metin renk.
    #[serde(rename = "tab.active.foreground")]
    pub tab_active_foreground: Option<SharedString>,
    /// SekmeCubugu arka plan renk.
    #[serde(rename = "tab_bar.background")]
    pub tab_bar: Option<SharedString>,
    /// SekmeCubugu segmented arka plan renk.
    #[serde(rename = "tab_bar.segmented.background")]
    pub tab_bar_segmented: Option<SharedString>,
    /// Sekme metin renk.
    #[serde(rename = "tab.foreground")]
    pub tab_foreground: Option<SharedString>,
    /// Tablo arka plan renk.
    #[serde(rename = "table.background")]
    pub table: Option<SharedString>,
    /// Tablo etkin öğe arka plan renk.
    #[serde(rename = "table.active.background")]
    pub table_active: Option<SharedString>,
    /// Tablo etkin öğe kenarlık renk.
    #[serde(rename = "table.active.border")]
    pub table_active_border: Option<SharedString>,
    /// Stripe arka plan renk için even TabloSatiri.
    #[serde(rename = "table.even.background")]
    pub table_even: Option<SharedString>,
    /// Tablo başlık arka plan renk.
    #[serde(rename = "table.head.background")]
    pub table_head: Option<SharedString>,
    /// Tablo başlık metin renk.
    #[serde(rename = "table.head.foreground")]
    pub table_head_foreground: Option<SharedString>,
    /// Tablo alt bilgi arka plan renk.
    #[serde(rename = "table.foot.background")]
    pub table_foot: Option<SharedString>,
    /// Tablo alt bilgi metin renk.
    #[serde(rename = "table.foot.foreground")]
    pub table_foot_foreground: Option<SharedString>,
    /// Tablo öğe üzerine gelme arka plan renk.
    #[serde(rename = "table.hover.background")]
    pub table_hover: Option<SharedString>,
    /// Tablo satır kenarlık renk.
    #[serde(rename = "table.row.border")]
    pub table_row_border: Option<SharedString>,
    /// BaslikCubugu arka plan renk, kullanmak için Pencere başlık çubuk.
    #[serde(rename = "title_bar.background")]
    pub title_bar: Option<SharedString>,
    /// BaslikCubugu kenarlık renk.
    #[serde(rename = "title_bar.border")]
    pub title_bar_border: Option<SharedString>,
    /// arka plan renk için Tiles.
    #[serde(rename = "tiles.background")]
    pub tiles: Option<SharedString>,
    /// Warning arka plan renk.
    #[serde(rename = "warning.background")]
    pub warning: Option<SharedString>,
    /// Warning etkin arka plan renk.
    #[serde(rename = "warning.active.background")]
    pub warning_active: Option<SharedString>,
    /// Warning üzerine gelme arka plan renk.
    #[serde(rename = "warning.hover.background")]
    pub warning_hover: Option<SharedString>,
    /// Warning ön plan renk.
    #[serde(rename = "warning.foreground")]
    pub warning_foreground: Option<SharedString>,
    /// Overlay arka plan renk.
    #[serde(rename = "overlay")]
    pub overlay: Option<SharedString>,
    /// Pencere kenarlık renk.
    ///
    /// # Platform özel:
    ///
    /// Bu yalnızca çalışır üzerinde Linux, diğer platforms biz olabilir't değişir pencere kenarlık renk.
    #[serde(rename = "window.border")]
    pub window_border: Option<SharedString>,

    /// Taban blue renk.
    #[serde(rename = "base.blue")]
    blue: Option<String>,
    /// Taban açık blue renk.
    #[serde(rename = "base.blue.light")]
    blue_light: Option<String>,
    /// Taban cyan renk.
    #[serde(rename = "base.cyan")]
    cyan: Option<String>,
    /// Taban açık cyan renk.
    #[serde(rename = "base.cyan.light")]
    cyan_light: Option<String>,
    /// Taban green renk.
    #[serde(rename = "base.green")]
    green: Option<String>,
    /// Taban açık green renk.
    #[serde(rename = "base.green.light")]
    green_light: Option<String>,
    /// Taban magenta renk.
    #[serde(rename = "base.magenta")]
    magenta: Option<String>,
    #[serde(rename = "base.magenta.light")]
    magenta_light: Option<String>,
    /// Taban red renk.
    #[serde(rename = "base.red")]
    red: Option<String>,
    /// Taban açık red renk.
    #[serde(rename = "base.red.light")]
    red_light: Option<String>,
    /// Taban yellow renk.
    #[serde(rename = "base.yellow")]
    yellow: Option<String>,
    /// Taban açık yellow renk.
    #[serde(rename = "base.yellow.light")]
    yellow_light: Option<String>,
}

impl TemaRengi {
    /// Yeni bir `TemaRengi` bir `TemaYapilandirmasi` oluşturur.
    pub(crate) fn apply_config(&mut self, config: &TemaYapilandirmasi, default_theme: &TemaRengi) {
        let colors = config.colors.clone();

        macro_rules! apply_color {
            ($config_field:ident) => {
                if let Some(value) = colors.$config_field {
                    if let Ok(color) = try_parse_color(&value) {
                        self.$config_field = color;
                    } else {
                        self.$config_field = default_theme.$config_field;
                    }
                } else {
                    self.$config_field = default_theme.$config_field;
                }
            };
            // With fallback
            ($config_field:ident, fallback = $fallback:expr) => {
                if let Some(value) = colors.$config_field {
                    if let Ok(color) = try_parse_color(&value) {
                        self.$config_field = color;
                    }
                } else {
                    self.$config_field = $fallback;
                }
            };
        }

        apply_color!(background);

        // Base colors for fallback
        apply_color!(red);
        apply_color!(
            red_light,
            fallback = self.background.blend(self.red.opacity(0.8))
        );
        apply_color!(green);
        apply_color!(
            green_light,
            fallback = self.background.blend(self.green.opacity(0.8))
        );
        apply_color!(blue);
        apply_color!(
            blue_light,
            fallback = self.background.blend(self.blue.opacity(0.8))
        );
        apply_color!(magenta);
        apply_color!(
            magenta_light,
            fallback = self.background.blend(self.magenta.opacity(0.8))
        );
        apply_color!(yellow);
        apply_color!(
            yellow_light,
            fallback = self.background.blend(self.yellow.opacity(0.8))
        );
        apply_color!(cyan);
        apply_color!(
            cyan_light,
            fallback = self.background.blend(self.cyan.opacity(0.8))
        );

        apply_color!(border);
        apply_color!(foreground);
        apply_color!(muted);
        apply_color!(
            muted_foreground,
            fallback = self.muted.blend(self.foreground.opacity(0.7))
        );

        // Dugme colors
        let active_darken = if config.mode.is_dark() { 0.2 } else { 0.1 };
        let hover_opacity = 0.9;
        apply_color!(primary);
        apply_color!(primary_foreground, fallback = self.foreground);
        apply_color!(
            primary_hover,
            fallback = self.background.blend(self.primary.opacity(hover_opacity))
        );
        apply_color!(
            primary_active,
            fallback = self.primary.darken(active_darken)
        );
        apply_color!(button_primary, fallback = self.primary);
        apply_color!(
            button_primary_foreground,
            fallback = self.primary_foreground
        );
        apply_color!(button_primary_hover, fallback = self.primary_hover);
        apply_color!(button_primary_active, fallback = self.primary_active);
        apply_color!(secondary);
        apply_color!(secondary_foreground, fallback = self.foreground);
        apply_color!(
            secondary_hover,
            fallback = self.background.blend(self.secondary.opacity(hover_opacity))
        );
        apply_color!(
            secondary_active,
            fallback = self.secondary.darken(active_darken)
        );
        apply_color!(success, fallback = self.green);
        apply_color!(success_foreground, fallback = self.primary_foreground);
        apply_color!(
            success_hover,
            fallback = self.background.blend(self.success.opacity(hover_opacity))
        );
        apply_color!(
            success_active,
            fallback = self.success.darken(active_darken)
        );
        apply_color!(info, fallback = self.cyan);
        apply_color!(info_foreground, fallback = self.primary_foreground);
        apply_color!(
            info_hover,
            fallback = self.background.blend(self.info.opacity(hover_opacity))
        );
        apply_color!(info_active, fallback = self.info.darken(active_darken));
        apply_color!(warning, fallback = self.yellow);
        apply_color!(warning_foreground, fallback = self.primary_foreground);
        apply_color!(
            warning_hover,
            fallback = self.background.blend(self.warning.opacity(0.9))
        );
        apply_color!(
            warning_active,
            fallback = self.background.blend(self.warning.darken(active_darken))
        );

        // Other colors
        apply_color!(accent, fallback = self.secondary);
        apply_color!(accent_foreground, fallback = self.foreground);
        apply_color!(accordion, fallback = self.background);
        apply_color!(accordion_hover, fallback = self.accent.opacity(0.8));
        apply_color!(
            group_box,
            fallback = self
                .background
                .blend(
                    self.secondary
                        .opacity(if config.mode.is_dark() { 0.3 } else { 0.4 })
                )
        );
        apply_color!(group_box_foreground, fallback = self.foreground);
        apply_color!(caret, fallback = self.primary);
        apply_color!(chart_1, fallback = self.blue.lighten(0.4));
        apply_color!(chart_2, fallback = self.blue.lighten(0.2));
        apply_color!(chart_3, fallback = self.blue);
        apply_color!(chart_4, fallback = self.blue.darken(0.2));
        apply_color!(chart_5, fallback = self.blue.darken(0.4));
        apply_color!(chart_bullish, fallback = self.green);
        apply_color!(chart_bearish, fallback = self.red);
        apply_color!(danger, fallback = self.red);
        apply_color!(danger_active, fallback = self.danger.darken(active_darken));
        apply_color!(danger_foreground, fallback = self.primary_foreground);
        apply_color!(
            danger_hover,
            fallback = self.background.blend(self.danger.opacity(0.9))
        );
        apply_color!(
            description_list_label,
            fallback = self.background.blend(self.border.opacity(0.2))
        );
        apply_color!(
            description_list_label_foreground,
            fallback = self.muted_foreground
        );
        apply_color!(drag_border, fallback = self.primary.opacity(0.65));
        apply_color!(drop_target, fallback = self.primary.opacity(0.2));
        apply_color!(input, fallback = self.border);
        apply_color!(link, fallback = self.primary);
        apply_color!(link_active, fallback = self.link);
        apply_color!(link_hover, fallback = self.link);
        apply_color!(list, fallback = self.background);
        apply_color!(
            list_active,
            fallback = self.background.blend(self.primary.opacity(0.1))
        );
        apply_color!(
            list_active_border,
            fallback = self.background.blend(self.primary.opacity(0.6))
        );
        apply_color!(list_even, fallback = self.list);
        apply_color!(list_head, fallback = self.list);
        apply_color!(list_hover, fallback = self.accent.opacity(0.6));
        apply_color!(popover, fallback = self.background);
        apply_color!(popover_foreground, fallback = self.foreground);
        apply_color!(progress_bar, fallback = self.primary);
        apply_color!(ring, fallback = self.blue);
        apply_color!(scrollbar, fallback = self.background);
        apply_color!(scrollbar_thumb, fallback = self.accent);
        apply_color!(scrollbar_thumb_hover, fallback = self.scrollbar_thumb);
        apply_color!(selection, fallback = self.primary);
        apply_color!(
            sidebar,
            fallback = self.background.blend(self.border.opacity(0.15))
        );
        apply_color!(sidebar_accent, fallback = self.accent);
        apply_color!(sidebar_accent_foreground, fallback = self.accent_foreground);
        apply_color!(sidebar_border, fallback = self.border);
        apply_color!(sidebar_foreground, fallback = self.foreground);
        apply_color!(sidebar_primary, fallback = self.primary);
        apply_color!(
            sidebar_primary_foreground,
            fallback = self.primary_foreground
        );
        apply_color!(skeleton, fallback = self.secondary);
        apply_color!(slider_bar, fallback = self.primary);
        apply_color!(slider_thumb, fallback = self.primary_foreground);
        apply_color!(switch, fallback = self.secondary_active);
        apply_color!(switch_thumb, fallback = self.background);
        apply_color!(tab, fallback = self.background);
        apply_color!(tab_active, fallback = self.background);
        apply_color!(tab_active_foreground, fallback = self.foreground);
        apply_color!(tab_bar, fallback = self.background);
        apply_color!(tab_bar_segmented, fallback = self.secondary);
        apply_color!(tab_foreground, fallback = self.foreground);
        apply_color!(table, fallback = self.list);
        apply_color!(table_active, fallback = self.list_active);
        apply_color!(table_active_border, fallback = self.list_active_border);
        apply_color!(table_even, fallback = self.list_even);
        apply_color!(table_head, fallback = self.list_head);
        apply_color!(table_head_foreground, fallback = self.muted_foreground);
        apply_color!(table_foot, fallback = self.list_head);
        apply_color!(table_foot_foreground, fallback = self.muted_foreground);
        apply_color!(table_hover, fallback = self.list_hover);
        apply_color!(table_row_border, fallback = self.border);
        apply_color!(title_bar, fallback = self.background);
        apply_color!(title_bar_border, fallback = self.border);
        apply_color!(tiles, fallback = self.background);
        apply_color!(overlay);
        apply_color!(window_border, fallback = self.border);

        // TODO: Apply default fallback colors to highlight.

        // Ensure opacity for list_active, table_active
        self.list_active = self.list_active.alpha(self.list_active.a.min(0.2));
        self.table_active = self.table_active.alpha(self.table_active.a.min(0.2));
        self.selection = self.selection.alpha(self.selection.a.min(0.3));
    }
}

impl Tema {
    /// verilen tema yapılandırma için geçerli tema. uygular.
    pub fn apply_config(&mut self, config: &Rc<TemaYapilandirmasi>) {
        if config.mode.is_dark() {
            self.dark_theme = config.clone();
        } else {
            self.light_theme = config.clone();
        }
        if let Some(style) = &config.highlight {
            let highlight_theme = Arc::new(VurguTemasi {
                name: config.name.to_string(),
                appearance: config.mode,
                style: style.clone(),
            });
            self.highlight_theme = highlight_theme.clone();
        }

        let default_colors = if config.mode.is_dark() {
            TemaRengi::dark()
        } else {
            TemaRengi::light()
        };

        if let Some(font_size) = config.font_size {
            self.font_size = px(font_size);
        }
        if let Some(font_family) = &config.font_family {
            self.font_family = font_family.clone();
        }
        if let Some(mono_font_family) = &config.mono_font_family {
            self.mono_font_family = mono_font_family.clone();
        }
        if let Some(mono_font_size) = config.mono_font_size {
            self.mono_font_size = px(mono_font_size);
        }
        if let Some(radius) = config.radius {
            self.radius = px(radius as f32);
        }
        if let Some(radius_lg) = config.radius_lg {
            self.radius_lg = px(radius_lg as f32);
        }
        if let Some(shadow) = config.shadow {
            self.shadow = shadow;
        }

        self.colors.apply_config(&config, &default_colors);
        self.mode = config.mode;
    }
}
