# Katkı Rehberi

Katkılar kabul edilir. Lütfen hata bildirimi, özellik önerisi veya pull request gönderirken değişikliği dar kapsamlı ve mevcut kod stiliyle uyumlu tutun.

## Kurallar

- Her PR tek bir işi çözmelidir.
- Mevcut modül düzeni, Türkçe adlandırma ve builder kalıpları izlenmelidir.
- Yapay zeka yardımı kullanıldıysa PR açıklamasında hangi bölümlerin üretildiği belirtilmelidir.
- UI bileşenlerinde masaüstü uygulama davranışı temel alınır; düğmeler varsayılan imleci kullanır, yalnızca bağlantı gibi davranan öğeler pointer imleci kullanır.
- Varsayılan bileşen boyutu çoğu durumda `md` olmalıdır.

## Ortam Kurulumu

```bash
./script/bootstrap
```

Windows üzerinde:

```powershell
.\script\install-window.ps1
```

## Geliştirme Komutları

```bash
cargo run
cargo check
cargo fmt --check
cargo test --all
```

Tekil örnekler:

```bash
cargo run -p hello_world
cargo run -p app_assets
cargo run -p system_monitor
```

Story örnekleri:

```bash
cargo run --example editor
cargo run --example dock
cargo run --example table
```

## Performans

Çizim veya yerleşim kodu değiştiyse FPS ve profil çıktısını kontrol edin:

```bash
MTL_HUD_ENABLED=1 cargo run
samply record cargo run
```

## Sürüm

Önerilen sürüm artırma yolu:

```bash
./script/bump-version.sh x.y.z
```

Elle yayın gerekiyorsa crate sürümlerini güncelleyin, `Bump vx.y.z` mesajıyla commit oluşturun, `vx.y.z` tag'ini basıp ana dal ve tag'i uzak depoya gönderin.
