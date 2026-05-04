# kavis-ui / gpui-component 0.5.1 Initial Fork Map

Bu not, Kavis UI'nin ilk `gpui-component` baz alma ve uyarlama dalgası için yol haritasıdır. Gelecekte upstream karşılaştırması yapılırken önce bu dosya okunmalı; 2026-05-04 gibi sonraki tarihli küçük merge haritaları bunun üzerine uygulanmalıdır.

## Kapsam

- Kaynak upstream: `longbridge/gpui-component`
- Kaynak tag: `v0.5.1`
- Kaynak commit: `0f0ab352` (`Bump v0.5.1`)
- Kavis ilk import commit: `4374573` (`ilk kommit`, 2026-05-02)
- İlk uyarlama/stabilizasyon aralığı: `4374573..1b3816e`
- Sonraki bağımsız upstream güncelleme haritası: `docs/gpui-component-2026-05-04-merge-map.md`

## Ana Karar

Kavis UI, `gpui-component 0.5.1` üzerine kurulmuş sert bir çataldır. Amaç upstream ile birebir eş kodlama değil; GPUI üzerinde Türkçe API yüzeyi, yerel Zed bağımlılıkları, Türkçe dokümantasyon ve Kavis'e uygun story/web deneyimi sağlamaktır.

## Paket ve Workspace Haritası

| Upstream | Kavis | Not |
| --- | --- | --- |
| `gpui-component` | `kavis-ui` | Ana UI crate adı değişti. |
| `gpui-component-macros` | `kavis-ui-macros` | Makro crate adı değişti. |
| `gpui-component-assets` | `kavis-ui-assets` | Varlık crate adı değişti. |
| `gpui-component-story` | `kavis-ui-story` | Story crate adı değişti. |
| Yok | `kavis-ui-story-web` | WASM galeri için yeni crate eklendi. |
| `crates/reqwest_client` | Yok | Upstream kopyası kullanılmıyor; Zed path bağımlılığı tercih edildi. |
| `crates/ui/src/webview.rs` | `crates/webview` | Webview deneysel ayrı crate olarak tutuluyor. |
| `0.5.1` | `0.1.0` | Kavis bağımsız erken sürüm çizgisine geçti. |

## Bağımlılık Politikası

- Upstream `gpui = "0.2.2"` ve `gpui-macros = "0.2.2"` kullanıyordu.
- Kavis tarafında GPUI, Zed checkout'undan path bağımlılığı olarak kullanılır:
  - `../zed/crates/gpui`
  - `../zed/crates/gpui_platform`
  - `../zed/crates/gpui_web`
  - `../zed/crates/gpui_macros`
- `reqwest_client` de Zed path bağımlılığıdır.
- `reqwest` için Zed'in git fork'u kullanılır.
- WASM story-web için `getrandom` wasm desteği, `errno` stub'u ve `psm` patch'i korunur.
- `docs.rs` ve `crates.io` yayın yüzeyi bilinçli olarak kapalıdır; yayın açılana kadar upstream metadata geri taşınmamalıdır.

## Türkçe API Yüzeyi

İlk uyarlamanın ana yönü, public kullanımda Türkçe adları tercih etmektir. Upstream kod taşırken İngilizce sembolleri doğrudan bırakmadan önce kavis-ui karşılığı aranmalıdır.

| Upstream kavramı | Kavis karşılığı | Dosya / not |
| --- | --- | --- |
| `Button` | `Dugme` | `crates/ui/src/button` |
| `ButtonGroup` | `DugmeGrubu` | `crates/ui/src/button/button_group.rs` |
| `Icon` | `Simge` | `crates/ui/src/icon.rs` |
| `IconName` | `SimgeAdi` | `crates/ui/src/icon.rs` |
| `Theme` | `Tema` | `crates/ui/src/theme` |
| `ActiveTheme` | `EtkinTema` | `crates/ui/src/theme` |
| `Root` | `KokGorunum` | `crates/ui/src/root.rs` |
| `Application` / `App` | `UygulamaKurulumu` / `Uygulama` | `crates/ui/src/cekirdek.rs` |
| `Window` | `Pencere` | `crates/ui/src/cekirdek.rs` |
| `Entity<T>` | `Varlik<T>` | `crates/ui/src/cekirdek.rs` |
| `SharedString` | `PaylasimliMetin` | `crates/ui/src/cekirdek.rs` |
| GPUI raw escape hatch | `ham_gpui` | `crates/ui/src/lib.rs` |
| Turkish GPUI aliases | `turkce` | `crates/ui/src/lib.rs`, `gpui_turkce.rs` |

## Eklenen Kavis Katmanları

- `crates/ui/src/cekirdek.rs`: uygulama/pencere/entity/type alias katmanı, `KavisMotoru`, `PencereAyarlari`.
- `crates/ui/src/bilesenler.rs`: bileşenlerin toplu Türkçe dışa aktarım noktası.
- `crates/ui/src/bilesenler/buton.rs`: `KavisButon` gibi Kavis'e özel üst seviye sarmalayıcılar.
- `crates/ui/src/gpui_turkce.rs`: GPUI için Türkçe alias yüzeyi.
- `crates/story-web`: WASM tabanlı story galeri.
- `crates/webview`: webview deneysel yüzeyinin ayrı crate'e alınması.
- Ek ikonlar ve örnekler: sistem izleme ikonları, `system_monitor`, `focus_trap`, `tooltip_top_edge`, `webview` örnekleri.

## Modül Dosya Adı Kararı

İlk uyarlamada `mod.rs` kullanımı kaldırıldı. Upstreamden dosya taşırken en sık kaçırılan nokta budur.

| Upstream yolu | Kavis yolu |
| --- | --- |
| `crates/ui/src/avatar/mod.rs` | `crates/ui/src/avatar.rs` |
| `crates/ui/src/button/mod.rs` | `crates/ui/src/button.rs` |
| `crates/ui/src/chart/mod.rs` | `crates/ui/src/chart.rs` |
| `crates/ui/src/dialog/mod.rs` | `crates/ui/src/dialog.rs` |
| `crates/ui/src/dock/mod.rs` | `crates/ui/src/dock.rs` |
| `crates/ui/src/form/mod.rs` | `crates/ui/src/form.rs` |
| `crates/ui/src/highlighter/mod.rs` | `crates/ui/src/highlighter.rs` |
| `crates/ui/src/input/mod.rs` | `crates/ui/src/input.rs` |
| `crates/ui/src/input/display_map/mod.rs` | `crates/ui/src/input/display_map.rs` |
| `crates/ui/src/input/lsp/mod.rs` | `crates/ui/src/input/lsp.rs` |
| `crates/ui/src/input/popovers/mod.rs` | `crates/ui/src/input/popovers.rs` |
| `crates/ui/src/list/mod.rs` | `crates/ui/src/list.rs` |
| `crates/ui/src/menu/mod.rs` | `crates/ui/src/menu.rs` |
| `crates/ui/src/plot/mod.rs` | `crates/ui/src/plot.rs` |
| `crates/ui/src/progress/mod.rs` | `crates/ui/src/progress.rs` |
| `crates/ui/src/resizable/mod.rs` | `crates/ui/src/resizable.rs` |
| `crates/ui/src/scroll/mod.rs` | `crates/ui/src/scroll.rs` |
| `crates/ui/src/setting/mod.rs` | `crates/ui/src/setting.rs` |
| `crates/ui/src/setting/fields/mod.rs` | `crates/ui/src/setting/fields.rs` |
| `crates/ui/src/sidebar/mod.rs` | `crates/ui/src/sidebar.rs` |
| `crates/ui/src/stepper/mod.rs` | `crates/ui/src/stepper.rs` |
| `crates/ui/src/tab/mod.rs` | `crates/ui/src/tab.rs` |
| `crates/ui/src/table/mod.rs` | `crates/ui/src/table.rs` |
| `crates/ui/src/text/mod.rs` | `crates/ui/src/text.rs` |
| `crates/ui/src/text/format/mod.rs` | `crates/ui/src/text/format.rs` |
| `crates/ui/src/text/format/html5minify/mod.rs` | `crates/ui/src/text/format/html5minify.rs` |
| `crates/ui/src/theme/mod.rs` | `crates/ui/src/theme.rs` |
| `crates/ui/src/time/mod.rs` | `crates/ui/src/time.rs` |
| `crates/story/src/stories/mod.rs` | `crates/story/src/stories.rs` |
| `crates/story/src/stories/theme_story/mod.rs` | `crates/story/src/stories/theme_story.rs` |

## Story ve Dokümantasyon Kararları

- Story kodu `crates/story/src/stories/` altında kalır; üst modül `crates/story/src/stories.rs` dosyasındadır.
- Story metinleri ve dokümanlar Türkçedir.
- `docs/zh-CN` ağacı ilk uyarlama sonrasında kaldırıldı; upstreamden geri getirilmemelidir.
- VitePress yayın yolu Kavis'e göre ayarlandı:
  - Dokümantasyon: `/kavis-ui/`
  - WASM galeri: `/kavis-ui/gallery/`
- `LanguageSwitcher.vue` ve çoklu dil VitePress ayarları kaldırılmış kabul edilmelidir.

## İptal Edilen veya Taşınmaması Gereken Upstream Yaklaşımları

- Upstream yayın metadata'sı (`docs.rs`, `crates.io`, upstream repository/homepage) geri alınmamalı.
- `crates/reqwest_client` upstream kopyası geri eklenmemeli.
- `mod.rs` düzeni geri getirilmemeli.
- `docs/zh-CN` ve çok dilli docs altyapısı geri getirilmemeli.
- Upstream `webview` modülü UI crate içine doğrudan alınmamalı; mevcut `crates/webview` ayrımı korunmalı.
- Sadece İngilizce API eklemek yerine önce Türkçe karşılık veya bilinçli alias tasarlanmalı.

## İlk Uyarlamada Sonradan Düzeltilen Alanlar

- Açılır menü/düğme davranışı birkaç kez düzeltildi:
  - yön belirtme
  - ekran sınırında otomatik yön değiştirme
  - menü genişliğinin tetikleyiciyle hizalanması
  - popup tema mirası
  - WCAG kontrast yedeği
- Popover taşma/kaçış yönü düzeltildi.
- Web galeri WebGPU hatası sadeleştirildi.
- Grup bileşenlerinde görsel etki uyumu sağlandı.
- Makro tarafında `derive_aksiyon` ve crate adı uyarlamaları yapıldı.

## Gelecek Upstream Merge Akışı

1. Upstream değişikliğin kaynak aralığını netleştir:
   - `git -C ../gpui-component log --date=short --pretty=format:%h%x09%ad%x09%s`
   - `git -C ../gpui-component show --stat <sha>`
2. Dosya yolunu Kavis düzenine çevir:
   - `mod.rs` dosyalarını Kavis'teki düz dosya karşılığına uygula.
   - Story modülünü `stories.rs` ile bağla.
3. İngilizce API adını Kavis karşılığına eşle:
   - Önce `rg` ile mevcut Türkçe sembolü ara.
   - Yoksa küçük ve tutarlı bir Türkçe API ekle.
   - Gerekmedikçe İngilizce compatibility alias ekleme.
4. Bağımlılık değişikliği varsa lokal Zed path politikasını bozma.
5. Story etkisi varsa iki hedefi de kontrol et:
   - `cargo check -p kavis-ui-story`
   - `cargo check -p kavis-ui-story-web`
6. UI crate için en az:
   - `cargo fmt`
   - `cargo check -p kavis-ui`
   - Değişen alana odaklı `cargo test -p kavis-ui <filtre> --lib`
   - `git diff --check`
7. Davranış değişikliği animation/focus/menu/popup/layout içeriyorsa story üzerinden runtime veya görsel kontrol planla.

## Riskler

- Bu fork upstreamden çok erken ayrıldığı için otomatik patch uygulamak çoğu zaman yanlış dosya yoluna veya yanlış API adına gider.
- Türkçe public API ile `ham_gpui` escape hatch'i birlikte var; yeni kodda ham GPUI sadece karşılığı yoksa kullanılmalı.
- Zed path bağımlılıkları başka bilgisayarda da aynı relative checkout beklentisini taşır.
- `story-web` default feature seti farklıdır; desktop story derlemesi geçse bile web hedefi kırılabilir.
