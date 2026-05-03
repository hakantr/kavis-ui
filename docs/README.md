# kavis-ui-docs

Bu klasör Kavis UI dokümantasyon sitesini içerir. İçerik Türkçedir; mevcut rota klasörleri geriye dönük uyumluluk için korunmuştur.

```bash
bun install
bun run dev
bun run build
```

## Birleşik Web Dağıtımı

`gpui-component` doküman yapısına benzer şekilde ana dokümantasyon `/kavis-ui/`, WASM galeri ise `/kavis-ui/gallery/` altında yayınlanır.

Depo kökünden tek çıktı üretmek için:

```bash
make build-web-dist
```

Bu komut `docs/.vitepress/dist` klasörünü üretir ve `crates/story-web/www/dist` galeri çıktısını `docs/.vitepress/dist/gallery` altına kopyalar.
