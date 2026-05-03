# Input Display Map

Bu klasör, metin girdisinin tampon konumu ile ekranda görünen konum arasındaki dönüşümü yönetir. `InputState`, satır kaydırma, katlama ve seçimi doğru çizmek için bu katmanı kullanır.

- `display_map.rs`: Tampon noktası ve görüntü noktası dönüşümleri.
- `wrap_map.rs`: Satır kaydırma hesapları.
- `fold_map.rs`: Katlanmış aralıkların eşlemesi.
- `text_wrapper.rs`: Görsel satır oluşturma yardımcıları.
- `folding.rs`: Katlama bilgisi ve aralık yönetimi.

Uygulama tarafında doğrudan bu modül yerine `InputState`, `Input`, `NumberInput` ve `OtpInput` kullanılmalıdır.
