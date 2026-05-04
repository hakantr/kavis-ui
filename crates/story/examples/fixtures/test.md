# Markdown Gösterim Örneği

Bu dosya, Kavis UI Markdown ve metin görünümü örneklerinde kullanılan Türkçe test içeriğidir.

## Başlıklar

Paragraflar normal metin, **kalın metin**, *italik metin*, `satır içi kod` ve bağlantı içeriklerini birlikte gösterebilir.

[Kavis UI deposu](https://github.com/hakantr/kavis-ui)

## Liste

- Birinci madde
- İkinci madde
- Üçüncü madde

1. Hazırlık
2. Uygulama
3. Doğrulama

## Alıntı

> Kavis UI, GPUI üzerinde modern masaüstü arayüzleri kurmak için bileşenler sağlar.

## Kod Bloğu

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::button::{Dugme, DugmeVaryantlari as _};

Dugme::new("kaydet")
    .primary()
    .label("Kaydet")
    .on_click(|_, _, _| println!("Kaydedildi"));
```

## Tablo

| Bileşen | Görev |
| --- | --- |
| `Dugme` | Eylem başlatır |
| `Girdi` | Metin alır |
| `Uyari` | Mesaj gösterir |

## Görevler

- [x] Markdown başlıklarını göster
- [x] Kod bloğunu vurgula
- [ ] Yeni örnek ekle
