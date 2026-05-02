/// Tema renk anahtarlarını eşlemek için uyumluluk köprüsü.
///
/// Bu modül eski snake_case anahtarlar ile
/// `TemaRengi` içinde kullanılan değerler ve
/// renk tema görüntüleyicisinin beklediği mantıksal kategori/adlar arasında çeviri sağlar.
///
/// ### Bu mapper nasıl kaldırılır
///
/// Proje tema şemasını proje genelinde nokta gösterimiyle birleştirmeye karar verirse,
/// bu geçici köprüyü kaldırmak için şu adımları izleyin:
///
/// 1. **`TemaRengi` değerini güncelleyin**:
/// `crates/ui/src/theme/theme_color.rs` içinde `#[serde(rename = "...")]` nitelikleri ekleyin
/// ve `TemaRengi` structındaki tüm alanları kanonik nokta gösterimiyle eşleştirin.
/// Örnek: `accent_foreground` -> `#[serde(rename = "accent.foreground")]`.
///
/// 2. **JSON temalarını güncelleyin**:
/// `crates/ui/src/theme/varsayılan-theme.json` ve `themes/` içindeki dosyaların
/// yalnızca nokta gösterimli anahtarlar kullandığından emin olun.
///
/// 3. **Görüntüleyiciyi refactor edin**:
/// `crates/story/src/stories/theme_story/color_theme_story.rs` içinde
/// `super::mapper::parse_theme_key` çağrılarını kaldırıp `.` karakterine göre basit bölme ile değiştirin.
///
/// 4. **Bu dosyayı silin**:
/// `mapper.rs` dosyasını ve `mod.rs` içindeki modül bildirimini kaldırın.
///
/// Ayrıştırılmış tema anahtarını kategori, gösterim adı ve kanonik nokta gösterimli anahtarla temsil eder.
pub struct ParsedKey {
    pub category: String,
    pub name: String,
    pub canonical_key: String,
}

/// Tema anahtarını (snake_case veya nokta gösterimi) mantıksal kategori ve ada ayrıştırır.
pub fn parse_theme_key(key: &str) -> ParsedKey {
    // 1. Check for dot-notation (e.g., "accent.background")
    if key.contains('.') {
        let parts: Vec<&str> = key.splitn(2, '.').collect();
        return ParsedKey {
            category: to_title_case_full(parts[0]),
            name: to_title_case_full(parts[1]),
            canonical_key: key.to_string(),
        };
    }

    // 2. Handle legacy snake_case remapping (e.g., "accent_foreground" -> "Accent" / "Foreground")
    // This list attempts to reconstruct the hierarchy from the flat TemaRengi struct.
    let (category, name, canonical) = match key {
        // Accent
        "accent" => ("Accent", "Background", "accent.background"),
        "accent_foreground" => ("Accent", "Foreground", "accent.foreground"),

        // Primary
        "primary" => ("Primary", "Background", "primary.background"),
        "primary_active" => ("Primary", "Active Background", "primary.active.background"),
        "primary_foreground" => ("Primary", "Foreground", "primary.foreground"),
        "primary_hover" => ("Primary", "Hover Background", "primary.hover.background"),

        // Secondary
        "secondary" => ("Secondary", "Background", "secondary.background"),
        "secondary_active" => (
            "Secondary",
            "Active Background",
            "secondary.active.background",
        ),
        "secondary_foreground" => ("Secondary", "Foreground", "secondary.foreground"),
        "secondary_hover" => (
            "Secondary",
            "Hover Background",
            "secondary.hover.background",
        ),

        // YanCubuk
        "sidebar" => ("YanCubuk", "Background", "sidebar.background"),
        "sidebar_accent" => ("YanCubuk", "Accent Background", "sidebar.accent.background"),
        "sidebar_accent_foreground" => {
            ("YanCubuk", "Accent Foreground", "sidebar.accent.foreground")
        }
        "sidebar_border" => ("YanCubuk", "Border", "sidebar.border"),
        "sidebar_foreground" => ("YanCubuk", "Foreground", "sidebar.foreground"),
        "sidebar_primary" => (
            "YanCubuk",
            "Primary Background",
            "sidebar.primary.background",
        ),
        "sidebar_primary_foreground" => (
            "YanCubuk",
            "Primary Foreground",
            "sidebar.primary.foreground",
        ),

        // Liste
        "list" => ("Liste", "Background", "list.background"),
        "list_active" => ("Liste", "Active Background", "list.active.background"),
        "list_active_border" => ("Liste", "Active Border", "list.active.border"),
        "list_even" => ("Liste", "Even Background", "list.even.background"),
        "list_head" => ("Liste", "Head Background", "list.head.background"),
        "list_hover" => ("Liste", "Hover Background", "list.hover.background"),

        // Tablo
        "table" => ("Tablo", "Background", "table.background"),
        "table_active" => ("Tablo", "Active Background", "table.active.background"),
        "table_active_border" => ("Tablo", "Active Border", "table.active.border"),
        "table_even" => ("Tablo", "Even Background", "table.even.background"),
        "table_head" => ("Tablo", "Head Background", "table.head.background"),
        "table_head_foreground" => ("Tablo", "Head Foreground", "table.head.foreground"),
        "table_hover" => ("Tablo", "Hover Background", "table.hover.background"),
        "table_row_border" => ("Tablo", "Row Border", "table.row.border"),

        // Tabs
        "tab" => ("Sekme", "Background", "tab.background"),
        "tab_active" => ("Sekme", "Active Background", "tab.active.background"),
        "tab_active_foreground" => ("Sekme", "Active Foreground", "tab.active.foreground"),
        "tab_bar" => ("Sekme Bar", "Background", "tab_bar.background"),
        "tab_bar_segmented" => (
            "Sekme Bar",
            "Segmented Background",
            "tab_bar.segmented.background",
        ),
        "tab_foreground" => ("Sekme", "Foreground", "tab.foreground"),

        // Input
        "input" => ("Input", "Border", "input.border"),
        "caret" => ("Input", "Caret", "caret"),
        "selection" => ("Input", "Selection", "selection.background"),

        // Kaydirici / Anahtar
        "slider_bar" => ("Kaydirici", "Bar", "slider.background"),
        "slider_thumb" => ("Kaydirici", "Thumb", "slider.thumb.background"),
        "switch" => ("Anahtar", "Background", "switch.background"),
        "switch_thumb" => ("Anahtar", "Thumb", "switch.thumb.background"),

        // Muted / Iskelet
        "muted" => ("Muted", "Background", "muted.background"),
        "muted_foreground" => ("Muted", "Foreground", "muted.foreground"),
        "skeleton" => ("Iskelet", "Background", "skeleton.background"),

        // Charts
        "chart_1" => ("Chart", "Color 1", "chart.1"),
        "chart_2" => ("Chart", "Color 2", "chart.2"),
        "chart_3" => ("Chart", "Color 3", "chart.3"),
        "chart_4" => ("Chart", "Color 4", "chart.4"),
        "chart_5" => ("Chart", "Color 5", "chart.5"),

        // Danger / Success / Warning / Info
        "danger" => ("Danger", "Background", "danger.background"),
        "danger_active" => ("Danger", "Active", "danger.active.background"),
        "danger_foreground" => ("Danger", "Foreground", "danger.foreground"),
        "danger_hover" => ("Danger", "Hover", "danger.hover.background"),

        "success" => ("Success", "Background", "success.background"),
        "success_active" => ("Success", "Active", "success.active.background"),
        "success_foreground" => ("Success", "Foreground", "success.foreground"),
        "success_hover" => ("Success", "Hover", "success.hover.background"),

        "warning" => ("Warning", "Background", "warning.background"),
        "warning_active" => ("Warning", "Active", "warning.active.background"),
        "warning_foreground" => ("Warning", "Foreground", "warning.foreground"),
        "warning_hover" => ("Warning", "Hover", "warning.hover.background"),

        "info" => ("Info", "Background", "info.background"),
        "info_active" => ("Info", "Active", "info.active.background"),
        "info_foreground" => ("Info", "Foreground", "info.foreground"),
        "info_hover" => ("Info", "Hover", "info.hover.background"),

        // Base Colors
        "red" => ("Base", "Red", "base.red"),
        "red_light" => ("Base", "Red Light", "base.red.light"),
        "green" => ("Base", "Green", "base.green"),
        "green_light" => ("Base", "Green Light", "base.green.light"),
        "blue" => ("Base", "Blue", "base.blue"),
        "blue_light" => ("Base", "Blue Light", "base.blue.light"),
        "yellow" => ("Base", "Yellow", "base.yellow"),
        "yellow_light" => ("Base", "Yellow Light", "base.yellow.light"),
        "magenta" => ("Base", "Magenta", "base.magenta"),
        "magenta_light" => ("Base", "Magenta Light", "base.magenta.light"),
        "cyan" => ("Base", "Cyan", "base.cyan"),
        "cyan_light" => ("Base", "Cyan Light", "base.cyan.light"),

        // Everything else remains in Global or attempts a split
        _ => {
            if key.contains('_') {
                let parts: Vec<&str> = key.splitn(2, '_').collect();
                (parts[0], parts[1], key)
            } else {
                ("Global", key, key)
            }
        }
    };

    ParsedKey {
        category: translate_display_label(&to_title_case_full(category)),
        name: translate_display_label(&to_title_case_full(name)),
        canonical_key: canonical.to_string(),
    }
}

fn to_title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_title_case_full(s: &str) -> String {
    s.split(|c| c == '_' || c == '.')
        .map(to_title_case)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_display_label(label: &str) -> String {
    match label {
        "Accent" => "Vurgu",
        "Active" => "Etkin",
        "Active Background" => "Etkin Arka Plan",
        "Active Border" => "Etkin Kenarlık",
        "Active Foreground" => "Etkin Ön Plan",
        "Background" => "Arka Plan",
        "Bar" => "Çubuk",
        "Base" => "Temel",
        "Bid Ask Ratio" => "Alış Satış Oranı",
        "Blue" => "Mavi",
        "Blue Light" => "Açık Mavi",
        "Border" => "Kenarlık",
        "Caret" => "İmleç",
        "Chart" => "Grafik",
        "Color 1" => "Renk 1",
        "Color 2" => "Renk 2",
        "Color 3" => "Renk 3",
        "Color 4" => "Renk 4",
        "Color 5" => "Renk 5",
        "Cyan" => "Camgöbeği",
        "Cyan Light" => "Açık Camgöbeği",
        "Danger" => "Tehlike",
        "Even Background" => "Çift Satır Arka Planı",
        "Foreground" => "Ön Plan",
        "Global" => "Genel",
        "Green" => "Yeşil",
        "Green Light" => "Açık Yeşil",
        "Head Background" => "Başlık Arka Planı",
        "Head Foreground" => "Başlık Ön Planı",
        "Hover" => "Üzerine Gelme",
        "Hover Background" => "Üzerine Gelme Arka Planı",
        "Info" => "Bilgi",
        "Input" => "Girdi",
        "Liste" => "Liste",
        "Magenta" => "Macenta",
        "Magenta Light" => "Açık Macenta",
        "Muted" => "Sönük",
        "Primary" => "Birincil",
        "Primary Background" => "Birincil Arka Plan",
        "Primary Foreground" => "Birincil Ön Plan",
        "Red" => "Kırmızı",
        "Red Light" => "Açık Kırmızı",
        "Row Border" => "Satır Kenarlığı",
        "Secondary" => "İkincil",
        "Segmented Background" => "Segmentli Arka Plan",
        "Selection" => "Seçim",
        "YanCubuk" => "Kenar Çubuğu",
        "Iskelet" => "İskelet",
        "Kaydirici" => "Kaydırıcı",
        "Structure" => "Yapı",
        "Success" => "Başarı",
        "Anahtar" => "Anahtar",
        "Sekme" => "Sekme",
        "Sekme Bar" => "Sekme Çubuğu",
        "Tablo" => "Tablo",
        "Thumb" => "Tutamak",
        "Warning" => "Uyarı",
        "Yellow" => "Sarı",
        "Yellow Light" => "Açık Sarı",
        _ => label,
    }
    .to_string()
}
