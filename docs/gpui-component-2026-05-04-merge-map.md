# kavis-ui / gpui-component 2026-05-04 Merge Map

Bu not, `../gpui-component` reposundaki 2026-05-04 tarihli upstream değişiklikleri `kavis-ui` içine uyarlarken alınan kararları kaydeder. Gelecekte benzer karşılaştırmalarda önce bu harita okunmalı, sonra upstream diff incelenmelidir.

## Kaynak Commitler

- `09409026` - `highlighter: add inline markdown syntax highlighting (#2333)`
- `3d69b147` - `sidebar: Support shadcn collapsible modes (#2329)`
- `49b4ad41` - `separator: Rename Divider to Separator (#2335)`
- `5355ce29` - `notification: Align status icon to the title (#2334)`

## Uygulanan Dosya Haritası

| Upstream alanı | kavis-ui karşılığı | Not |
| --- | --- | --- |
| `crates/ui/src/sidebar.rs` | `crates/ui/src/sidebar.rs` | Shadcn collapse modları Türkçe API ile taşındı. |
| `SidebarCollapsible` | `YanCubukDaralma` | `Icon`, `Offcanvas`, `None` varyantları var. |
| `.collapsible(true/false)` | `.collapsible(YanCubukDaralma::...)` | Geri uyumluluk iptal edildi; bool kabul edilmiyor. |
| `crates/story/src/stories/sidebar_story.rs` | `crates/story/src/stories/sidebar_story.rs` | Story yeni collapse mode seçicisine geçirildi. |
| `examples/sidebar` | `examples/sidebar` | Shadcn collapse modlarını gösteren Kavis örneği Türkçe API ile taşındı. |
| `ButtonGroup` | `DugmeGrubu` | Story kontrolü için kullanıldı. |
| `Selectable` trait | `Secilebilir as _` | `.selected(...)` çağrıları için import edildi. |
| `Divider` -> `Separator` | `Ayirici` korunuyor, `separator` alias eklendi | Türkçe ana API bozulmadı, upstream İngilizce isim için köprü var. |
| `DividerStory` -> `SeparatorStory` | `AyiriciStory` | Kavis story adı Türkçe karşılığa taşındı. |
| `DescriptionList::divider()` | `DescriptionList::separator()` alias | `separator()` çağrısı `divider()`a yönlenir. |
| `notification` icon alignment | `Bildirim` ikon wrapper | `.top(px(18.)).left_4()` ile başlık hizasına alındı. |
| Markdown inline injections | `SozdizimiVurgulayici` injection layers | Inline markdown katmanları ayrı parse ediliyor. |

## Sidebar Kararları

- Yeni enum: `YanCubukDaralma::{Icon, Offcanvas, None}`.
- Varsayılan davranış: `Icon`.
- `Icon`: collapse durumunda sidebar genişliği `48px` olur ve içerik `Daraltilabilir::collapsed(true)` görür.
- `Offcanvas`: collapse durumunda wrapper genişliği `0px`e animasyonlanır; içerik animasyon bitene kadar mounted kalır, sonra çıkarılır.
- `None`: `collapsed` bilgisi yok sayılır; wrapper animasyonu kullanılmaz.
- Sağ/sol taraf hizalama offcanvas ve icon modlarında ayrı ele alınır.
- Özel width piksel değilse animasyon yerine eski doğal layout korunur.

## İptal Edilen Yaklaşımlar

- İlk taslakta `.collapsible(bool)` için `impl From<bool> for YanCubukDaralma` ve `pub type SidebarCollapsible = YanCubukDaralma` vardı.
- Kullanıcı geri uyumluluk gerekmediğini belirttiği için bu uyumluluk katmanı kaldırıldı.
- Eski `.collapsible(false)` çağrıları `YanCubukDaralma::None` olarak düzeltildi.
- Bool uyumluluğunu doğrulayan test kaldırıldı.

## Story / Story Web Uyarlaması

- `SidebarStory` state içine `collapsible: YanCubukDaralma` eklendi.
- Story üst kontrol alanına `DugmeGrubu::new("collapsible-mode")` eklendi.
- Modlar: `Icon`, `Offcanvas`, `None`.
- `None` modunda `YanCubukGecisDugmesi` ve yanındaki `Ayirici` render edilmiyor.
- Header/footer içerik gizleme koşulu raw `collapsed` yerine `icon_collapsed` ile yapıldı.
- Main content layout `size_full()` yerine `h_full().flex_1().min_w_0().overflow_hidden()` oldu.
- `story-web` kırılımı için ayrıca `cargo check -p kavis-ui-story-web` çalıştırıldı ve geçti.
- Upstream `examples/sidebar` örneği `examples/sidebar` olarak Kavis API ile eklendi.

## Highlighter Uyarlaması

- `injections_query` artık `Option<Arc<Query>>`.
- `injection_layers: Vec<InjectionLayer>` eklendi.
- `InjectionLayer` alanları: `language_name`, `ranges`, `byte_range`, `tree`.
- `InjectionParseData` içine `language_capture_index` ve `old_layers` eklendi.
- `compute_injection_layers`, `parse_injection_layers`, `should_parse_injection_layer` akışı eklendi.
- Markdown inline için gereksiz layer üretmemek adına trigger kontrolü var.
- Markdown inline highlights:
  - `emphasis`
  - `emphasis.strong`
  - `strikethrough`
  - `text.literal`
- `default-theme.json` light/dark syntax stillerine `emphasis`, `emphasis.strong`, `strikethrough` eklendi.
- `YaziTipiStili::Strikethrough` eklendi ve GPUI `StrikethroughStyle`a dönüştürülüyor.

## Separator / Divider Notu

- Upstream İngilizce API `Divider` -> `Separator` ad değişimine gitti.
- kavis-ui tarafında mevcut Türkçe ad `Ayirici` olduğu için tam dosya rename yapılmadı.
- Eklenen köprü:
  - Yeni dosya: `crates/ui/src/separator.rs`
  - Re-export: `Separator = Ayirici`, `SeparatorStyle = AyiriciStili`
  - `lib.rs`: `pub mod separator;`
  - `bilesenler.rs`: `pub use crate::separator::*;`
  - `description_list.rs`: `separator()` alias
- Story tarafında eski `DividerStory` adı yerine Türkçe `AyiriciStory` kullanılır.
- Gelecekte İngilizce örnek kod upstreamden doğrudan taşınırsa `separator` modülü importları kurtarır.

## Test ve Kontrol Komutları

Bu merge sonunda geçen komutlar:

```bash
cargo fmt
cargo check -p kavis-ui
cargo test -p kavis-ui highlighter --lib --features tree-sitter-markdown
cargo test -p kavis-ui sidebar --lib
cargo check -p kavis-ui-story
cargo check -p kavis-ui-story-web
git diff --check
```

## Sonradan Kapatılan Test Boşlukları

İlk uyarlamada upstream test grubunun bir kısmı taşınmamıştı. Şimdi eklendi:

### Sidebar (`crates/ui/src/sidebar.rs` `tests` modülü)

- `icon_modu_acikken_genisleyen_genisligi_kullanir`
- `icon_modu_acikken_piksel_olmayan_genislikte_ozgun_yerlesimi_korur`
- `offcanvas_acikken_piksel_genisligi_kullanir`
- `offcanvas_daralinca_piksel_olmayan_genislik_durumunda_yerlesimi_statik_birakir`
- `offcanvas_acikken_piksel_olmayan_genislikte_ozgun_yerlesimi_korur`
- `offcanvas_alt_ogeyi_icerik_kenarina_yaslamali`
- `animasyon_kimligi_yan_cubuk_kimligine_baglanmali`
- `animasyon_durumu_bekleyen_offcanvas_gizlemesini_yeniden_planlamamali`
- `animasyon_durumu_yeniden_acilinca_bekleyen_gizlemeyi_iptal_etmeli`
- `animasyon_durumu_eski_gizleme_isteklerini_yok_saymali`
- `animasyon_durumu_baslangicta_offcanvas_kapaliysa_gizli_baslamali`
- `icon_modu_daralinca_simge_genisligi_kullanir` testine `align_child_to_end` doğrulaması eklendi.
- `bool_collapsible_should_remain_backward_compatible` testi alınmadı; bool API'si bilinçli olarak desteklenmiyor.

### Highlighter (`crates/ui/src/highlighter/highlighter.rs` `tests` modülü)

- `test_markdown_inline_emphasis`
- `test_markdown_inline_strikethrough`
- `test_markdown_inline_emphasis_style_depends_on_theme`
- `test_markdown_inline_markers_create_injection_layer`
- `test_markdown_inline_latex_marker_creates_injection_layer`
- `test_markdown_inline_nested_emphasis_uses_default_bold_italic_style`
- `test_markdown_inline_link_text`
- `test_markdown_inline_code_span`
- `test_markdown_inline_latex_span`
- `test_markdown_inline_regions_do_not_combine_across_paragraphs`
- `test_html_script_and_style_injections` (HTML/JS/CSS feature kombinasyonuna feature-gated)
- `test_highlight_theme` yardımcı fonksiyonu `emphasis` (italic) ve `text.literal` stillerini de döndürecek şekilde upstream ile eşitlendi.
- `has_highlight_covering` yardımcı fonksiyonu HTML feature kombinasyonu için de derlenmesi adına feature gate genişletildi.
- `test_markdown_plain_inline_skips_injection_layer` upstream'da Japonca CJK metniyle çağrılırken, kavis-ui tarafında Türkçe ASCII metin korundu (trigger byte tetiklemediği için fonksiyonel sonuç aynı).

## Sonradan Kapatılan Rename Boşlukları

Upstream `49b4ad41` commit'inde `Divider`'dan ileri taşınan iç adlandırmalar başlangıçta atlandı. Türkçe karşılıklarına dönüştürüldü:

- `crates/ui/src/kbd.rs` `Kbd::format` içindeki `const DIVIDER` → `const AYIRICI`. (Tamamen iç değişken, `Ayirici` tip adıyla aynı kalıba çekildi.)
- `crates/ui/src/text/node.rs` markdown blok render'ındaki `div().id("divider")` → `div().id("ayirici")`. (`BlockNode::Ayirici` varyantıyla tutarlı.)
- `BlockNode::Ayirici` varyantı upstream'de `BlockNode::HorizontalRule` olarak yeniden adlandırıldı, ancak `pub(crate)` iç tip olduğu için fork tercihi olarak Türkçe `Ayirici` korunuyor (semantik olarak hem ayırıcı hem yatay çizgi anlamlarını kapsıyor).

## Temizlenen Ölü i18n Anahtarları

`crates/story/src/lib.rs` içinde `divider_story` döneminden kalan, artık çağrılmayan dört anahtar kaldırıldı:

- `"Combination Dividers" => "Birleşik Ayırıcılar"`
- `"Horizontal Dividers" => "Yatay Ayırıcılar"`
- `"Vertical Dividers" => "Dikey Ayırıcılar"`
- `"A divider that can be either vertical or horizontal." => "Dikey veya yatay kullanılabilen ayırıcı."`

`ayirici_story.rs` artık Türkçe stringleri (`section("Yatay Ayırıcılar")` vb.) doğrudan render ettiği için bu i18n anahtarları lookup edilmiyordu.

## Gelecek Merge Kontrol Listesi

1. Upstream tarih filtresini netleştir:
   - `git log --date=short --pretty=format:%h%x09%ad%x09%s`
2. Her commit için önce upstream diff oku:
   - `git show --stat <sha>`
   - `git show <sha> -- <ilgili-dosya>`
3. kavis-ui adlandırma eşleşmesini çıkar:
   - `Sidebar` -> `YanCubuk`
   - `Button` -> `Dugme`
   - `ButtonGroup` -> `DugmeGrubu`
   - `Selectable` -> `Secilebilir`
   - `Divider` -> `Ayirici` veya `separator` alias
4. Eski kullanım taraması yap:
   - `.collapsible(true/false)`
   - `divider::`
   - `DividerStory` / `SeparatorStory` -> `AyiriciStory`
5. Story ve story-web ayrı ayrı derle.
6. Eğer kullanıcı geri uyumluluk istemiyorsa compatibility adapter ekleme.
7. Geniş davranış değişikliklerinde küçük unit test ekle; UI story davranışı için compile check ile yetinme, gerekirse görsel/runtime kontrol planla.

## Riskler

- `Offcanvas` modunda içerik animasyon sonrası unmount edilir; focus/tab order davranışı ileride runtime testle doğrulanmalı.
- `separator` için ana Türkçe API `Ayirici` olarak kalır. İngilizce `Separator` yalnızca upstream importlarını taşımak için köprü alias'tır.
- Highlighter injection akışı daha karmaşık hale geldi; yeni dil injectionları eklenirse `should_parse_injection_layer` sadece markdown inline özelinde kalmalı.
