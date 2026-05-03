use gpui::{
    Action, AnyElement, AnyView, App, AppContext, Bounds, Context, Div, Entity, EventEmitter,
    FocusHandle, Focusable, Global, Hsla, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, Pixels, Render, RenderOnce, SharedString, Size, StyleRefinement, Styled, Window,
    WindowBounds, WindowKind, WindowOptions, actions, div, prelude::FluentBuilder as _, px, rems,
    size,
};
use kavis_ui::{
    BaslikCubugu, EtkinTema, KokGorunum, PencereUzantisi, SimgeAdi,
    button::Dugme,
    dock::{Panel, PanelControl, PanelEvent, PanelInfo, PanelState, TitleStyle, register_panel},
    group_box::{GrupKutusu, GrupKutusuVaryantlari as _},
    h_flex,
    menu::PopupMenu,
    notification::Bildirim,
    scroll::{KaydirilabilirOge as _, KaydirmaCubuguGosterimi},
    text::markdown,
    v_flex,
};
use serde::{Deserialize, Serialize};

mod app_menus;
mod embedded_themes;
mod gallery;
mod stories;
mod themes;
mod title_bar;
pub use crate::title_bar::AppTitleBar;
pub use gallery::Gallery;
pub use stories::*;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectScrollbarShow(KaydirmaCubuguGosterimi);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectLocale(SharedString);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectFont(usize);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = story, no_json)]
pub struct SelectRadius(usize);

actions!(
    story,
    [
        About,
        Open,
        Quit,
        ToggleSearch,
        TestAction,
        Sekme,
        TabPrev,
        ShowPanelInfo,
        ToggleListActiveHighlight
    ]
);

const PANEL_NAME: &str = "StoryContainer";

pub struct AppState {
    pub invisible_panels: Entity<Vec<SharedString>>,
}
impl AppState {
    fn init(cx: &mut App) {
        let state = Self {
            invisible_panels: cx.new(|_| Vec::new()),
        };
        cx.set_global::<AppState>(state);
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

pub fn create_new_window<F, E>(title: &str, crate_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    create_new_window_with_size(title, None, crate_view_fn, cx);
}

pub fn create_new_window_with_size<F, E>(
    title: &str,
    window_size: Option<Size<Pixels>>,
    crate_view_fn: F,
    cx: &mut App,
) where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = window_size.unwrap_or(size(px(1600.0), px(1200.0)));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(BaslikCubugu::title_bar_options()),
            window_min_size: Some(gpui::Size {
                width: px(480.),
                height: px(320.),
            }),
            kind: WindowKind::Normal,
            #[cfg(target_os = "linux")]
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(gpui::WindowDecorations::Client),
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = crate_view_fn(window, cx);
                let story_root = cx.new(|cx| StoryRoot::new(title.clone(), view, window, cx));

                // Set focus to the StoryRoot to enable it's actions.
                let focus_handle = story_root.focus_handle(cx);
                window.defer(cx, move |window, cx| {
                    if window.focused(cx).is_none() {
                        focus_handle.focus(window, cx);
                    }
                });

                cx.new(|cx| KokGorunum::new(story_root, window, cx))
            })
            .expect("pencere açılamadı");

        window.update(cx, |_, window, _| {
            window.activate_window();
            window.set_window_title(&title);
        })?;

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}

impl Global for AppState {}

pub fn init(cx: &mut App) {
    // Try to initialize tracing subscriber, but ignore if already initialized
    #[cfg(not(target_family = "wasm"))]
    {
        use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("kavis_ui=trace".parse().unwrap()),
            )
            .try_init();
    }

    // For WASM, use a subscriber without time support
    #[cfg(target_family = "wasm")]
    {
        use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().without_time())
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("kavis_ui=trace".parse().unwrap()),
            )
            .try_init();
    }

    kavis_ui::init(cx);
    AppState::init(cx);
    themes::init(cx);
    stories::init(cx);

    #[cfg(not(target_family = "wasm"))]
    {
        let http_client = reqwest_client::ReqwestClient::user_agent("kavis-ui/story").unwrap();
        cx.set_http_client(std::sync::Arc::new(http_client));
    }

    #[cfg(target_family = "wasm")]
    {
        // Safety: the web examples run single-threaded; the client is
        // created and used exclusively on the main thread.
        let http_client = unsafe {
            gpui_web::FetchHttpClient::with_user_agent("kavis-ui/story")
                .expect("FetchHttpClient oluşturulamadı")
        };
        cx.set_http_client(std::sync::Arc::new(http_client));
    }

    cx.bind_keys([
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-o", Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", Open, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);

    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    cx.on_action(|_: &About, cx: &mut App| {
        if let Some(window) = cx.active_window().and_then(|w| w.downcast::<KokGorunum>()) {
            cx.defer(move |cx| {
                window
                    .update(cx, |_, window, cx| {
                        window.defer(cx, |window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert.title("Hakkında").description(markdown(
                                    "Kavis UI Storybook\n\n\
                                    Version 0.1.0\n\n\
                                    https://hakantr.github.io/kavis-ui",
                                ))
                            });
                        });
                    })
                    .unwrap();
            });
        }
    });

    register_panel(cx, PANEL_NAME, |_, _, info, window, cx| {
        let story_state = match info {
            PanelInfo::Panel(value) => StoryState::from_value(value.clone()),
            _ => {
                unreachable!("Invalid PanelInfo: {:?}", info)
            }
        };

        let view = cx.new(|cx| {
            let (title, description, closable, zoomable, story, on_active) =
                story_state.to_story(window, cx);
            let mut container = StoryContainer::new(window, cx)
                .story(story, story_state.story_klass)
                .on_active(on_active);

            cx.on_focus_in(
                &container.focus_handle,
                window,
                |this: &mut StoryContainer, _, _| {
                    println!("Story kapsayıcısı odağa girdi: {}", this.name);
                },
            )
            .detach();

            container.name = localized_story_title(title).into();
            container.navigation_name = localized_story_navigation_title(title);
            container.description = localized_story_description(description).into();
            container.closable = closable;
            container.zoomable = zoomable;
            container
        });
        Box::new(view)
    });

    cx.activate(true);
}

#[derive(IntoElement)]
struct StorySection {
    base: Div,
    title: SharedString,
    sub_title: Vec<AnyElement>,
    children: Vec<AnyElement>,
}

impl StorySection {
    pub fn sub_title(mut self, sub_title: impl IntoElement) -> Self {
        self.sub_title.push(sub_title.into_any_element());
        self
    }

    #[allow(unused)]
    fn max_w_md(mut self) -> Self {
        self.base = self.base.max_w(rems(48.));
        self
    }

    #[allow(unused)]
    fn max_w_lg(mut self) -> Self {
        self.base = self.base.max_w(rems(64.));
        self
    }

    #[allow(unused)]
    fn max_w_xl(mut self) -> Self {
        self.base = self.base.max_w(rems(80.));
        self
    }

    #[allow(unused)]
    fn max_w_2xl(mut self) -> Self {
        self.base = self.base.max_w(rems(96.));
        self
    }
}

impl ParentElement for StorySection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for StorySection {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for StorySection {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        GrupKutusu::new()
            .id(self.title.clone())
            .outline()
            .title(
                h_flex()
                    .justify_between()
                    .w_full()
                    .gap_4()
                    .child(self.title)
                    .children(self.sub_title),
            )
            .content_style(
                StyleRefinement::default()
                    .rounded(cx.theme().radius_lg)
                    .overflow_x_hidden()
                    .items_center()
                    .justify_center(),
            )
            .child(self.base.children(self.children))
    }
}

pub(crate) fn section(title: impl Into<SharedString>) -> StorySection {
    let title = title.into();

    StorySection {
        title: localized_section_title(title.as_str()),
        sub_title: vec![],
        base: h_flex()
            .w_full()
            .flex_wrap()
            .justify_center()
            .items_center()
            .gap_4(),
        children: vec![],
    }
}

fn localized_section_title(title: &str) -> SharedString {
    match title {
        "UyariIletisimKutusu" => "Uyarı Penceresi",
        "Alignment" => "Hizalama",
        "Appearance false with Input" => "Görünüm kapalı, girdi ile",
        "Auto Grow" => "Otomatik Büyüme",
        "Auto Grow with No Wrap" => "Sarmasız Otomatik Büyüme",
        "Avatar Group" => "Avatar Grubu",
        "Avatar with image" => "Görselli Avatar",
        "Avatar with text" => "Metinli Avatar",
        "Rozet on icon" => "Simge üzerinde rozet",
        "Rozet with color" => "Renkli rozet",
        "Rozet with count" => "Sayaçlı rozet",
        "Rozet with dot" => "Noktalı rozet",
        "Rozet with icon" => "Simgeli rozet",
        "Banner" => "Banner",
        "Basic" => "Temel",
        "Basic GezintiYolu" => "Temel Konum Yolu",
        "Basic IletisimKutusu" => "Temel İletişim Kutusu",
        "Basic AcilirKatman" => "Temel Açılır İçerik",
        "Basic Puanlama" => "Temel Puanlama",
        "Dugme Group" => "Düğme Grubu",
        "Dugme Group (Vertical)" => "Düğme Grubu (Dikey)",
        "Dugme with Simge" => "Simgeli Düğme",
        "Dugme with size" => "Boyutlu düğme",
        "Card" => "Kart",
        "OnayKutusu" => "Onay Kutusu",
        "OnayKutusu AracIpucu" => "Onay Kutusu İpucu",
        "Circle Ilerleme" => "Dairesel İlerleme",
        "Circle with Color" => "Renkli Daire",
        "Cleanable and ESC to clean" => "Temizlenebilir ve ESC ile temizleme",
        "Click Handlers" => "Tıklama İşleyicileri",
        "Pano" => "Pano",
        "Pano AracIpucu" => "Pano İpucu",
        "Color Simge" => "Renkli Simge",
        "Color Picker" => "Renk Seçici",
        "Color Tags" => "Renk Etiketleri",
        "Combination Dividers" => "Birleşik Ayırıcılar",
        "Compact Style" => "Kompakt Stil",
        "Complex use" => "Karmaşık kullanım",
        "Context Menu" => "Bağlam Menüsü",
        "Currency Input with thousands separator" => "Binlik ayırıcılı para girdisi",
        "Custom (First 5 days of each month disabled)" => "Özel (Her ayın ilk 5 günü devre dışı)",
        "Custom Appearance" => "Özel Görünüm",
        "Custom Dugme" => "Özel Düğme",
        "Custom Color" => "Özel Renk",
        "Custom Context Menu" => "Özel Bağlam Menüsü",
        "Custom Simge" => "Özel Simge",
        "Custom Menu Max Height" => "Özel Menü Maksimum Yüksekliği",
        "Custom Paddings" => "Özel Dolgular",
        "Custom Style" | "Custom style" => "Özel Stil",
        "Custom Text Color" => "Özel Metin Rengi",
        "Custom Timing" => "Özel Zamanlama",
        "Custom Year Range (birthday, 1900 to current)" => {
            "Özel Yıl Aralığı (doğum tarihi, 1900'den bugüne)"
        }
        "Custom buttons" => "Özel düğmeler",
        "Date Picker Value" => "Tarih Seçici Değeri",
        "Date Input" => "Tarih Girişi",
        "Date Range" => "Tarih Aralığı",
        "Default" => "Varsayılan",
        "Default Open" => "Varsayılan Açık",
        "Default Range Mode" => "Varsayılan Aralık Modu",
        "Default Style" => "Varsayılan Stil",
        "Destructive Action" => "Yıkıcı Eylem",
        "IletisimKutusu without Title" => "Başlıksız İletişim Kutusu",
        "Disabled" => "Devre Dışı",
        "Dropdown Dugme" => "Açılır Düğme",
        "Empty Items" => "Boş Öğeler",
        "Expland Paragraphs" => "Paragrafları Genişlet",
        "File tree" => "Dosya Ağacı",
        "Fill Style" => "Dolgulu Stil",
        "Focus back test" => "Odağı geri alma testi",
        "Focused Input" => "Odaklı Girdi",
        "Font Size" => "Yazı Boyutu",
        "Ghost" => "Hayalet",
        "Horizontal Dividers" => "Yatay Ayırıcılar",
        "Horizontal Kaydirici" => "Yatay Kaydırıcı",
        "Horizontal Adimlayici" => "Yatay Adım Göstergesi",
        "Simge" => "Simge",
        "Simge Dugme" => "Simge Düğmesi",
        "Simge Adimlayici" => "Simgeli Adım Göstergesi",
        "Input Size" => "Girdi Boyutu",
        "Input State" => "Girdi Durumu",
        "Input with mask pattern: (999)-999-9999" => "Maske desenli girdi: (999)-999-9999",
        "Input with mask pattern: AAA-###-AAA" => "Maske desenli girdi: AAA-###-AAA",
        "KlavyeTusu" => "Klavye Kısayolu",
        "Keyboard Disabled" => "Klavye Devre Dışı",
        "Etiket" => "Etiket",
        "Etiket with color" => "Renkli etiket",
        "Etiket with secondary text" => "İkincil metinli etiket",
        "Etiket wrapping" => "Etiket sarmalama",
        "Large" => "Büyük",
        "Large size" => "Büyük boyut",
        "Logarithmic Kaydirici" => "Logaritmik Kaydırıcı",
        "Manual Close Bildirim" => "Elle Kapatılan Bildirim",
        "Masked Etiket" => "Maskeli Etiket",
        "Menu with scrollbar" => "Kaydırma çubuklu menü",
        "Multi-line" => "Çok Satırlı",
        "Multi-line Etiket" => "Çok Satırlı Etiket",
        "Multi-line, line-height and text wrap" => "Çok satır, satır yüksekliği ve metin sarmalama",
        "No Wrap" => "Sarma Yok",
        "Normal" => "Normal",
        "Normal Dugme" => "Normal Düğme",
        "Normal Input" => "Normal Girdi",
        "Normal SayfaKatmani" => "Normal Kenar Paneli",
        "Normal Size" => "Normal Boyut",
        "Bildirim with Type" => "Türlü Bildirim",
        "Open IletisimKutusu with IletisimIcerigi" => "IletisimIcerigi ile İletişim Kutusu Aç",
        "Outline" => "Çerçeveli",
        "Outline Dugme" => "Çerçeveli Düğme",
        "Outline Style" => "Çerçeveli Stil",
        "Outline Tabs" => "Çerçeveli Sekmeler",
        "Overlay Closable" => "Kaplama Kapatılabilir",
        "Sayfalama with 10 visible pages" => "10 görünür sayfalı sayfalama",
        "Pill Tabs" => "Hap Sekmeler",
        "Placeholder" => "Yer Tutucu",
        "AcilirKatman Anchor" => "Açılır İçerik Ankrajı",
        "AcilirKatman with Form" => "Formlu Açılır İçerik",
        "AcilirKatman with Liste" => "Listeli Açılır İçerik",
        "Popup Menu" => "Açılır Menü",
        "Positioning" => "Konumlandırma",
        "Prefix and Suffix" => "Ön Ek ve Son Ek",
        "Prevent Close" => "Kapatmayı Önle",
        "Ilerleme Bar" => "İlerleme Çubuğu",
        "Radyo" => "Radyo",
        "Radyo Group" => "Radyo Grubu",
        "Radyo Group Vertical (With container style)" => "Dikey Radyo Grubu (Kapsayıcı stili ile)",
        "Radyo AracIpucu" => "Radyo İpucu",
        "Range Mode" => "Aralık Modu",
        "Rich description (Markdown)" => "Zengin açıklama (Markdown)",
        "Right click to open AcilirKatman" => "Açılır içeriği sağ tıkla aç",
        "SVG from URL" => "URL'den SVG",
        "Kaydirilabilir IletisimKutusu" => "Kaydırılabilir İletişim Kutusu",
        "Kaydirilabilir SayfaKatmani" => "Kaydırılabilir Kenar Paneli",
        "Searchable" => "Aranabilir",
        "Searchable Secim" => "Aranabilir Seçim",
        "Segmented Tabs" => "Segmentli Sekmeler",
        "Segmented Tabs (With filling space)" => "Segmentli Sekmeler (Alanı dolduran)",
        "Secim" => "Seçim",
        "Selected Values" => "Seçili Değerler",
        "Session Timeout" => "Oturum Zaman Aşımı",
        "Simple Bildirim" => "Basit Bildirim",
        "Single line code editor" => "Tek satırlı kod düzenleyici",
        "Sizeable" => "Boyutlandırılabilir",
        "Iskelet" => "İskelet",
        "Kaydirici (0 - 5) and with color" => "Kaydırıcı (0 - 5) ve renkli",
        "Small" => "Küçük",
        "Small Size" => "Küçük Boyut",
        "Small Size with suffix" => "Son ekli küçük boyut",
        "Small size" => "Küçük boyut",
        "Small with 180px width" => "180px genişlikte küçük",
        "DonerGosterge" => "Yükleniyor Göstergesi",
        "DonerGosterge with Simge" => "Simgeli Yükleniyor Göstergesi",
        "DonerGosterge with color" => "Renkli Yükleniyor Göstergesi",
        "DonerGosterge with size" => "Boyutlu Yükleniyor Göstergesi",
        "Styling AcilirKatman" => "Açılır İçeriği Stillendirme",
        "Anahtar AracIpucu" => "Anahtar İpucu",
        "Sekme" => "Sekme",
        "Tablo" => "Tablo",
        "Tablo in IletisimKutusu" => "İletişim Kutusunda Tablo",
        "Tabs" => "Sekmeler",
        "Cip (default)" => "Etiket (varsayılan)",
        "Cip (outline)" => "Etiket (çerçeveli)",
        "Cip (rounded 0px)" => "Etiket (0px yuvarlaklık)",
        "Cip (rounded full)" => "Etiket (tam yuvarlak)",
        "Cip (small with rounded full)" => "Etiket (küçük ve tam yuvarlak)",
        "Cip (small)" => "Etiket (küçük)",
        "Text Align" => "Metin Hizalama",
        "Text Center" => "Ortalanmış Metin",
        "Textarea" => "Metin Alanı",
        "Gecis" => "Geçiş",
        "Gecis Dugme Group" => "Geçiş Düğmesi Grubu",
        "Gecis Group with Ghost Style" => "Hayalet Stilli Geçiş Grubu",
        "Gecis Group with Outline Style" => "Çerçeveli Stilli Geçiş Grubu",
        "Gecis Group with Segmented Style" => "Segmentli Stilli Geçiş Grubu",
        "Gecis AracIpucu" => "Geçiş İpucu",
        "AracIpucu for Dugme" => "Düğme İpucu",
        "Underline Tabs" => "Altı Çizili Sekmeler",
        "Unique Bildirim" => "Benzersiz Bildirim",
        "Unique with Key" => "Anahtarlı Benzersiz",
        "Update Available" => "Güncelleme Var",
        "User Profile Preview" => "Kullanıcı Profili Önizlemesi",
        "Vertical Dividers" => "Dikey Ayırıcılar",
        "Vertical Adimlayici" => "Dikey Adım Göstergesi",
        "Vertical with Range" => "Aralıklı Dikey",
        "With 3 Months" => "3 Ay ile",
        "With Border" => "Kenarlıklı",
        "With Disabled matcher (Sundays, Wednesdays, Saturdays)" => {
            "Devre dışı eşleştirici ile (Pazar, Çarşamba, Cumartesi)"
        }
        "With Simge" => "Simgeli",
        "With Etiket" => "Etiketli",
        "With Ilerleme" => "İlerleme ile",
        "With confirm mode" => "Onay modu ile",
        "With in an Input" => "Girdi içinde",
        "With mask pattern" => "Maske deseni ile",
        "With open_alert_dialog" => "open_alert_dialog ile",
        "With preview label" => "Önizleme etiketi ile",
        "With size" => "Boyut ile",
        "Without Appearance" => "Görünüm Olmadan",
        "Without Title" => "Başlıksız",
        "Without appearance" => "Görünüm olmadan",
        "Without label" => "Etiketsiz",
        "XSmall Size" => "Çok Küçük Boyut",
        _ => title,
    }
    .into()
}

pub struct StoryContainer {
    focus_handle: gpui::FocusHandle,
    pub name: SharedString,
    pub navigation_name: SharedString,
    pub title_bg: Option<Hsla>,
    pub description: SharedString,
    width: Option<gpui::Pixels>,
    height: Option<gpui::Pixels>,
    story: Option<AnyView>,
    story_klass: Option<SharedString>,
    closable: bool,
    zoomable: Option<PanelControl>,
    paddings: Pixels,
    on_active: Option<fn(AnyView, bool, &mut Window, &mut App)>,
}

#[derive(Debug)]
pub enum ContainerEvent {
    Close,
}

impl EventEmitter<ContainerEvent> for StoryContainer {}

fn localized_story_title(title: &'static str) -> &'static str {
    match title {
        "Akordeon" => "Akordeon",
        "Uyari" => "Uyarı",
        "UyariIletisimKutusu" => "Uyarı Penceresi",
        "Avatar" => "Avatar",
        "Rozet" => "Rozet",
        "GezintiYolu" => "Konum Yolu",
        "Dugme" => "Düğme",
        "Takvim" => "Takvim",
        "Chart" => "Grafik",
        "OnayKutusu" => "Onay Kutusu",
        "Pano" => "Pano",
        "Daraltilabilir" => "Katlanabilir",
        "RenkSecici" => "Renk Seçici",
        "VeriTablosu" => "Veri Tablosu",
        "TarihSecici" => "Tarih Seçici",
        "AciklamaListesi" => "Açıklama Listesi",
        "IletisimKutusu" => "İletişim Kutusu",
        "Ayirici" => "Ayırıcı",
        "AcilirDugme" => "Açılır Düğme",
        "Editor" => "Düzenleyici",
        "Form" => "Form",
        "GrupKutusu" => "Grup Kutusu",
        "UzerineGelmeKarti" => "Üzerine Gelme Kartı",
        "Simge" => "Simge",
        "Image" => "Görsel",
        "Input" => "Girdi",
        "Introduction" => "Giriş",
        "KlavyeTusu" => "Klavye Kısayolu",
        "Etiket" => "Etiket",
        "Liste" => "Liste",
        "Menu" => "Menü",
        "Bildirim" => "Bildirim",
        "NumberInput" => "Sayı Girdisi",
        "OtpInput" => "OTP Girdisi",
        "Sayfalama" => "Sayfalama",
        "AcilirKatman" => "Açılır İçerik",
        "Ilerleme" => "İlerleme",
        "Radyo" => "Radyo",
        "Puanlama" => "Puanlama",
        "Resizable" => "Yeniden Boyutlandırılabilir",
        "KaydirmaCubugu" => "Kaydırma Çubuğu",
        "Secim" => "Seçim",
        "Ayarlar" => "Ayarlar",
        "SayfaKatmani" => "Kenar Paneli",
        "YanCubuk" => "Kenar Çubuğu",
        "Iskelet" => "İskelet",
        "Kaydirici" => "Kaydırıcı",
        "DonerGosterge" => "Yükleniyor Göstergesi",
        "Adimlayici" => "Adım Göstergesi",
        "Anahtar" => "Anahtar",
        "Tablo" => "Tablo",
        "Tabs" => "Sekmeler",
        "Cip" => "Etiket",
        "Textarea" => "Metin Alanı",
        "Tema Colors" => "Tema Renkleri",
        "ToggleButton" => "Geçiş Düğmesi",
        "AracIpucu" => "İpucu",
        "Agac" => "Ağaç",
        "SanalListe" => "Sanal Liste",
        "Zed 1.0 Workspace" => "Zed 1.0 Çalışma Alanı",
        _ => title,
    }
}

fn localized_story_navigation_title(title: &'static str) -> SharedString {
    let localized = localized_story_title(title);
    if localized == title || matches!(title, "Zed 1.0 Workspace") {
        localized.into()
    } else {
        format!("{} ({})", localized, title).into()
    }
}

fn localized_story_description(description: &'static str) -> &'static str {
    match description {
        "A breadcrumb navigation element that shows the current location in a hierarchy." => {
            "Hiyerarşi içindeki geçerli konumu gösteren konum yolu bileşeni."
        }
        "A button that helps you copy text or other content to your clipboard." => {
            "Metni veya başka içerikleri panoya kopyalamayı sağlayan düğme."
        }
        "A calendar to select a date or date range." => {
            "Tarih veya tarih aralığı seçmek için takvim."
        }
        "A collection of settings groups and items for the application." => {
            "Uygulama için ayar grupları ve ayar öğeleri koleksiyonu."
        }
        "A color picker to select color." => "Renk seçmek için renk seçici.",
        "A complex data table with selection, sorting, column moving, and loading more." => {
            "Seçim, sıralama, sütun taşıma ve daha fazla yükleme destekleyen gelişmiş veri tablosu."
        }
        "A composable, themeable and customizable sidebar component." => {
            "Birleştirilebilir, temalanabilir ve özelleştirilebilir kenar çubuğu bileşeni."
        }
        "A control that allows the user to toggle between checked and not checked." => {
            "Kullanıcının seçili ve seçili olmayan durumlar arasında geçiş yapmasını sağlayan kontrol."
        }
        "A date picker to select a date or date range." => {
            "Tarih veya tarih aralığı seçmek için tarih seçici."
        }
        "A dialog dialog" => "İçerik ve eylemler göstermek için iletişim kutusu.",
        "A divider that can be either vertical or horizontal." => {
            "Dikey veya yatay kullanılabilen ayırıcı."
        }
        "A hover card displays content when hovering over a trigger element, with configurable delays." => {
            "Tetikleyici öğenin üzerine gelindiğinde, ayarlanabilir gecikmelerle içerik gösteren kart."
        }
        "A list displays a series of items." => "Bir dizi öğeyi gösteren liste.",
        "A modal dialog that interrupts the user with important content" => {
            "Önemli içerikle kullanıcı akışını kesen modal iletişim kutusu."
        }
        "A popup displays content on top of the main page." => {
            "Ana sayfanın üzerinde içerik gösteren açılır yüzey."
        }
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it." => {
            "Öğe klavye odağı aldığında veya üzerine gelindiğinde ilgili bilgiyi gösteren açılır ipucu."
        }
        "A red dot that indicates the number of unread messages." => {
            "Okunmamış mesaj sayısını belirten rozet."
        }
        "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time." => {
            "Aynı anda yalnızca biri seçilebilen radyo düğmeleri grubu."
        }
        "A set of layered sections of content—known as tab panels—that are displayed one at a time." => {
            "İçerik bölümlerini sekmeler halinde, her seferinde bir panel görünecek şekilde gösterir."
        }
        "A short item that can be used to categorize or label content." => {
            "İçeriği sınıflandırmak veya etiketlemek için kullanılan kısa öğe."
        }
        "A simple interactive star rating component." => {
            "Basit ve etkileşimli yıldız puanlama bileşeni."
        }
        "A step-by-step process for users to navigate through a series of steps." => {
            "Kullanıcıyı bir dizi adım içinde yönlendiren adım adım süreç."
        }
        "A styled container element that with an optional title \
        to groups related content together." => {
            "İlgili içeriği isteğe bağlı başlıkla gruplayan stillendirilmiş kapsayıcı."
        }
        "A tag style to display keyboard shortcuts" => {
            "Klavye kısayollarını göstermek için etiket stili."
        }
        "A Zed-inspired workspace pattern with parallel agent threads, split diff context, and right-docked project/git panels." => {
            "Paralel ajan oturumları, bölünmüş diff bağlamı ve sağa sabitlenen Proje/Git panelleri içeren Zed esinli çalışma alanı düzeni."
        }
        "Add scrollbar to a scrollable element." => "Kaydırılabilir öğeye kaydırma çubuğu ekler.",
        "Add vertical or horizontal, or both scrollbars to a container, \
        and use `virtual_list` to render a large number of items." => {
            "Bir kapsayıcıya dikey, yatay veya iki yönde kaydırma çubuğu ekler ve çok sayıda öğeyi `virtual_list` ile render eder."
        }
        "An interactive element that expands/collapses." => "Açılıp kapanabilen etkileşimli öğe.",
        "Avatar is an image that represents a user or organization." => {
            "Kullanıcıyı veya kurumu temsil eden görsel."
        }
        "Beautiful Charts & Graphs." => "Şık grafikler ve çizimler.",
        "Code editor with syntax highlighting by tree-sitter." => {
            "Agac-sitter ile sözdizimi vurgulaması yapan kod düzenleyici."
        }
        "Displays a button or a component that looks like a button." => {
            "Düğme veya düğme gibi görünen bir bileşen gösterir."
        }
        "Displays a callout for user attention." => {
            "Kullanıcının dikkatini çekmek için uyarı gösterir."
        }
        "Displays a list of options for the user to pick from—triggered by a button." => {
            "Düğmeyle açılan ve kullanıcının seçim yapabildiği seçenek listesi gösterir."
        }
        "Displays a slider control for selecting a value within a range." => {
            "Belirli aralıktaki değeri seçmek için kaydırıcı kontrolü gösterir."
        }
        "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar." => {
            "Bir görevin tamamlanma durumunu genellikle ilerleme çubuğu olarak gösterir."
        }
        "Displays an spinner showing the completion progress of a task." => {
            "Bir görevin ilerleme durumunu gösteren yükleniyor göstergesi."
        }
        "Form to collect multiple inputs." => "Birden fazla girdi toplamak için form.",
        "Image and SVG image supported." => "Görsel ve SVG görsel desteği.",
        "Input with multi-line mode." => "Çok satırlı girdi modu.",
        "Etiket used to display text or other content." => {
            "Metin veya başka içerik göstermek için kullanılan etiket."
        }
        "NumberInput design to support + - to adjust the input value." => {
            "Girdi değerini + ve - ile ayarlamayı destekleyen sayı girdisi."
        }
        "OTP Input uses to one-time password (OTP) input field or number password input field." => {
            "Tek kullanımlık parola (OTP) veya sayısal parola girişi için kullanılır."
        }
        "Sayfalama with page navigation, next and previous links." => {
            "Sayfa gezintisi, ileri ve geri bağlantıları içeren sayfalama."
        }
        "Popup menu and context menu" => "Açılır menü ve bağlam menüsü.",
        "Push notifications to display a message at the top right of the window" => {
            "Pencerenin sağ üstünde mesaj göstermek için bildirimler."
        }
        "SayfaKatmani for open a popup in the edge of the window" => {
            "Pencerenin kenarında açılır panel göstermek için kullanılır."
        }
        "SVG Icons based on Lucide.dev" => "Lucide.dev tabanlı SVG simgeleri.",
        "The accordion uses collapse internally to make it collapsible." => {
            "Akordeon, açılıp kapanmak için dahili olarak katlanabilir yapıyı kullanır."
        }
        "The resizable panels." => "Yeniden boyutlandırılabilir paneller.",
        "Use to display details with a tidy layout." => {
            "Ayrıntıları düzenli bir yerleşimle göstermek için kullanılır."
        }
        "Use to show a placeholder while content is loading." => {
            "İçerik yüklenirken yer tutucu göstermek için kullanılır."
        }
        _ => description,
    }
}

impl StoryContainer {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            name: "".into(),
            navigation_name: "".into(),
            title_bg: None,
            description: "".into(),
            width: None,
            height: None,
            story: None,
            story_klass: None,
            closable: true,
            zoomable: Some(PanelControl::default()),
            paddings: px(16.),
            on_active: None,
        }
    }

    pub fn panel<S: Story>(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let name = localized_story_title(S::title());
        let navigation_name = localized_story_navigation_title(S::title());
        let description = localized_story_description(S::description());
        let story = S::new_view(window, cx);
        let story_klass = S::klass();

        let view = cx.new(|cx| {
            let mut story = Self::new(window, cx)
                .story(story.into(), story_klass)
                .on_active(S::on_active_any);
            story.focus_handle = cx.focus_handle();
            story.closable = S::closable();
            story.zoomable = S::zoomable();
            story.name = name.into();
            story.navigation_name = navigation_name;
            story.description = description.into();
            story.title_bg = S::title_bg();
            story.paddings = S::paddings();
            story
        });

        view
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn story(mut self, story: AnyView, story_klass: impl Into<SharedString>) -> Self {
        self.story = Some(story);
        self.story_klass = Some(story_klass.into());
        self
    }

    pub fn on_active(mut self, on_active: fn(AnyView, bool, &mut Window, &mut App)) -> Self {
        self.on_active = Some(on_active);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryState {
    pub story_klass: SharedString,
}

impl StoryState {
    fn to_value(&self) -> serde_json::Value {
        serde_json::json!({
            "story_klass": self.story_klass,
        })
    }

    fn from_value(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }

    fn to_story(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> (
        &'static str,
        &'static str,
        bool,
        Option<PanelControl>,
        AnyView,
        fn(AnyView, bool, &mut Window, &mut App),
    ) {
        macro_rules! story {
            ($klass:tt) => {
                (
                    $klass::title(),
                    $klass::description(),
                    $klass::closable(),
                    $klass::zoomable(),
                    $klass::view(window, cx).into(),
                    $klass::on_active_any,
                )
            };
        }

        match self.story_klass.to_string().as_str() {
            "BreadcrumbStory" => story!(BreadcrumbStory),
            "ButtonStory" => story!(ButtonStory),
            "CalendarStory" => story!(CalendarStory),
            "SelectStory" => story!(SelectStory),
            "IconStory" => story!(IconStory),
            "ImageStory" => story!(ImageStory),
            "InputStory" => story!(InputStory),
            "ListStory" => story!(ListStory),
            "DialogStory" => story!(DialogStory),
            "DividerStory" => story!(DividerStory),
            "PopoverStory" => story!(PopoverStory),
            "ProgressStory" => story!(ProgressStory),
            "ResizableStory" => story!(ResizableStory),
            "ScrollbarStory" => story!(ScrollbarStory),
            "SwitchStory" => story!(SwitchStory),
            "DataTableStory" => story!(DataTableStory),
            "TableStory" => story!(TableStory),
            "LabelStory" => story!(LabelStory),
            "TooltipStory" => story!(TooltipStory),
            "AccordionStory" => story!(AccordionStory),
            "SidebarStory" => story!(SidebarStory),
            "FormStory" => story!(FormStory),
            "NotificationStory" => story!(NotificationStory),
            "ThemeColorsStory" => story!(ThemeColorsStory),
            _ => {
                unreachable!("Invalid story klass: {}", self.story_klass)
            }
        }
    }
}

impl Panel for StoryContainer {
    fn panel_name(&self) -> &'static str {
        "StoryContainer"
    }

    fn title(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.name.clone().into_any_element()
    }

    fn title_style(&self, cx: &App) -> Option<TitleStyle> {
        if let Some(bg) = self.title_bg {
            Some(TitleStyle {
                background: bg,
                foreground: cx.theme().foreground,
            })
        } else {
            None
        }
    }

    fn closable(&self, _cx: &App) -> bool {
        self.closable
    }

    fn zoomable(&self, _cx: &App) -> Option<PanelControl> {
        self.zoomable
    }

    fn visible(&self, cx: &App) -> bool {
        !AppState::global(cx)
            .invisible_panels
            .read(cx)
            .contains(&self.name)
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("panel: {} yakınlaştırıldı: {}", self.name, zoomed);
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, cx: &mut Context<Self>) {
        println!("panel: {} etkin: {}", self.name, active);
        if let Some(on_active) = self.on_active {
            if let Some(story) = self.story.clone() {
                on_active(story, active, _window, cx);
            }
        }
    }

    fn dropdown_menu(
        &mut self,
        menu: PopupMenu,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> PopupMenu {
        menu.menu("Bilgi", Box::new(ShowPanelInfo))
    }

    fn toolbar_buttons(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Vec<Dugme>> {
        Some(vec![
            Dugme::new("info")
                .icon(SimgeAdi::Info)
                .on_click(|_, window, cx| {
                    window.push_notification("Bilgi düğmesine tıkladınız.", cx);
                }),
            Dugme::new("search")
                .icon(SimgeAdi::Search)
                .on_click(|_, window, cx| {
                    window.push_notification("Arama düğmesine tıkladınız.", cx);
                }),
        ])
    }

    fn dump(&self, _cx: &App) -> PanelState {
        let mut state = PanelState::new(self);
        let story_state = StoryState {
            story_klass: self.story_klass.clone().unwrap(),
        };
        state.info = PanelInfo::panel(story_state.to_value());
        state
    }
}

impl EventEmitter<PanelEvent> for StoryContainer {}
impl Focusable for StoryContainer {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for StoryContainer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("story-container")
            .size_full()
            .overflow_y_scrollbar()
            .track_focus(&self.focus_handle)
            .when_some(self.story.clone(), |this, story| {
                this.child(div().size_full().p(self.paddings).child(story))
            })
    }
}

pub struct StoryRoot {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) title_bar: Entity<AppTitleBar>,
    pub(crate) view: AnyView,
}

impl StoryRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        Self {
            focus_handle: cx.focus_handle(),
            title_bar,
            view: view.into(),
        }
    }

    fn on_action_panel_info(
        &mut self,
        _: &ShowPanelInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        struct Info;
        let note = Bildirim::new()
            .message("Panel bilgisine tıkladınız.")
            .id::<Info>();
        window.push_notification(note, cx);
    }

    fn on_action_toggle_search(
        &mut self,
        _: &ToggleSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.propagate();
        if window.has_focused_input(cx) {
            return;
        }

        struct Search;
        let note = Bildirim::new()
            .message("Aramayı açıp kapattınız.")
            .id::<Search>();
        window.push_notification(note, cx);
    }
}

impl Focusable for StoryRoot {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StoryRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = KokGorunum::render_sheet_layer(window, cx);
        let dialog_layer = KokGorunum::render_dialog_layer(window, cx);
        let notification_layer = KokGorunum::render_notification_layer(window, cx);

        div()
            .id("story-root")
            .on_action(cx.listener(Self::on_action_panel_info))
            .on_action(cx.listener(Self::on_action_toggle_search))
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(
                        div()
                            .track_focus(&self.focus_handle)
                            .flex_1()
                            .overflow_hidden()
                            .child(self.view.clone()),
                    )
                    .children(sheet_layer)
                    .children(dialog_layer)
                    .children(notification_layer),
            )
    }
}
