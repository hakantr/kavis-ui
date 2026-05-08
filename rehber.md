# Zed GPUI Kullanım Haritası

Bu rehber, bu çalışma ağacındaki Zed/GPUI koduna göre hazırlanmıştır
(`git rev-parse --short HEAD`: `4b23564f36`). Amaç, Zed içinde yeni bir pencere,
başlık çubuğu, pencere dekorasyonu, platforma özel davranış, blur/transparency ve
UI bileşeni eklerken başka bir rehbere ihtiyaç duymadan doğru dosyaya ve doğru API'ye
yönelebilmektir.

Ana kaynak dosyalar:

- GPUI çekirdeği: `crates/gpui/src/gpui.rs`, `app.rs`, `window.rs`, `platform.rs`
- Platform seçimi: `crates/gpui_platform/src/gpui_platform.rs`
- Platform uygulamaları: `crates/gpui_macos`, `crates/gpui_windows`, `crates/gpui_linux`, `crates/gpui_web`
- Zed ana pencere seçenekleri: `crates/zed/src/zed.rs`
- Zed başlık çubuğu/dekorasyon: `crates/platform_title_bar`, `crates/title_bar`, `crates/workspace/src/workspace.rs`
- Zed UI bileşenleri: `crates/ui`, metin girişi için `crates/ui_input`
- Ayarlar: `crates/settings_content`, `crates/settings`, `crates/theme_settings`, `crates/theme`

## 1. Büyük Resim

GPUI üç katmanlıdır:

1. **Platform katmanı**: macOS, Windows, Linux, web ve test ortamlarını soyutlar.
   `Platform` ve `PlatformWindow` trait'leri burada ana sözleşmedir.
2. **Uygulama/durum katmanı**: `Application`, `App`, `Context<T>`, `Entity<T>`,
   `WeakEntity<T>`, `Task`, `Subscription`, `Global` ve event sistemini yönetir.
3. **Render/element katmanı**: `Render`, `RenderOnce`, `IntoElement`, `Element`,
   `div`, `canvas`, `list`, `uniform_list`, `img`, `svg`, `anchored`, `surface`
   ve `Styled`/`InteractiveElement` fluent API'leri ile UI ağacını oluşturur.

Zed bu katmanların üstüne kendi tasarım sistemini koyar:

- `crates/ui`: Button, Icon, Label, Modal, ContextMenu, Tooltip, Tab, Table, Toggle vb.
- `crates/platform_title_bar`: platforma göre pencere kontrol butonlarını ve başlık
  çubuğu davranışını çizer.
- `crates/workspace`: ana çalışma alanını, client-side decoration gölgesini, resize
  bölgelerini ve pencere içeriğini birleştirir.

## 2. Platform Başlatma

Yeni GPUI uygulaması başlatmanın standart yolu:

```rust
use gpui::{App, AppContext as _, WindowOptions, div, prelude::*};
use gpui_platform::application;

struct Root;

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child("Hello")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        if let Err(error) = cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Root)) {
            eprintln!("failed to open window: {error:?}");
        }
    });
}
```

Zed'de pratikte `application()` çağrısı, işletim sistemine göre şu platformu seçer:

- macOS: `gpui_macos::MacPlatform::new(headless)`
- Windows: `gpui_windows::WindowsPlatform::new(headless)`
- Linux/FreeBSD: `gpui_linux::current_platform(headless)`; Wayland/X11 backend'i
  platform crate içinde seçilir.
- Web/WASM: `gpui_web::WebPlatform`

`gpui_platform::headless()` test ve başsız çalıştırma için platform sağlar.
`gpui_platform::current_headless_renderer()` şu anda test desteği altında macOS'ta
Metal headless renderer döndürür.

## 3. Temel Bağlamlar

GPUI'de neredeyse her şey `cx` ile yapılır:

- `App`: global durum, pencere listesi, platform servisleri, keymap, global state,
  entity oluşturma ve pencere açma.
- `Context<T>`: bir `Entity<T>` güncellenirken gelir. `App` içine deref eder ve
  `cx.notify()`, `cx.emit(...)`, `cx.listener(...)`, `cx.observe(...)`,
  `cx.subscribe(...)`, `cx.spawn(...)` gibi entity odaklı API'leri ekler.
- `Window`: pencereye özel durum ve davranış. Focus, cursor, bounds, resize,
  title, background appearance, action dispatch, IME, prompt ve platform pencere
  işlemleri burada yapılır.
- `AsyncApp` / `AsyncWindowContext`: `await` noktaları boyunca tutulabilen async
  context. Entity/window kapanmış olabileceği için erişimler fallible olabilir.
- `TestAppContext` / `VisualTestContext`: testlerde simülasyon, zamanlayıcı ve
  görsel doğrulama için kullanılır.

Entity kullanımı:

```rust
let entity = cx.new(|cx| State::new(cx));

let value = entity.read(cx).value;

entity.update(cx, |state, cx| {
    state.value += 1;
    cx.notify();
});

let weak = entity.downgrade();
weak.update(cx, |state, cx| {
    state.value += 1;
    cx.notify();
})?;
```

Kurallar:

- Render çıktısını etkileyen state değiştiğinde `cx.notify()` çağır.
- Bir entity güncellenirken aynı entity'yi yeniden update etmeye çalışma; panic'e
  yol açabilir.
- Uzun yaşayan async işlerde `Entity<T>` yerine `WeakEntity<T>` yakala.
- `Task` düşerse iş iptal olur. Ya `await` et, ya struct alanında sakla, ya da
  `detach()` / `detach_and_log_err(cx)` kullan.

## 4. Render Modeli

Bir pencerenin root view'i `Entity<V>` olmalı ve `V: Render` implement etmelidir:

```rust
struct MyView {
    focus_handle: FocusHandle,
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("my-view")
            .track_focus(&self.focus_handle)
            .key_context("my-view")
            .on_action(cx.listener(|this, _: &CloseWindow, window, cx| {
                window.remove_window();
            }))
            .size_full()
            .child("Content")
    }
}
```

Reusable, state taşımayan bileşenlerde `RenderOnce` kullanılır:

```rust
#[derive(IntoElement)]
struct Badge {
    label: SharedString,
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().rounded_sm().px_2().child(self.label)
    }
}
```

`Render` mutable view state ile çalışır. `RenderOnce` sahipliği alır ve genellikle
Zed UI bileşenlerinde tercih edilir.

## 5. Element Haritası

GPUI yerleşik elementleri:

- `div()`: neredeyse tüm layout ve container işleri. Flex/grid, style, child,
  event, focus ve window-control area destekler.
- Metin: `&'static str`, `String`, `SharedString` doğrudan element olur.
  Daha karmaşık metin için `StyledText` ve `InteractiveText`.
- `svg()`: path veya external path ile SVG çizimi.
- `img(...)`: asset, path, URL, byte kaynağı gibi image kaynaklarını çizer; loading
  ve fallback slotları destekler.
- `canvas(prepaint, paint)`: düşük seviye çizim veya hitbox/cursor gibi prepaint
  gerektiren işler için.
- `anchored()`: pencere veya belirli bir noktaya sabitlenen popover/menu gibi UI.
- `deferred(child)`: öncelikli/ertelenmiş render.
- `list(...)`: değişken yükseklikli büyük listeler.
- `uniform_list(...)`: sabit/sık ölçülebilir item yüksekliği olan verimli listeler.
- `surface(...)`: platform/native surface kaynağını element olarak gösterir.

Sık kullanılan style grupları:

- Layout: `.flex()`, `.flex_col()`, `.flex_row()`, `.grid()`, `.items_center()`,
  `.justify_between()`, `.content_stretch()`, `.size_full()`, `.w(...)`, `.h(...)`
- Spacing: `.p_*`, `.px_*`, `.gap_*`, `.m_*`
- Text: `.text_color(...)`, `.text_sm()`, `.text_xl()`, `.font_family(...)`,
  `.truncate()`, `.line_clamp(...)`
- Border/shape: `.border_1()`, `.border_color(...)`, `.rounded_sm()`
- Position: `.absolute()`, `.relative()`, `.top(...)`, `.left(...)`
- State: `.hover(...)`, `.active(...)`, `.focus(...)`, `.focus_visible(...)`,
  `.group(...)`, `.group_hover(...)`
- Interaction: `.on_click(...)`, `.on_mouse_down(...)`, `.on_scroll_wheel(...)`,
  `.on_key_down(...)`, `.on_action(...)`, `.track_focus(...)`, `.key_context(...)`

Zed içinde `ui::prelude::*` genellikle `gpui::prelude::*` yerine tercih edilir;
tasarım sistemi tiplerini de getirir.

## 6. Pencere Oluşturma

Ana API:

```rust
let handle = cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::centered(size(px(900.), px(700.)), cx)),
        titlebar: Some(TitlebarOptions {
            title: Some("My Window".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.), px(9.))),
        }),
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        is_resizable: true,
        is_minimizable: true,
        window_min_size: Some(size(px(360.), px(240.))),
        window_background: cx.theme().window_background_appearance(),
        window_decorations: Some(gpui::WindowDecorations::Client),
        app_id: Some(ReleaseChannel::global(cx).app_id().to_owned()),
        ..Default::default()
    },
    |window, cx| {
        window.activate_window();
        cx.new(|cx| MyRootView::new(window, cx))
    },
)?;
```

`WindowOptions` alanları:

- `window_bounds`: `None` ise GPUI display default bounds seçer. `Some` verilirse
  `Windowed`, `Maximized` veya `Fullscreen` başlangıcı yapılır.
- `titlebar`: `Some(TitlebarOptions)` sistem başlık çubuğu konfigürasyonu.
  `None`, custom titlebar için kullanılır.
- `focus`: oluşturulduğunda odak alıp almayacağı.
- `show`: hemen gösterilip gösterilmeyeceği. Zed ana pencereleri başlangıçta
  `show: false`, `focus: false` ile açar ve hazır olunca gösterir.
- `kind`: `Normal`, `PopUp`, `Floating`, `Dialog`, Linux Wayland feature ile
  `LayerShell`.
- `is_movable`, `is_resizable`, `is_minimizable`: platform pencere kabiliyetleri.
- `display_id`: belirli monitor.
- `window_background`: `Opaque`, `Transparent`, `Blurred`, Windows için ayrıca
  `MicaBackdrop`, `MicaAltBackdrop`.
- `app_id`: Linux desktop grouping vb.
- `window_min_size`: minimum content size.
- `window_decorations`: `Server` veya `Client`. Linux'ta kritik; macOS/Windows'ta
  pratikte titlebar seçenekleri daha belirleyici.
- `icon`: X11 için pencere ikonu.
- `tabbing_identifier`: macOS native window tabs gruplaması.

`Window::new` içinde GPUI platform penceresini açar, sonra:

1. `platform_window.request_decorations(...)` çağırır.
2. `platform_window.set_background_appearance(window_background)` çağırır.
3. Bounds `Fullscreen` ise fullscreen, `Maximized` ise zoom uygular.
4. Platform callback'lerini bağlar.
5. İlk render'ı yapar.

## 7. Zed'de Ana Pencere Nasıl Açılır?

Zed'in ana referansı `crates/zed/src/zed.rs::build_window_options` fonksiyonudur.
Yeni ana workspace penceresi açacaksan bunu kullan:

```rust
let options = zed::build_window_options(display_uuid, cx);
let window = cx.open_window(options, |window, cx| {
    cx.new(|cx| Workspace::new(/* ... */, window, cx))
})?;
```

Bu fonksiyon şunları yapar:

- `display_uuid` ile uygun display'i bulur.
- `ZED_WINDOW_DECORATIONS=server|client` env override'ını okur.
- Aksi durumda `WorkspaceSettings::window_decorations` ayarını kullanır.
- `TitlebarOptions { appears_transparent: true, traffic_light_position: (9,9) }`
  kurar.
- `focus: false`, `show: false`, `kind: Normal` ayarlar.
- `window_background` değerini aktif temadan alır.
- Linux/FreeBSD'de app icon ekler.
- macOS native tabbing istenirse `tabbing_identifier: Some("zed")` verir.

Modal/About gibi küçük pencereler için doğrudan `WindowOptions` oluşturmak normaldir.
`crates/zed/src/zed.rs::about` örneği:

- Centered bounds
- `appears_transparent: true`
- `is_resizable: false`
- `is_minimizable: false`
- `kind: Normal`

## 8. Display ve Çoklu Monitor

Display bilgisi:

```rust
for display in cx.displays() {
    let id = display.id();
    let bounds = display.bounds();
    let visible = display.visible_bounds();
    let uuid = display.uuid().ok();
}
```

Belirli ekranda pencere açmak için:

```rust
WindowOptions {
    display_id: Some(display.id()),
    window_bounds: Some(WindowBounds::Windowed(bounds)),
    ..Default::default()
}
```

`Bounds` ekran koordinatlarıdır. `WindowBounds::centered(size, cx)` ana/default
display üzerinde merkezler. Elle konumlandırma gerekiyorsa `Bounds::new(origin, size)`
kullan.

## 9. WindowKind Davranışı

- `Normal`: ana uygulama penceresi.
- `PopUp`: diğer pencerelerin üstünde, bildirim ve geçici popup için. Zed bildirim
  pencerelerinde kullanır.
- `Floating`: parent üstünde floating panel.
- `Dialog`: parent etkileşimini kapatan modal platform penceresi.
- `LayerShell`: Wayland layer-shell feature ile dock/overlay/wallpaper benzeri
  yüzeyler için.

Pop-up/bildirim pencerelerinde tipik seçenekler:

```rust
WindowOptions {
    titlebar: None,
    kind: WindowKind::PopUp,
    focus: false,
    show: true,
    is_movable: false,
    window_background: WindowBackgroundAppearance::Transparent,
    window_decorations: Some(WindowDecorations::Client),
    ..Default::default()
}
```

Zed örnekleri: `crates/agent_ui/src/ui/agent_notification.rs`,
`crates/collab_ui/src/collab_ui.rs`.

## 10. Başlık Çubuğu ve Pencere Dekorasyonu

İki kavramı ayır:

- `TitlebarOptions`: macOS/Windows native titlebar görünümü, title ve macOS traffic
  light konumu.
- `WindowDecorations`: Linux/Wayland/X11 tarafında server-side decoration mı,
  client-side decoration mı istendiği.

GPUI tipleri:

```rust
pub enum WindowDecorations {
    Server,
    Client,
}

pub enum Decorations {
    Server,
    Client { tiling: Tiling },
}
```

`WindowDecorations`, pencere açarken istenen moddur. `Decorations`, platformun fiili
durumudur ve `window.window_decorations()` ile okunur.

Zed ayarı:

```json
{
  "window_decorations": "client"
}
```

Env override:

```sh
ZED_WINDOW_DECORATIONS=server
ZED_WINDOW_DECORATIONS=client
```

Zed settings tipi `settings_content::workspace::WindowDecorations` sadece `client`
ve `server` destekler; default `client`.

## 11. Custom Titlebar Nasıl Tanımlanır?

Basit GPUI uygulamasında:

```rust
cx.open_window(
    WindowOptions {
        titlebar: None,
        ..Default::default()
    },
    |_, cx| cx.new(|_| MyView),
)?;
```

Root view içinde kendi başlık çubuğunu çiz:

```rust
div()
    .flex()
    .flex_col()
    .size_full()
    .child(
        h_flex()
            .window_control_area(WindowControlArea::Drag)
            .h(px(34.))
            .child("Title")
    )
    .child(content)
```

Windows'ta caption button bölgeleri için `window_control_area` çok önemlidir:

- `WindowControlArea::Drag`: sürüklenebilir başlık alanı.
- `WindowControlArea::Close`: native close hit-test alanı.
- `WindowControlArea::Max`: maximize/restore hit-test alanı.
- `WindowControlArea::Min`: minimize hit-test alanı.

Zed'de yeni workspace benzeri pencere yapıyorsan custom titlebar'ı sıfırdan yazma.
`PlatformTitleBar` kullan:

```rust
let platform_titlebar = cx.new(|cx| PlatformTitleBar::new("my-titlebar", cx));

platform_titlebar.update(cx, |titlebar, _| {
    titlebar.set_children([my_left_or_center_content.into_any_element()]);
});

platform_titlebar.into_any_element()
```

`PlatformTitleBar` şunları halleder:

- Linux client-side decoration için sol/sağ pencere kontrol butonları.
- Windows pencere kontrol butonları.
- macOS traffic light padding ve double-click davranışı.
- Linux double-click ile zoom/maximize.
- Başlık çubuğu drag alanı.
- Linux'ta sağ tık window menu.
- Sidebar açıkken kontrol butonları ve köşe yuvarlamalarını ayarlama.

## 12. Kontrol Butonları Nasıl Yönetilir?

Kontrol butonları platforma göre farklı çizilir:

- macOS: native traffic lights; Zed sadece padding ve `traffic_light_position` ayarlar.
- Windows: `platform_title_bar::platforms::platform_windows::WindowsWindowControls`
  caption button render eder; her buton `WindowControlArea` ile native hit-test
  alanına bağlanır.
- Linux: `platform_title_bar::platforms::platform_linux::LinuxWindowControls`
  `WindowButtonLayout` ve `WindowControls` bilgisine göre close/min/max çizer.

Sol/sağ kontrol çizmek için hazır fonksiyonlar:

```rust
platform_title_bar::render_left_window_controls(
    cx.button_layout(),
    Box::new(workspace::CloseWindow),
    window,
)

platform_title_bar::render_right_window_controls(
    cx.button_layout(),
    Box::new(workspace::CloseWindow),
    window,
)
```

Close butonu doğrudan `window.remove_window()` çağırmaz; Zed'de close action
dispatch edilir:

```rust
window.dispatch_action(workspace::CloseWindow.boxed_clone(), cx);
```

Böylece dirty buffer, confirmation, workspace close mantığı ve keybinding ile aynı
akış kullanılır.

Linux `WindowButtonLayout`:

- Platformdan `cx.button_layout()` ile gelir.
- GNOME tarzı `"close,minimize:maximize"` formatı parse edilebilir.
- Default Linux fallback: sağda minimize, maximize, close.
- `TitleBarSettings` içinde kullanıcı override'ı da vardır; `TitleBar` bunu
  `PlatformTitleBar::set_button_layout` ile geçirir.

## 13. Client-Side Decoration ve Resize

Zed'in client-side decoration wrapper'ı:

```rust
workspace::client_side_decorations(element, window, cx, border_radius_tiling)
```

Yaptıkları:

- `window.window_decorations()` ile fiili decoration modunu okur.
- Client decoration ise `window.set_client_inset(theme::CLIENT_SIDE_DECORATION_SHADOW)` çağırır.
- Server decoration ise inset'i `0` yapar.
- Tiling durumuna göre köşe yuvarlamalarını kaldırır.
- Border ve shadow çizer.
- Kenar/corner bölgelerinde cursor'u resize cursor'a çevirir.
- Mouse down'da `window.start_window_resize(edge)` çağırır.

Kendi client-side decoration yapacaksan aynı prensipleri uygula:

1. Fiili modu `window.window_decorations()` ile oku.
2. Client ise gölge/invisible resize alanı kadar `set_client_inset` ver.
3. Tiling varsa ilgili kenar/köşeye radius, padding ve shadow verme.
4. Resize bölgelerinde `ResizeEdge` hesapla.
5. Hareket için titlebar'a `WindowControlArea::Drag` veya Linux/macOS için
   `window.start_window_move()` bağla.

Linux'ta server-side decoration her zaman mümkün olmayabilir:

- Wayland'de compositor decoration protocol sağlamazsa server isteği client'a düşer.
- X11'de compositor yoksa client-side decoration server'a düşebilir.

Bu yüzden pencere açarken istediğin modu değil, her render'da fiili
`window.window_decorations()` sonucunu esas al.

## 14. Platforma Göre Dekorasyon Davranışı

### macOS

- `TitlebarOptions::appears_transparent = true` style mask'e
  `NSFullSizeContentViewWindowMask` ekler.
- `traffic_light_position` native close/min/zoom butonlarının konumunu taşır.
- `titlebar_double_click()` native double-click aksiyonunu uygular.
- `start_window_move()` native `performWindowDragWithEvent` çağırır.
- `tabbing_identifier` verilirse native window tabbing açılır.
- `WindowDecorations` pratikte platform no-op gibi davranır; macOS için başlık
  çubuğu davranışını `TitlebarOptions` belirler.

### Windows

- `TitlebarOptions::appears_transparent` custom/full content titlebar için kullanılır.
- Caption butonlarının native hit-test davranışı `WindowControlArea` üzerinden
  `HTCLOSE`, `HTMAXBUTTON`, `HTMINBUTTON`, `HTCAPTION` olarak platform event
  katmanında eşlenir.
- `WindowBackgroundAppearance::MicaBackdrop` ve `MicaAltBackdrop` DWM backdrop
  attribute ile uygulanır.
- `WindowControls` çizimi Zed tarafında Windows component ile yapılır.

### Linux/FreeBSD - Wayland

- `WindowDecorations::Server` xdg-decoration protocol ile istenir.
- Compositor server-side decoration desteklemiyorsa client-side'a düşülür.
- `window_controls()` Wayland capabilities bilgisinden gelir: fullscreen,
  maximize, minimize, window menu.
- `show_window_menu`, `start_window_move`, `start_window_resize` xdg_toplevel
  üzerinden compositor'a devredilir.
- Blur için compositor `blur_manager` destekliyorsa `Blurred` yüzeyde blur commit
  edilir.

### Linux/FreeBSD - X11

- `request_decorations` `_MOTIF_WM_HINTS` yazar.
- Client-side decoration compositor gerektirir; yoksa server-side'a düşer.
- `show_window_menu` `_GTK_SHOW_WINDOW_MENU` client message gönderir.
- Move/resize `_NET_WM_MOVERESIZE` tarzı mesajla başlatılır.
- Tiling, fullscreen ve maximize state'leri `Decorations::Client { tiling }`
  sonucunu etkiler.

### Web/WASM

- Web platformunda native pencere dekorasyonu kavramı yoktur.
- `WindowBackgroundAppearance` şu anda web window için opaque/no-op kabul edilir.
- Entry point'te `gpui_platform::web_init()` çağır.

## 15. Blur, Transparency ve Mica Yönetimi

GPUI tipi:

```rust
pub enum WindowBackgroundAppearance {
    Opaque,
    Transparent,
    Blurred,
    MicaBackdrop,
    MicaAltBackdrop,
}
```

Zed tema ayarı sadece şunları kullanıcı tema içeriğinden destekler:

```json
{
  "experimental.theme_overrides": {
    "window_background_appearance": "blurred"
  }
}
```

Desteklenen setting değerleri: `opaque`, `transparent`, `blurred`.
`MicaBackdrop` ve `MicaAltBackdrop` GPUI seviyesinde var, ancak Zed tema schema'sı
şu anda bunları expose etmiyor.

Zed akışı:

- Tema refine edilirken `WindowBackgroundContent` -> `WindowBackgroundAppearance`
  dönüştürülür.
- Ana pencere açılırken `window_background: cx.theme().window_background_appearance()`.
- Settings/theme değiştiğinde `crates/zed/src/main.rs` tüm açık pencerelerde
  `window.set_background_appearance(background_appearance)` çağırır.
- UI tarafında `ui::styles::appearance::theme_is_transparent(cx)` transparent veya
  blurred ise true döner; opak arka plan varsayan bileşenler buna göre davranmalıdır.

Platform davranışı:

- macOS:
  - `Opaque` native window opaque yapar.
  - `Transparent` ve `Blurred` için renderer transparency açılır.
  - macOS 12 öncesi blur `CGSSetWindowBackgroundBlurRadius` ile 80 radius kullanır.
  - macOS 12+ `NSVisualEffectView` tabanlı blur view ekler/kaldırır.
- Windows:
  - `Opaque`: composition attribute kapatılır.
  - `Transparent`: composition state transparent.
  - `Blurred`: acrylic/blur benzeri composition attribute.
  - `MicaBackdrop`: DWM `DWMSBT_MAINWINDOW`.
  - `MicaAltBackdrop`: DWM `DWMSBT_TABBEDWINDOW`.
- Wayland:
  - Compositor blur protocol desteklerse `Blurred` yüzeye blur uygular.
  - Aksi durumda blur isteği görünür fark yaratmayabilir.
- X11:
  - Transparent/blur renderer transparency'yi etkiler, gerçek backdrop blur window
    manager/compositor desteğine bağlıdır.

Pratik karar tablosu:

- Tema/ana pencere için: `cx.theme().window_background_appearance()` kullan.
- Geçici overlay/bildirim için: `Transparent`.
- Windows 11 özel Mica istiyorsan: doğrudan `WindowBackgroundAppearance::MicaBackdrop`
  veya `MicaAltBackdrop` kullan; fakat Zed theme setting'e otomatik bağlanmaz.
- Blur kullanıyorsan: içerikte gerçekten alfa bırak; tamamen opak root background
  blur'u görünmez yapar.

## 16. Pencere Üzerinden Yapılan İşlemler

Sık kullanılan `Window` API'leri:

- `window.bounds()`: global ekran koordinatlarında bounds.
- `window.window_bounds()`: tekrar açma/restore için `WindowBounds`.
- `window.inner_window_bounds()`: Linux inset hariç bounds.
- `window.viewport_size()`: drawable content size.
- `window.resize(size)`: content size değiştirir.
- `window.is_fullscreen()`, `window.is_maximized()`
- `window.activate_window()`
- `window.minimize_window()`
- `window.zoom_window()`
- `window.toggle_fullscreen()`
- `window.remove_window()`
- `window.set_window_title(title)`
- `window.set_app_id(app_id)`
- `window.set_background_appearance(appearance)`
- `window.set_window_edited(true/false)` macOS dirty indicator.
- `window.set_document_path(path)` macOS document accessibility/path.
- `window.show_window_menu(position)` Linux titlebar context menu.
- `window.start_window_move()`, `window.start_window_resize(edge)`
- `window.request_decorations(WindowDecorations::Client/Server)`
- `window.window_decorations()`
- `window.window_controls()`
- `window.prompt(...)`
- `window.play_system_bell()`

macOS window tab API'leri:

- `window.tabbed_windows()`
- `window.tab_bar_visible()`
- `window.merge_all_windows()`
- `window.move_tab_to_new_window()`
- `window.toggle_window_tab_overview()`
- `window.set_tabbing_identifier(...)`

## 17. Focus, Blur ve Keyboard

Focus handle:

```rust
struct View {
    focus_handle: FocusHandle,
}

impl View {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}
```

Render:

```rust
div()
    .track_focus(&self.focus_handle)
    .focus_visible(|style| style.border_color(cx.theme().colors().border_focused))
```

Focus vermek:

```rust
self.focus_handle.focus(window, cx);
// veya
cx.focus_view(&child_entity, window);
```

Focus olayları:

- `cx.on_focus(handle, window, ...)`: handle doğrudan focus aldı.
- `cx.on_focus_in(handle, window, ...)`: handle veya descendant focus aldı.
- `cx.on_blur(handle, window, ...)`: handle focus kaybetti.
- `cx.on_focus_out(handle, window, ...)`: handle veya descendant focus dışına çıktı.
- `cx.on_focus_lost(window, ...)`: pencere içinde focus kalmadı.

Keyboard action akışı:

1. `actions!(namespace, [ActionA, ActionB])` veya `#[gpui::action]` ile action tanımla.
2. Element ağacında `.key_context("context-name")` belirt.
3. `cx.bind_keys([KeyBinding::new("cmd-k", ActionA, Some("context-name"))])`.
4. Handler için `.on_action(...)`, `.capture_action(...)` veya `cx.on_action(...)` kullan.

Event propagation:

- Mouse/key event handler'lar default propagate eder.
- `cx.stop_propagation()` daha arkadaki/üstteki handler'lara gitmesini keser.
- Action bubble phase'de handler'lar default propagation'ı durdurur; gerekirse
  `cx.propagate()` kullanılır.

## 18. Mouse, Drag, Drop ve Hitbox

Element interactivity:

- `.on_click(...)`
- `.on_mouse_down(...)`, `.on_mouse_up(...)`, `.on_mouse_move(...)`
- `.on_mouse_down_out(...)`, `.on_mouse_up_out(...)`
- `.on_scroll_wheel(...)`
- `.on_pinch(...)`
- `.on_drag_move::<T>(...)`
- `.drag_over::<T>(...)`
- `.on_drop::<T>(...)`
- `.can_drop(...)`
- `.occlude()` veya `.block_mouse_except_scroll()`
- `.cursor_pointer()`, `.cursor(...)`

Pencere kontrol hitbox'ı:

```rust
h_flex()
    .window_control_area(WindowControlArea::Drag)
```

Custom resize/cursor için `canvas` ile hitbox eklemek Zed'deki client decoration
desenidir:

```rust
canvas(
    |_bounds, window, _| {
        window.insert_hitbox(bounds, HitboxBehavior::Normal)
    },
    |_bounds, hitbox, window, cx| {
        window.set_cursor_style(CursorStyle::ResizeLeftRight, &hitbox);
    },
)
```

## 19. Async İşler

Foreground task:

```rust
cx.spawn(async move |cx| {
    cx.update(|cx| {
        // App state güncelle
    })
})
.detach();
```

Entity task:

```rust
cx.spawn(|this, cx| async move {
    let result = cx.background_executor().timer(Duration::from_millis(200)).await;
    this.update(cx, |this, cx| {
        this.ready = true;
        cx.notify();
    })?;
    anyhow::Ok(())
})
.detach_and_log_err(cx);
```

Window'a bağlı task:

```rust
cx.spawn_in(window, |this, cx| async move {
    this.update_in(cx, |this, window, cx| {
        window.activate_window();
        cx.notify();
    })?;
    anyhow::Ok(())
})
.detach_and_log_err(cx);
```

Background thread:

```rust
let task = cx.background_spawn(async move {
    expensive_work().await
});

cx.spawn(async move |cx| {
    let result = task.await;
    cx.update(|cx| {
        // sonucu UI'a taşı
    })
})
.detach();
```

Testlerde zamanlayıcı:

- GPUI testlerinde `smol::Timer::after(...)` yerine
  `cx.background_executor().timer(duration).await` kullan.
- `run_until_parked()` ile uyum için GPUI executor timer'ı tercih edilir.

## 20. Global State, Observe ve Event

Global state:

```rust
struct MyGlobal(State);
impl Global for MyGlobal {}

cx.set_global(MyGlobal(state));

cx.update_global::<MyGlobal, _>(|global, cx| {
    global.0.changed = true;
});

let value = cx.read_global::<MyGlobal, _>(|global, _| global.0.clone());
```

Observe:

```rust
subscriptions.push(cx.observe(&other, |this, other, cx| {
    this.copy = other.read(cx).value;
    cx.notify();
}));
```

Event:

```rust
struct Saved;
impl EventEmitter<Saved> for Document {}

cx.emit(Saved);

subscriptions.push(cx.subscribe(&document, |this, document, _: &Saved, cx| {
    this.last_saved = Some(document.entity_id());
    cx.notify();
}));
```

Window observe:

- `cx.observe_window_bounds(window, ...)`
- `cx.observe_window_activation(window, ...)`
- `cx.observe_window_appearance(window, ...)`
- `cx.observe_button_layout_changed(window, ...)`
- `cx.observe_pending_input(window, ...)`
- `cx.observe_keystrokes(...)`

## 21. Platform Servisleri

`App` üzerinden:

- Uygulama: `quit`, `restart`, `activate`, `hide`, `hide_other_apps`, `unhide_other_apps`
- Pencereler: `windows`, `active_window`, `window_stack`
- Display: `displays`, `primary_display`, `find_display`
- Appearance: `window_appearance`, `button_layout`
- Clipboard: `read_from_clipboard`, `write_to_clipboard`
- Linux primary selection: `read_from_primary`, `write_to_primary`
- macOS find pasteboard: `read_from_find_pasteboard`, `write_to_find_pasteboard`
- Keychain: `write_credentials`, `read_credentials`, `delete_credentials`
- URL: `open_url`, `register_url_scheme`
- Dosya/prompt: `prompt_for_paths`, `prompt_for_new_path`, `reveal_path`,
  `open_with_system`, `can_select_mixed_files_and_dirs`
- Menü: `set_menus`, `get_menus`, `set_dock_menu`, `add_recent_document`,
  `update_jump_list`
- Termal durum: `thermal_state`, `on_thermal_state_change`
- Cursor: `set_cursor_style`, `hide_cursor_until_mouse_moves`, `is_cursor_visible`
- Screen capture: `is_screen_capture_supported`, `screen_capture_sources`

Platform trait implementasyonu yazıyorsan `Platform` ve `PlatformWindow` içindeki
tüm bu sözleşmeleri karşılaman gerekir. Uygulama geliştirirken doğrudan trait'e
değil `App`/`Window` wrapper'larına dokunmak tercih edilir.

## 22. Text Input ve IME

Platform IME entegrasyonu `InputHandler` üzerinden çalışır. Editor benzeri metin
alanları şunları sağlamalıdır:

- `selected_text_range`
- `marked_text_range`
- `text_for_range`
- `replace_text_in_range`
- `replace_and_mark_text_in_range`
- `unmark_text`
- `bounds_for_range`
- `character_index_for_point`
- `accepts_text_input`
- `prefers_ime_for_printable_keys`

IME aday penceresini doğru yerde tutmak için:

```rust
window.invalidate_character_coordinates();
```

Zed'de form tipi tek satır input için doğrudan editor yazmak yerine `ui_input::InputField`
kullan. Bu crate editor'a bağlı olduğu için `ui` içinde değildir.

## 23. Zed UI Bileşen Envanteri

Zed'de yeni UI yazarken önce `ui` bileşenlerini ara. Başlıca bileşenler:

- Metin: `Label`, `Headline`, `HighlightedLabel`, `LoadingLabel`, `SpinnerLabel`
- Buton: `Button`, `IconButton`, `SelectableButton`, `ButtonLike`,
  `ButtonLink`, `CopyButton`, `SplitButton`, `ToggleButton`
- İkon: `Icon`, `DecoratedIcon`, `IconDecoration`, `IconName`, `IconSize`
- Form/toggle: `Checkbox`, `Switch`, `SwitchField`, `DropdownMenu`
- Menü/popup: `ContextMenu`, `RightClickMenu`, `Popover`, `PopoverMenu`, `Tooltip`
- Liste/tree: `List`, `ListItem`, `ListHeader`, `ListSubHeader`, `ListSeparator`,
  `TreeViewItem`, `StickyItems`, `IndentGuides`
- Tab: `Tab`, `TabBar`
- Layout yardımcıları: `h_flex`, `v_flex`, `h_group*`, `v_group*`, `Stack`,
  `Group`, `Divider`
- Veri: `Table`, `TableInteractionState`, `RedistributableColumnsState`,
  `render_table_row`, `render_table_header`
- Feedback: `Banner`, `Callout`, `Modal`, `AlertModal`, `AnnouncementToast`,
  `Notification`, `CountBadge`, `Indicator`, `ProgressBar`, `CircularProgress`
- Diğer: `Avatar`, `Facepile`, `Chip`, `DiffStat`, `Disclosure`,
  `GradientFade`, `Image`, `KeyBinding`, `KeybindingHint`, `Navigable`,
  `RedistributableColumns`
- AI/collab özel: `AiSettingItem`, `AgentSetupButton`, `ThreadItem`,
  `ConfiguredApiCard`, `CollabNotification`, `UpdateButton`

Genel kural:

- Zed içinde ham `div().on_click(...)` ile buton üretmeden önce `Button` veya
  `IconButton` kullan.
- Sadece görsel/tek seferlik parça için `RenderOnce`, stateful view için `Render`.
- Listeler çok büyükse `list` veya `uniform_list` kullan.
- Tooltip, popover ve context menu için hazır bileşenleri kullan; focus/blur
  kapanma davranışı orada çözülmüş durumdadır.

## 24. Reçeteler

### Yeni Workspace Penceresi

1. `zed::build_window_options(display_uuid, cx)` kullan.
2. Root view olarak workspace/multi-workspace entity oluştur.
3. Titlebar için `TitleBar`/`PlatformTitleBar` yolunu izle.
4. Root content'i `workspace::client_side_decorations(...)` ile sar.
5. Close işlemi için `workspace::CloseWindow` action'ını dispatch et.

### Küçük Dialog Penceresi

```rust
cx.open_window(
    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Dialog".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(12.), px(12.))),
        }),
        window_bounds: Some(WindowBounds::centered(size(px(440.), px(300.)), cx)),
        is_resizable: false,
        is_minimizable: false,
        kind: WindowKind::Dialog,
        app_id: Some(ReleaseChannel::global(cx).app_id().to_owned()),
        ..Default::default()
    },
    |window, cx| {
        window.activate_window();
        cx.new(|cx| DialogView::new(window, cx))
    },
)?;
```

### Transparent/Blurred Notification

```rust
WindowOptions {
    titlebar: None,
    focus: false,
    show: true,
    kind: WindowKind::PopUp,
    is_movable: false,
    is_resizable: false,
    window_background: WindowBackgroundAppearance::Transparent,
    window_decorations: Some(WindowDecorations::Client),
    ..Default::default()
}
```

Blur istersen `Transparent` yerine `Blurred` kullan; içerik root'unun tamamen opak
arka plan çizmediğinden emin ol.

### Platforma Göre UI Ayırma

```rust
match PlatformStyle::platform() {
    PlatformStyle::Mac => { /* macOS */ }
    PlatformStyle::Linux => { /* Linux */ }
    PlatformStyle::Windows => { /* Windows */ }
}
```

Compile-time farklılık gerekiyorsa `cfg!(target_os = "...")` veya `#[cfg(...)]`
kullan. Runtime styling için `PlatformStyle` daha okunur.

### Titlebar Drag ve Double Click

```rust
h_flex()
    .window_control_area(WindowControlArea::Drag)
    .on_click(|event, window, _| {
        if event.click_count() == 2 {
            if cfg!(target_os = "macos") {
                window.titlebar_double_click();
            } else {
                window.zoom_window();
            }
        }
    })
```

Linux/macOS'ta elle drag başlatman gerekirse mouse move sırasında:

```rust
window.start_window_move();
```

Windows için `WindowControlArea::Drag` native hit-test tarafında daha doğru yoldur.

### Client-Side Resize Handle

```rust
.on_mouse_down(MouseButton::Left, move |event, window, _| {
    if let Some(edge) = resize_edge(event.position, shadow, size, tiling) {
        window.start_window_resize(edge);
    }
})
```

Cursor'u da aynı edge'e göre `ResizeUpDown`, `ResizeLeftRight`,
`ResizeUpLeftDownRight`, `ResizeUpRightDownLeft` yap.

### Tema Değişince Pencere Arka Planını Güncelleme

```rust
cx.observe_global::<SettingsStore>(move |cx| {
    for window in cx.windows() {
        let appearance = cx.theme().window_background_appearance();
        window.update(cx, |_, window, _| {
            window.set_background_appearance(appearance);
        }).ok();
    }
}).detach();
```

Zed ana uygulaması bu deseni zaten kullanır.

## 25. Test Rehberi

GPUI testlerinde:

- `#[gpui::test]` macro'su ve `TestAppContext` kullan.
- Pencere gerekiyorsa test context'in offscreen/window helper'larını kullan.
- Async timer için `cx.background_executor().timer(duration).await`.
- UI action testlerinde key binding ve action dispatch'i doğrudan test et.
- Görsel test gerekiyorsa `VisualTestContext` ve headless renderer desteğini kontrol et.
- Element debug bounds gerekiyorsa test-support altında `.debug_selector(...)` ekle.

Testlerde kaçınılacaklar:

- `smol::Timer::after(...)` ile `run_until_parked()` beklemek.
- `unwrap()` ile test dışı production yoluna panik taşımak.
- Async hata sonuçlarını `let _ = ...` ile yutmak.

## 26. Sık Hatalar ve Doğru Desenler

- **İstenen decoration'a güvenme**: `WindowOptions.window_decorations` sadece istektir.
  Render sırasında `window.window_decorations()` sonucunu kullan.
- **Blur görünmüyor**: Root veya theme tamamen opak renk çiziyor olabilir. Transparent
  surface ve alfa gerekir.
- **Linux kontrol butonları yanlış tarafta**: `cx.button_layout()` ve
  `observe_button_layout_changed` kullanılmalı.
- **Windows caption butonları tıklanmıyor**: Buton elementlerinde
  `window_control_area(Close/Max/Min)` eksik olabilir.
- **Close davranışı bypass ediliyor**: Zed workspace penceresinde doğrudan
  `remove_window` yerine `workspace::CloseWindow` action'ını dispatch et.
- **Async task çalışırken yok oluyor**: Dönen `Task` saklanmamış veya detach edilmemiştir.
- **Entity leak**: Uzun yaşayan task/subscription içinde güçlü `Entity` yakalamak yerine
  `WeakEntity` kullan.
- **Render güncellenmiyor**: State değişiminden sonra `cx.notify()` unutulmuştur.
- **Focus callback'i fire etmiyor**: Element `.track_focus(&focus_handle)` ile ağaca
  bağlanmamış olabilir.
- **Custom titlebar altında içerik tıklanamıyor**: Drag/window control hitbox'ı fazla
  geniş olabilir veya `.occlude()` yanlış yerde olabilir.
- **Client decoration shadow boşluğu**: `set_client_inset` ve dış wrapper padding/shadow
  birlikte yönetilmelidir.

## 27. Dosya Yoluna Göre Ne Nerede?

- Pencere açma API'si: `crates/gpui/src/app.rs::open_window`
- Pencere seçenekleri: `crates/gpui/src/platform.rs::WindowOptions`
- Platform penceresi sözleşmesi: `crates/gpui/src/platform.rs::PlatformWindow`
- Pencere wrapper metotları: `crates/gpui/src/window.rs`
- Element ve render trait'leri: `crates/gpui/src/element.rs`, `view.rs`
- Style fluent API: `crates/gpui/src/styled.rs`
- Interactivity fluent API: `crates/gpui/src/elements/div.rs`
- Platform seçimi: `crates/gpui_platform/src/gpui_platform.rs`
- macOS pencere davranışı: `crates/gpui_macos/src/window.rs`
- Windows pencere davranışı: `crates/gpui_windows/src/window.rs`, `events.rs`
- Linux Wayland davranışı: `crates/gpui_linux/src/linux/wayland/window.rs`
- Linux X11 davranışı: `crates/gpui_linux/src/linux/x11/window.rs`
- Zed ana window options: `crates/zed/src/zed.rs::build_window_options`
- Zed platform titlebar: `crates/platform_title_bar/src/platform_title_bar.rs`
- Linux controls: `crates/platform_title_bar/src/platforms/platform_linux.rs`
- Windows controls: `crates/platform_title_bar/src/platforms/platform_windows.rs`
- Workspace client decoration: `crates/workspace/src/workspace.rs::client_side_decorations`
- Zed titlebar composition: `crates/title_bar/src/title_bar.rs`
- Theme background appearance: `crates/theme/src/theme.rs`,
  `crates/theme_settings/src/theme_settings.rs`,
  `crates/settings/src/content_into_gpui.rs`
- UI component export list: `crates/ui/src/components.rs`
- UI input: `crates/ui_input/src/ui_input.rs`, `input_field.rs`

## 28. Yeni Pencere Eklerken Kontrol Listesi

1. Bu pencere workspace mi, modal mı, popup mı? `WindowKind` seç.
2. Ana Zed penceresiyse `build_window_options` kullan.
3. Bounds restore edilecek mi? `WindowBounds` persist et.
4. Hangi display'de açılacak? `display_id` veya `display_uuid` seç.
5. Titlebar native mi custom mı? `TitlebarOptions`/`PlatformTitleBar` kararını ver.
6. Linux decoration modu ayardan mı gelecek? `window_decorations` bağla.
7. Client decoration varsa wrapper, inset, resize handle ve tiling durumunu ekle.
8. Close action doğrudan pencereyi kapatmalı mı, yoksa workspace close flow mu?
9. Blur/transparent gerekiyorsa `window_background` ve root alpha uyumunu kontrol et.
10. Focus başlangıcı doğru mu? `focus`, `show`, `activate_window`, focus handle.
11. Minimum size gerekli mi?
12. App id ve Linux icon gerekiyor mu?
13. macOS native tabbing isteniyor mu? `tabbing_identifier`.
14. Settings/theme değişiminde arka plan güncellenecek mi?
15. Button layout değişiminde titlebar yeniden render olacak mı?
16. Testte timer gerekiyorsa GPUI executor timer kullanıldı mı?

## 29. Kısa Cevaplar

**İleride pencere oluşturmak için nasıl yapmalıyım?**

Workspace penceresi için `zed::build_window_options` ile başla. Özel küçük pencere
için doğrudan `cx.open_window(WindowOptions { ... }, |window, cx| cx.new(...))`
kullan. Root view `Render` implement eden bir `Entity` olmalı.

**Pencere dekorunu nasıl tanımlarım?**

Linux için `WindowOptions.window_decorations = Some(WindowDecorations::Client/Server)`.
Render tarafında fiili sonucu `window.window_decorations()` ile oku. Zed tarzı
client decoration için `workspace::client_side_decorations` kullan. macOS/Windows'ta
custom titlebar için `TitlebarOptions { appears_transparent: true }` veya
`titlebar: None` ve `PlatformTitleBar` kullan.

**Kontrol butonlarını nasıl yönetirim?**

Zed içinde `platform_title_bar::render_left_window_controls` ve
`render_right_window_controls` kullan. Linux'ta `cx.button_layout()` ve
`window.window_controls()` sonucu belirleyicidir. Windows'ta butonlar
`WindowControlArea::{Min, Max, Close}` ile native hit-test'e bağlanır. Close için
workspace akışında `CloseWindow` action dispatch et.

**Blur yönetimini işletim sistemine göre nasıl yaparım?**

Pencere açarken veya tema değişince `window.set_background_appearance(...)` kullan.
Zed tema akışı `opaque`, `transparent`, `blurred` destekler. macOS gerçek blur'u
`NSVisualEffectView`/legacy blur radius ile, Windows composition/DWM ile, Wayland
compositor blur protocol ile uygular. Destek yoksa `Blurred` transparan gibi
davranabilir. Root UI opak çiziyorsa blur görünmez.

**Platform farklarını nerede soyutlarım?**

Davranış platform penceresiyle ilgiliyse GPUI `Platform`/`PlatformWindow` katmanında.
Zed UI görünümüyle ilgiliyse `PlatformStyle::platform()` ve `platform_title_bar`
bileşenlerinde. Ayar farkı gerekiyorsa `settings_content` schema ve `settings`
dönüşümlerinde.

## 30. Animasyon Sistemi

Animasyon API `crates/gpui/src/elements/animation.rs` içindedir.

`Animation` üç alandan oluşur: `duration: Duration`, `oneshot: bool` (false ise
tekrar eder), `easing: Rc<dyn Fn(f32) -> f32>`. Inşa: `Animation::new(duration)`
linear easing ile tek seferlik animasyon oluşturur. `.repeat()` döngüye alır,
`.with_easing(fn)` easing değiştirir.

`AnimationExt` trait, herhangi bir `IntoElement` için iki metot ekler:

```rust
use gpui::{Animation, AnimationExt};
use std::time::Duration;

div()
    .size(px(100.))
    .with_animation(
        "grow",
        Animation::new(Duration::from_millis(500))
            .with_easing(gpui::ease_in_out),
        |el, delta| el.size(px(100. + 100. * delta)),
    )
```

Çoklu animasyon zinciri için `with_animations(id, vec![anim_a, anim_b], |el, ix, delta| ...)`.

Yerleşik easing fonksiyonları (`crates/gpui/src/elements/animation.rs:211+`):
`linear`, `quadratic`, `ease_in_out`, `ease_out_quint()`, `bounce(inner)`,
`pulsating_between(min, max)`. `pulsating_between` yön değiştirerek değer döndürür
(loading indicator için ideal; `Animation::repeat()` ile birleştirilir).

Tuzaklar:

- Element ID render boyunca stabil olmalıdır; değişirse animasyon state sıfırlanır.
- Animator closure `'static` olduğundan dış state'i `Rc`/`Arc`/`clone` ile yakala.
- Repeat animasyonu sürekli `cx.notify()` çağırır; gerekmiyorsa oneshot bırak.
- Frame'ler arası progress değeri executor saatinden hesaplanır; testlerde
  `cx.background_executor.advance_clock(...)` ile ilerlet.

## 31. Renkler, Gradient ve Background

`crates/gpui/src/color.rs` ve `colors.rs`.

İki temel tip:

- `Rgba { r, g, b, a }`: 0.0–1.0 aralığında bileşenler.
- `Hsla { h, s, l, a }`: 0.0–1.0 aralığında bileşenler.

Constructor'lar:

```rust
let red = rgb(0xff0000);                    // Rgba, alfa 1.0
let translucent = rgba(0xff000080);         // 0xRRGGBBAA
let h = hsla(0.0, 1.0, 0.5, 1.0);           // saf kırmızı
let grey = opaque_grey(0.5, 1.0);           // gri yardımcısı
```

Sık kullanılan metotlar (`color.rs:472+`):

- `is_transparent()`, `is_opaque()`
- `opacity(factor)`: alfayı çarpar.
- `alpha(a)`: alfayı doğrudan ayarlar.
- `fade_out(factor)`: in-place alfa azaltma.
- `blend(other)`: pre-multiplied alpha ile karıştırır.
- `grayscale()`: doygunluğu sıfırlar.
- `to_rgb()`: Hsla → Rgba.

Background tipi (`color.rs:763+`) sadece düz renk değildir:

```rust
solid_background(rgb(0xffffff))
linear_gradient(
    angle_deg,
    linear_color_stop(rgb(0x000000), 0.0),
    linear_color_stop(rgb(0xffffff), 1.0),
)
checkerboard(rgb(0xeeeeee), 8.0)
pattern_slash(rgb(0xff0000), width = 2.0, interval = 6.0)
```

`linear_gradient(...).color_space(ColorSpace::Oklab)` ile renk uzayı seçilebilir;
`opacity(factor)` her stop'a uygulanır.

`.bg(impl Into<Background>)` her style fluent API'sinde mevcut. Düz `Hsla` da
`Into<Background>` implement eder, bu yüzden `.bg(theme.colors().panel_background)`
tipik kullanımdır.

Pratik notlar:

- Alfa = 0 fakat opaque arka planın üzerine çiziyorsan temadaki opak rengi tercih et.
- Gradient stop'lar `0.0` ve `1.0` arasında sıralı vermeli; aksi halde GPU shader'ı
  beklenmedik dağılım verebilir.
- Hsla'da hue 1.0'a sarılmaz (clamp'lenir); rotasyon için `hue + delta` modulo 1.0
  ile hesapla.

## 32. ScrollHandle ve Scroll Davranışı

`crates/gpui/src/elements/div.rs:3387+`.

`ScrollHandle`, scroll offset'ini paylaşılabilir bir handle olarak temsil eder.
`Rc<RefCell<ScrollHandleState>>` üzerinden çalışır, view'lar arasında klonlanabilir.

Public API:

- `ScrollHandle::new()`
- `offset() -> Point<Pixels>`: anlık scroll konumu.
- `max_offset() -> Point<Pixels>`
- `top_item()`, `bottom_item()`: görünür ilk/son child item dizini.
- `bounds()`: scroll container bounds.
- `bounds_for_item(ix)`: child bounds.
- `scroll_to_item(ix)`, `scroll_to_top_of_item(ix)`: prepaint zamanında istenen
  item'a scroll eder.
- `scroll_to_bottom()`

Element üzerine bağlama:

```rust
let handle = ScrollHandle::new();

div()
    .id("list")
    .overflow_y_scroll()
    .track_scroll(&handle)
    .child(/* ... */)
```

`overflow_scroll`, `overflow_x_scroll`, `overflow_y_scroll` üçü de Style'a sahiptir;
overflow `Scroll` olduğunda input wheel/touch event'i bu container içinde tüketilir.
`track_scroll` aynı handle'ı render geçişleri arasında bağlar; aynı handle başka
yerden okunabilir.

`ScrollAnchor` (`div.rs:3332+`) bir handle ile çalışan helper'dır; immediate child
olmasa bile belirli bir element'in görünür kalmasını ister:

```rust
let anchor = ScrollAnchor::for_handle(handle.clone());
anchor.scroll_to(window, cx);
```

Tuzaklar:

- `id(...)` çağırmadan `overflow_*_scroll` çalışmaz; element interaktif değildir.
- `track_scroll` çağırılmadan handle değerleri eski kalır; offset güncel olmaz.
- Klavye ile scroll dispatch için `.on_key_down(...)` veya action ile
  `scroll_to_item` çağrılır; otomatik klavye scroll yoktur.

## 33. Asset, Image ve SVG Yükleme

`crates/gpui/src/asset_cache.rs`, `assets.rs`, `elements/img.rs`, `svg.rs`.

`Asset` trait async loader sözleşmesidir:

```rust
trait Asset {
    type Source: ...;
    type Output: ...;
    fn load(source: Self::Source, cx: &mut App) -> impl Future<Output = Self::Output>;
}
```

Kaynak gösterimi:

- `Resource::Path(Arc<Path>)`
- `Resource::Uri(SharedUri)` — `http://`, `https://`, `file://` vb.
- `Resource::Embedded(SharedString)` — `AssetSource` üzerinden gömülü asset.

`AssetSource` trait `App::with_assets` ile kurulan global asset providerdır.
`crates/assets` Zed binary'sinde `RustEmbed` ile SVG/icons dahil eder.

Image element:

```rust
img("path/to/icon.png")
    .w(px(24.))
    .h(px(24.))
    .object_fit(ObjectFit::Contain)
    .with_fallback(|| div().bg(rgb(0xeeeeee)))
    .on_load(cx.listener(|_, _, _, cx| cx.notify()))
```

`img(impl Into<ImageSource>)` kabul eder. `ImageSource`:

- `Resource(Resource)`
- `Render(Arc<RenderImage>)` — önceden raster edilmiş frame'ler.
- `Image(Arc<Image>)` — encoded bytes (PNG/JPEG/WebP).
- `Custom(Arc<dyn Fn(&mut App) -> Task<...>>)`

URL string otomatik `Uri` olarak parse edilir; path string `Path` olur.

SVG:

```rust
svg().path("icons/check.svg").size(px(16.)).text_color(rgb(0x000000))
```

SVG path `AssetSource`'tan okunur. `text_color` SVG'deki `currentColor` referanslarını
boyamak için kullanılır. Custom path string yerine derive edilen `IconName::path()`
de geçirilebilir (Zed'de `Icon::new(IconName::Check)` doğrudan kullanılır).

Cache davranışı: aynı `Resource` farklı kullanıcılar için tek seferde yüklenir;
load Task `Arc` ile paylaşılır. Hata loglama `ImgResourceLoader = AssetLogger<...>`
ile otomatiktir.

Tuzaklar:

- URL parse başarısızsa load sessizce başarısız olur (ImgResourceLoader log'a düşer).
- Custom closure `'static` olmalı; `cx`'i closure dışında klonlayıp taşı.
- `with_fallback` yalnızca yükleme tamamlandığında ve hatalıysa fallback render eder.
- `RenderImage` GIF/animated PNG için `frames()` ile frame loop yönetilir.

## 34. Anchored ve Popover Konumlandırma

`crates/gpui/src/elements/anchored.rs`.

`anchored()` fonksiyonu bir `Anchored` builder döndürür:

```rust
anchored()
    .anchor(Corner::TopLeft)
    .position(point(px(120.), px(80.)))
    .offset(point(px(0.), px(4.)))
    .snap_to_window_with_margin(Edges::all(px(8.)))
    .child(menu_view.into_any_element())
```

API:

- `anchor(Corner)`: child'ın hangi köşesinin `position`'a hizalanacağı.
- `position(point)`: anchor noktası (window veya local koordinatlarda).
- `offset(point)`: hizalama sonrası ek kayma.
- `position_mode(AnchoredPositionMode::Window | Local)`: koordinat referansı.
- `snap_to_window()` ve `snap_to_window_with_margin(Edges)`: pencere dışına taşıyorsa
  sığacak şekilde kaydırır veya köşeyi flip eder.

`AnchoredFitMode`:

- `SwitchAnchor` (default): yetersiz alanda tersine flip.
- `SnapToWindow`: aynı köşede kalır, pencere kenarına oturur.
- `SnapToWindowWithMargin(Edges)`: marjin bırakarak oturur.

Anchored element ağaca normal çocuk gibi eklenir, fakat layout fazında parent
bounds'unu yok sayar; absolute positioning gibi davranır. Tooltip, popover ve
ContextMenu altta bu element ile çalışır.

Tuzaklar:

- Position `Local` modda parent'ın content origin'ine görelidir; window modda
  ekranda mutlak değil, **pencere içi** koordinatlardır.
- Snap fonksiyonları arasında en son çağrılan kazanır.
- Anchored child kendi içinde overflow `Visible` davranır; içerik pencereyi taşırsa
  scroll için ekstra wrapper gerekir.

## 35. Geometri Tipleri ve Birim Yönetimi

`crates/gpui/src/geometry.rs`.

GPUI üç farklı pixel birimi kullanır:

- `Pixels(f32)`: scale-bağımsız mantıksal piksel. UI kodunda neredeyse her zaman
  bu kullanılır.
- `ScaledPixels(f32)`: `Pixels * window.scale_factor()`. Renderer'a iletilen değer.
- `DevicePixels(i32)`: fiziksel cihaz pikseli. Asset/texture boyutlarında kullanılır.

Yardımcılar:

```rust
let p = px(12.0);                  // Pixels
let r = rems(1.5);                 // Rems
let p2 = point(px(10.), px(20.));  // Point<Pixels>
let s = size(px(100.), px(40.));   // Size<Pixels>
let b = Bounds::from_corners(point(px(0.), px(0.)), point(px(100.), px(100.)));
```

Diğer birimler:

- `Rems(f32)`: kök font boyutuna görelidir (Zed'de `theme.ui_font_size` ile bağlı).
  `.text_sm()`, `.gap_2()` gibi makro üretilen helper'lar genelde Rems üzerinden
  Pixels üretir.
- `AbsoluteLength`: `Pixels` veya `Rems`.
- `DefiniteLength`: `Absolute(AbsoluteLength)` veya `Fraction(f32)`.
- `Length`: `Definite(DefiniteLength)` veya `Auto`.

Stil API'leri Length kabul eder:

```rust
div().w(px(120.))           // Pixels
    .min_h(rems(2.))        // Rems
    .flex_basis(relative(0.5)) // Fraction
    .h_auto()
```

Generic container'lar `Point<T>`, `Size<T>`, `Bounds<T>`, `Edges<T>`, `Corners<T>`
çoğu metot için aritmetik destekler (`+`, `-`, `*`, `/`).

Tuzaklar:

- `Bounds::contains(point)` half-open intervallere göre çalışır; sınır pikseli
  `false` dönebilir.
- `Pixels` ile `ScaledPixels` aritmetiği `From`/`Into` üzerinden açık konversiyon
  ister; örtük çevrilmez.
- `point(x, y)` argument sırası önce X sonra Y'dir; `size(width, height)` de aynı.

## 36. Path Çizimi ve Custom Drawing

`crates/gpui/src/path_builder.rs`, `scene.rs`, `elements/canvas.rs`.

GPUI doğrudan path API yerine `canvas` elementi ve `PathBuilder` ile vektör çizim
sağlar. `PathBuilder` lyon tessellator wrapper'ıdır.

```rust
canvas(
    |bounds, window, cx| {
        // prepaint: hitbox, layout-zamanlı state
        window.insert_hitbox(bounds, HitboxBehavior::Normal)
    },
    |bounds, hitbox, window, cx| {
        // paint: window.paint_path(...) çağrıları
        let mut path = PathBuilder::fill();
        path.move_to(bounds.origin);
        path.line_to(bounds.origin + point(px(0.), bounds.size.height));
        path.line_to(bounds.origin + bounds.size);
        path.close();
        if let Ok(built) = path.build() {
            window.paint_path(built, rgb(0x4f46e5));
        }
    },
)
.size_full()
```

`PathBuilder`:

- `PathBuilder::fill()` ya da `PathBuilder::stroke(width)` ile başlat.
- `move_to(point)`, `line_to(point)`, `cubic_bezier_to(c1, c2, end)`,
  `quadratic_bezier_to(c, end)`, `arc(...)`, `close()`.
- `build()` → tessellated `Path<Pixels>` döner; `?` ile hata yay.

Window paint API'leri:

- `window.paint_path(path, color)`
- `window.paint_quad(quad)`: `fill(bounds, ...).border(...)` shorthand.
- `window.paint_strikethrough(...)`, `paint_underline(...)`
- `window.paint_image(...)`: raster image draw.
- `window.paint_layer(bounds, |window| ...)`: alt katman, clipping ve transform.

Tuzaklar:

- Tessellation pahalıdır; her frame yeni path inşa etmek FPS'i düşürür. Mümkünse
  prepaint'te build edip paint'te yalnızca çiz.
- Path bounds dışına taşan kısım clip'lenmez; `paint_layer` ile manuel clipping yap.
- Stroke genişliği logical Pixels'dir; DPI yüksek ekranda çok ince kalmasın diye
  `px(1.0).max(...)` ile zemin tut.

## 37. Uygulama Menüsü ve Dock

`crates/gpui/src/platform/app_menu.rs`.

Tipler:

- `Menu { name, items, disabled }`
- `MenuItem`:
  - `Separator`
  - `Submenu(Menu)`
  - `SystemMenu(OsMenu)` — macOS Services gibi sistem submenu'leri.
  - `Action { name, action, os_action, checked, disabled }`
- `OsAction`: `Cut`, `Copy`, `Paste`, `SelectAll`, `Undo`, `Redo`. Native edit
  menu eşlemesi için.

Builder örneği:

```rust
cx.set_menus(vec![
    Menu::new("Zed").items([
        MenuItem::action("About Zed", zed::About),
        MenuItem::Separator,
        MenuItem::action("Quit", workspace::Quit),
    ]),
    Menu::new("Edit").items([
        MenuItem::os_action("Undo", editor::Undo, OsAction::Undo),
        MenuItem::os_action("Redo", editor::Redo, OsAction::Redo),
        MenuItem::Separator,
        MenuItem::os_action("Cut", editor::Cut, OsAction::Cut),
        MenuItem::os_action("Copy", editor::Copy, OsAction::Copy),
        MenuItem::os_action("Paste", editor::Paste, OsAction::Paste),
        MenuItem::os_action("Select All", editor::SelectAll, OsAction::SelectAll),
    ]),
]);
```

`MenuItem::action(name, action)` veri taşımayan unit struct action'lar için kısayoldur;
veri taşıyan action'lar `Action::boxed_clone()` ile manuel build edilmelidir.

Diğer menü API'leri (`App` üzerinde):

- `cx.set_dock_menu(Menu)` — macOS dock right-click menüsü.
- `cx.add_recent_document(path)` — macOS recent items.
- `cx.update_jump_list(JumpListCategory::Recent, entries)` — Windows jump list,
  macOS recent stub.
- `cx.get_menus()` — şu anda set edili menü modelini okur.

Platform davranışı:

- macOS native `NSMenu` ile çizilir; klavye kısayolları binding'lerden okunur.
- Windows ana pencerede menubar; alt+key access keys otomatik yapılmaz.
- Linux backend'i çoğunlukla no-op'tur; menülerin uygulama içinde çizilmesi gerekir
  (Zed'de dropdown menubar Linux'ta kullanılır).

Tuzak: Aynı action birden çok menü item'a bağlanırsa keymap'te tek shortcut
gösterilir. `os_action` yalnızca macOS native edit menu eşlemesini etkiler;
diğer platformlarda alelade action gibidir.

## 38. Action Sistemi Derinlemesine

`crates/gpui/src/action.rs`, `key_dispatch.rs`.

Action tanımının iki ana yolu vardır.

Veri taşımayan action:

```rust
use gpui::actions;
actions!(my_namespace, [Save, Close, Reload]);
```

`actions!` makrosu her isim için unit struct ve `Action` impl üretir; namespace
`my_namespace::Save` adıyla registery'e kaydolur.

Veri taşıyan action:

```rust
use gpui::Action;

#[derive(Clone, PartialEq, Action)]
#[action(namespace = editor)]
pub struct GoToLine { pub line: u32 }
```

`#[action(namespace = ..., name = "...", no_json, deprecated_aliases = [...])]`
attribute'leri kontrol sağlar. Default olarak `Deserialize` derive edilmesi
beklenir; pure code action için `no_json` kullan.

Dispatch:

- `window.dispatch_action(BoxedAction, cx)`: focused element'ten root'a doğru
  bubble.
- `focus_handle.dispatch_action(&action, window, cx)`: belirli handle'dan başlatır.
- Keymap girdileri eşleştiğinde otomatik dispatch edilir.

Listener kaydı:

```rust
.on_action(cx.listener(|this, action: &GoToLine, window, cx| {
    this.go_to(action.line);
    cx.notify();
}))
.capture_action(cx.listener(handler)) // capture phase
```

`DispatchPhase`:

- `Capture`: root'tan focused element'e doğru.
- `Bubble`: focused element'ten root'a doğru. Default; action handler'lar burada
  default olarak propagation'ı durdurur. Aksi gerekiyorsa içinde `cx.propagate()`.

Keybinding:

```rust
cx.bind_keys([
    KeyBinding::new("cmd-s", Save, Some("Workspace")),
    KeyBinding::new("ctrl-g", GoToLine { line: 0 }, Some("Editor")),
]);
```

Context predicate gramer (`crates/gpui/src/keymap/context.rs`):

- `Editor` — context stack'te `Editor` değeri var.
- `Editor && !ReadOnly`
- `pane == focused`
- `os == macos`
- `vim_mode in (normal, visual)`

`.key_context("Editor")` ile element ağaca context push eder; child'lar üst
context'leri görür. Aynı binding birden çok context'te match ederse en
spesifik (en derin) context kazanır.

Tuzaklar:

- Action register edilmeden binding atılırsa keymap parse hata verir; `actions!`
  veya `#[derive(Action)]` mutlaka ana modülde derlenmiş olmalı.
- Bubble fazında handler `cx.propagate()` çağırmazsa parent action handler'lara
  ulaşmaz (default davranış).
- Aynı action ismi iki crate'te tanımlanırsa registry collision olur; namespace zorunlu.
- Zed runtime'da bilinmeyen action ismi keymap'te warning log üretir, panic değil.

## 39. Tab Sırası ve Klavye Navigasyonu

`crates/gpui/src/tab_stop.rs`, `window.rs:397`.

Tab navigasyonu `FocusHandle` üzerindeki iki bayrakla kontrol edilir:

```rust
let handle = cx.focus_handle()
    .tab_stop(true)        // Tab tuşuyla durulabilir
    .tab_index(0);         // Pozitif: sıraya katılır, negatif: atlanır
```

Tab traversal sırası TabStopMap içindeki node sıralamasına göre belirlenir:

1. Aynı grup içinde `tab_index` küçükten büyüğe.
2. `tab_index` eşitse element ağaç sırası (DFS).
3. `tab_stop(false)` olan veya `tab_index < 0` olan handle atlanır.

Grup oluşturmak için elementlerin `.begin_group(index)` / `.end_group()`
operasyonları olur (`tab_stop.rs:92`); tipik kullanım Zed UI'da Pane ve Modal
seviyesindedir.

Window üzerindeki yardımcılar:

- `window.focus_next()` / `window.focus_prev()`: Tab/Shift-Tab sırasında çağrılır.
- `window.focused(cx)`: o anki odak handle'ı.

Custom input bileşeni yazıyorsan:

```rust
div()
    .track_focus(&self.focus_handle)
    .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| { ... }))
    .child(/* ... */)
```

`tab_stop(true)` olmadan handle yalnızca programatik focus alır; klavyeyle
ulaşılamaz. Aksesibilite ve form akışı için her interaktif element bir handle'a
sahip olmalı.

## 40. Test Bağlamları ve Simülasyon

`crates/gpui/src/app/test_context.rs`, `visual_test_context.rs`.

`#[gpui::test]` makrosu bir `TestAppContext` sağlar. Görsel test için
`add_window` bir `WindowHandle<V>` döndürür ve `VisualTestContext` ile sürülür.

```rust
#[gpui::test]
fn test_save(cx: &mut TestAppContext) {
    let window = cx.add_window(|window, cx| cx.new(|cx| Editor::new(window, cx)));

    cx.simulate_keystrokes("cmd-s");
    cx.run_until_parked();

    window.read_with(cx, |editor, _| {
        assert!(editor.is_clean);
    });
}
```

Sık kullanılan API'ler:

- `cx.add_window(|window, cx| cx.new(...))`: yeni offscreen pencere.
- `cx.simulate_keystrokes("cmd-s left")`: boşlukla ayrılmış keystroke dizisi.
- `cx.simulate_input("hello")`: text input simulasyonu (IME yolu üzerinden).
- `cx.simulate_mouse_move(point, modifiers)`,
  `cx.simulate_mouse_down(button, point, modifiers)`,
  `cx.simulate_click(point, modifiers)`.
- `cx.dispatch_action(window, &action)`.
- `cx.run_until_parked()`: tüm pending future/task tamamlanıncaya kadar sürer.
- `cx.background_executor.advance_clock(duration)`: deterministic timer ilerletme.
- `cx.background_executor.run_until_parked()`: test executor'ında yalnızca background.
- `window.update(cx, |view, window, cx| ...)`: pencere içi state mutate.

`VisualTestContext` (`add_window` sonucu `cx.into()` ile elde edilir) `Window`
referansını içerir; render output testleri ve element bounds doğrulaması için.

Pratik kurallar:

- Real tutarlılık için `smol::Timer` yerine `cx.background_executor.timer(d)` kullan.
- `run_until_parked` ile `advance_clock` kombine edilirken önce clock ilerlet,
  sonra park bekle.
- Async test için `#[gpui::test]` `async fn(cx: &mut TestAppContext)` formunu
  destekler; foreground task'ları orada `cx.spawn` ile kur.
- Pencerenin gerçekten render edilmesi için `cx.draw(window)` veya `run_until_parked`
  zorunlu olabilir; aksi halde layout `viewport_size` boyut sıfır olur.

Tuzaklar:

- `simulate_keystrokes` action dispatch'i tetikler ama keymap binding kaydedilmiş
  olmalıdır; testte `cx.bind_keys([...])` çağırmayı unutma.
- `run_until_parked` zaman ilerletmez; sadece pending future'ları sürer. Timer
  beklenirken `advance_clock` şart.
- `dispatch_action` focus tree'de action handler bulamazsa sessizce no-op'tur;
  view'in gerçekten focused olduğundan emin ol.

## 41. Pencere Bounds Persist ve Restore

`crates/gpui/src/platform.rs::WindowBounds`, Zed tarafında
`crates/workspace/src/persistence.rs` ve `crates/zed/src/zed.rs`.

`WindowBounds` enum üç durumu kapsar:

```rust
pub enum WindowBounds {
    Windowed(Bounds<Pixels>),
    Maximized(Bounds<Pixels>),
    Fullscreen(Bounds<Pixels>),
}
```

`Bounds` her durumda restore-ready koordinatları taşır; `Maximized`/`Fullscreen`
içindeki bounds, durum kapatıldığında dönülecek windowed bounds'tır.

Persist akışı:

```rust
let bounds = window.window_bounds();
serialize(bounds, display_uuid);
```

`window.window_bounds()` her zaman restore için kullanılacak `WindowBounds` döner;
fullscreen iken bile underlying windowed bounds gelir. Display UUID'si ayrı saklanır
çünkü kullanıcı sonradan monitor'ü ayırabilir.

Restore akışı `zed::build_window_options` içinde:

1. Saklı `display_uuid` ile `cx.find_display(uuid)` çağrılır.
2. Display bulunduysa kayıtlı `Bounds` ve durum kullanılır.
3. Bulunmazsa default display ve `WindowBounds::centered(...)` fallback.

Bounds değişimini izlemek için:

```rust
cx.observe_window_bounds(window, |this, window, cx| {
    let bounds = window.window_bounds();
    this.persist_bounds(bounds);
}).detach();
```

Aynı şekilde `cx.observe_window_appearance(window, ...)` light/dark değişimini,
`cx.observe_window_activation(window, ...)` foreground/background değişimini izler.

Tuzaklar:

- `window.bounds()` (live screen rect) ve `window.window_bounds()` (restore-ready)
  farklıdır; persist için her zaman ikincisi.
- Maximized iken `Bounds<Pixels>` muhtemelen ekran boyutu kadardır; restore sonrası
  windowed durumda aynı boyutta açılmaz, çünkü `window_bounds` kullanıcı son
  windowed bounds'unu hatırlar.
- Display UUID'si Linux/Wayland'de boş olabilir (`display.uuid().ok()` None döner);
  fallback gerekli.

## 42. Inspector ve Debug Yardımcıları

`crates/gpui/src/inspector.rs` (feature: `inspector`).

`gpui` crate'i `inspector` feature ile derlendiğinde dev tool entegrasyonu sağlar:

- `InspectorElementId`: her element için `(file, line, instance)` tabanlı kimlik.
- Element source location `#[track_caller]` ile yakalanır.
- Element seçimi window'da `Inspector` global state üzerinden tetiklenir.
- `Window::with_inspector_state(...)`, `Window::register_inspector_element(...)`
  ile bileşenler kendi state'lerini inspector UI'a açar.

Production build'de inspector kodu sıfır maliyetlidir; release Zed binary'sinde
dev tooling yoktur.

Diğer debug helper'ları:

- `div().debug_selector(|| "my-button")`: test ve inspector'da selector ata.
- `gpui::profile_label!("phase")` ve `crates/gpui/src/profiler.rs`: thread/task
  zamanlama profili.
- `RUST_LOG=gpui=debug` ile event/key dispatch log seviyesi yükselir.
- `ZED_INSPECT_BOUNDS=1` env: bounds debug overlay (workspace seviyesinde).

## 43. Subscription Yaşam Döngüsü

`crates/gpui/src/subscription.rs`.

`Subscription` opaque tiptir; düşürüldüğünde callback kaydını siler. Pratikte
üç desen vardır:

```rust
// 1. Field'da sakla
struct View { _subs: Vec<Subscription> }
// new(): self._subs.push(cx.subscribe(...));

// 2. Detach (callback view ömrü boyunca yaşar)
cx.subscribe(&entity, |...| { ... }).detach();

// 3. Geçici scope (drop sonrası unsubscribe)
let _sub = cx.observe(&entity, |...| { ... });
// _sub düştüğünde callback kaldırılır
```

Subscription üreten yöntemler (`Context<T>` üzerinde):

- `cx.observe(entity, f)`: `cx.notify()` çağrıldığında fire eder.
- `cx.subscribe(entity, f)`: `EventEmitter<E>` event'leri için.
- `cx.observe_global::<G>(f)`: global state değişti.
- `cx.observe_release(entity, f)`: entity drop edildi.
- `cx.on_focus(handle, window, f)` / `cx.on_blur(...)` / `cx.on_focus_in(...)` /
  `cx.on_focus_out(...)` / `cx.on_focus_lost(window, f)`.
- `cx.observe_window_bounds`, `cx.observe_window_activation`,
  `cx.observe_window_appearance`, `cx.observe_button_layout_changed`,
  `cx.observe_pending_input`, `cx.observe_keystrokes`.

Tuzaklar:

- `detach()` uzun yaşayan callback'i view ömründen koparır; view drop olduktan
  sonra hâlâ fire ederse `WeakEntity` ile koru.
- Subscription drop sırası önemli olabilir; `Vec<Subscription>` field'ında saklanan
  abonelikler struct drop sırasında aşağıdan yukarı kaldırılır.
- `observe` sırasında entity'yi update etmek panic verir; `cx.spawn(..)` ile
  ertele veya `cx.defer(|cx| ...)` kullan.

## 44. Hızlı Referans: GPUI Kavram Sözlüğü

| Kavram | Tip | Yer | Kısa Açıklama |
|---|---|---|---|
| Application | `Application` | `gpui_platform` | Platform seçer ve event loop'u sürer. |
| Root context | `App` | `app.rs` | Global state, window, entity create. |
| Entity | `Entity<T>` | `app/entity_map.rs` | Heap-allocated state handle. |
| Weak handle | `WeakEntity<T>` | aynı | Cycle önleyici zayıf handle. |
| Update context | `Context<T>` | `app.rs` | Entity update'inde, App'e deref. |
| Async context | `AsyncApp` | `app/async_context.rs` | Await boyu tutulan context. |
| Pencere | `Window` | `window.rs` | Tek pencere durumu. |
| Window handle | `WindowHandle<V>` | `window.rs` | View tipini bilen window referansı. |
| Future task | `Task<T>` | `executor.rs` | Drop'ta iptal eden future. |
| Subscription | `Subscription` | `subscription.rs` | Drop'ta unsubscribe. |
| Element | `impl Element` | `element.rs` | Layout + paint sözleşmesi. |
| View | `impl Render` | `view.rs` | Stateful element ağacı üreten entity. |
| Action | `impl Action` | `action.rs` | Dispatch tree mesajı. |
| Focus handle | `FocusHandle` | `window.rs` | Focus ve tab navigasyon kimliği. |
| Hitbox | `Hitbox` | `window.rs` | Mouse hit-test alanı. |
| ScrollHandle | `ScrollHandle` | `elements/div.rs` | Paylaşılan scroll state. |
| Animation | `Animation` | `elements/animation.rs` | Süre/easing tabanlı interpolation. |
| Asset source | `AssetSource` trait | `assets.rs` | Asset bytes provider. |
| Color | `Hsla`/`Rgba` | `color.rs` | UI renk tipleri. |
| Pixels | `Pixels` | `geometry.rs` | Mantıksal piksel. |
| Background | `Background` | `color.rs` | Solid/gradient/pattern fill. |
| Keymap | `Keymap` | `keymap/` | Bağlam-duyarlı keybinding tablosu. |
| Global | `impl Global` | `global.rs` | Tek instance app-genel state. |
| Event emitter | `EventEmitter<E>` | `app.rs` | Entity event yayınlayıcı. |
