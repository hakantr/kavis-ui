# Kavis UI Story Web

Kavis UI bileşen galerisinin WebAssembly sürümüdür.

Canlı galeri: https://hakantr.github.io/kavis-ui/gallery/

## Kurulum

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
make install
```

## Geliştirme

```bash
make dev
```

## Üretim

```bash
make build-prod
```

Çıktı `www/dist/` klasörüne yazılır ve GitHub Pages için `/kavis-ui/gallery/` base path'iyle hazırlanır.
