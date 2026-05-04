use gpui::{
    App, Background, Bounds, Corners, PaintQuad, Pixels, Point, Size, Window, fill, point, px,
};

use crate::plot::{
    label::{PlotLabel, TEXT_GAP, TEXT_HEIGHT, TEXT_SIZE, Text},
    origin_point,
};

/// Bir [`Bar`] şeklindeki çubukların hizalaması; yönü
/// (dikey veya yatay) ve taban çizgisinin bulunduğu tarafı belirler.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarAlignment {
    /// Taban çizgisi altta olan dikey çubuklar; çubuklar yukarı doğru büyür.
    #[default]
    Bottom,
    /// Taban çizgisi üstte olan dikey çubuklar; çubuklar aşağı doğru büyür.
    Top,
    /// Taban çizgisi solda olan yatay çubuklar; çubuklar sağa doğru büyür.
    Left,
    /// Taban çizgisi sağda olan yatay çubuklar; çubuklar sola doğru büyür.
    Right,
}

impl BarAlignment {
    pub fn yatay_mi(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn dikey_mi(self) -> bool {
        !self.yatay_mi()
    }

    /// Bu hizalama için çubuğun tabanından ucuna doğru ilerleyen doğrusal gradyan açısını
    /// derece cinsinden döndürür.
    ///
    /// gpui kuralı: `0°` yukarıyı gösterir (stop-0 altta, stop-1 üstte);
    /// açılar saat yönünde artar.
    pub fn gradient_angle(self) -> f32 {
        match self {
            Self::Bottom => 0.,
            Self::Top => 180.,
            Self::Left => 90.,
            Self::Right => 270.,
        }
    }
}

#[allow(clippy::type_complexity)]
pub struct Bar<T> {
    data: Vec<T>,
    alignment: BarAlignment,
    cross: Box<dyn Fn(&T) -> Option<f32>>,
    band_width: f32,
    base: Box<dyn Fn(&T) -> f32>,
    value: Box<dyn Fn(&T) -> Option<f32>>,
    fill: Box<dyn Fn(&T, Bounds<f32>, BarAlignment) -> Background>,
    label: Option<Box<dyn Fn(&T, Point<Pixels>) -> Vec<Text>>>,
    corner_radii: Corners<Pixels>,
}

impl<T> Default for Bar<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            alignment: BarAlignment::default(),
            cross: Box::new(|_| None),
            band_width: 0.,
            base: Box::new(|_| 0.),
            value: Box::new(|_| None),
            fill: Box::new(|_, _, _| gpui::black().into()),
            label: None,
            corner_radii: Corners::all(px(0.)),
        }
    }
}

impl<T> Bar<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bar verisini ayarlar.
    pub fn data<I>(mut self, data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.data = data.into_iter().collect();
        self
    }

    /// Bar hizalamasını ayarlar.
    ///
    /// Varsayılan [`BarAlignment::Bottom`] değeridir.
    pub fn alignment(mut self, alignment: BarAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Her çubuğun çapraz eksendeki konumunu (piksel cinsinden) ayarlar.
    ///
    /// Dikey hizalamalarda bu X koordinatıdır; yatay hizalamalarda
    /// Y koordinatıdır.
    pub fn cross<F>(mut self, cross: F) -> Self
    where
        F: Fn(&T) -> Option<f32> + 'static,
    {
        self.cross = Box::new(cross);
        self
    }

    /// Bar bant genişliğini (çapraz eksendeki çubuk kalınlığı) ayarlar.
    pub fn band_width(mut self, band_width: f32) -> Self {
        self.band_width = band_width;
        self
    }

    /// Her çubuğun taban çizgisi konumunu (değer ekseni boyunca piksel cinsinden) ayarlar.
    pub fn base<F>(mut self, base: F) -> Self
    where
        F: Fn(&T) -> f32 + 'static,
    {
        self.base = Box::new(base);
        self
    }

    /// Her çubuğun değer bitiş konumunu (değer ekseni boyunca piksel cinsinden) ayarlar.
    pub fn value<F>(mut self, value: F) -> Self
    where
        F: Fn(&T) -> Option<f32> + 'static,
    {
        self.value = Box::new(value);
        self
    }

    /// Her çubuğun dolgusunu ayarlar.
    ///
    /// Kapanış veri noktasını, çubuğun çizilen çerçevesini (`Bounds<f32>`)
    /// grafik sınırlarının başlangıcına göre ham piksel koordinatlarıyla ve
    /// çubuğun [`BarAlignment`] değerini alır. Böylece çağıran taraf yöne göre dallanabilir (örneğin
    /// gradyan açısını çevirebilir). Normalize veya grafiğe göreli
    /// koordinatlar üretmek isteyen çağıranlar bunu bu çerçeveden kendileri hesaplamalıdır.
    ///
    /// [`Background`] değerine dönüştürülebilen her tipi kabul eder; düz renkler ve
    /// tam belirtilmiş [`gpui::linear_gradient`] değerleri buna dahildir. Arka plan
    /// olduğu gibi kullanılır; gradyan açısı çubuk yönüne göre ayarlanmaz.
    pub fn fill<F, B>(mut self, fill: F) -> Self
    where
        F: Fn(&T, Bounds<f32>, BarAlignment) -> B + 'static,
        B: Into<Background>,
    {
        self.fill = Box::new(move |v, frame, alignment| fill(v, frame, alignment).into());
        self
    }

    /// Bar etiketini ayarlar.
    pub fn label<F>(mut self, label: F) -> Self
    where
        F: Fn(&T, Point<Pixels>) -> Vec<Text> + 'static,
    {
        self.label = Some(Box::new(label));
        self
    }

    /// Her çubuk dikdörtgenine uygulanacak köşe yarıçaplarını ayarlar.
    ///
    /// Tek tip yuvarlama için [`Corners::all`] kullanın veya
    /// yalnızca belirli köşeleri yuvarlamak için (ör. her çubuğun sadece uç kısmı) `Corners` değerini elle oluşturun.
    pub fn corner_radii(mut self, corner_radii: impl Into<Corners<Pixels>>) -> Self {
        self.corner_radii = corner_radii.into();
        self
    }

    fn path(&self, bounds: &Bounds<Pixels>) -> (Vec<PaintQuad>, PlotLabel) {
        let origin = bounds.origin;
        let mut graph = vec![];
        let mut labels = vec![];

        for v in &self.data {
            let Some(cross) = (self.cross)(v) else {
                continue;
            };
            let Some(value) = (self.value)(v) else {
                continue;
            };
            let base = (self.base)(v);

            let bw = self.band_width;
            let (frame, p1, p2) = if self.alignment.dikey_mi() {
                let x0 = cross;
                let x1 = cross + bw;
                let y_min = value.min(base);
                let y_max = value.max(base);
                let frame = Bounds {
                    origin: Point::new(x0, y_min),
                    size: Size::new(x1 - x0, y_max - y_min),
                };
                (
                    frame,
                    origin_point(px(x0), px(y_min), origin),
                    origin_point(px(x1), px(y_max), origin),
                )
            } else {
                let y0 = cross;
                let y1 = cross + bw;
                let x_min = value.min(base);
                let x_max = value.max(base);
                let frame = Bounds {
                    origin: Point::new(x_min, y0),
                    size: Size::new(x_max - x_min, y1 - y0),
                };
                (
                    frame,
                    origin_point(px(x_min), px(y0), origin),
                    origin_point(px(x_max), px(y1), origin),
                )
            };

            let bg = (self.fill)(v, frame, self.alignment);
            graph.push(fill(Bounds::from_corners(p1, p2), bg).corner_radii(self.corner_radii));

            if let Some(label) = &self.label {
                let label_origin = label_origin(self.alignment, cross, base, value, bw);
                labels.extend(label(v, label_origin));
            }
        }

        (graph, PlotLabel::new(labels))
    }

    /// Bar. çizer.
    pub fn paint(&self, bounds: &Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        let (graph, labels) = self.path(bounds);
        for quad in graph {
            window.paint_quad(quad);
        }
        labels.paint(bounds, window, cx);
    }
}

/// Çubuk etiketinin başlangıç noktasıdır; çubuğun dışında, değer bitişinde konumlandırılır.
///
/// çağıran chooses [`gpui::TextAlign`] (typically `Center` için dikey
/// çubuklar, `Left` için `BarAlignment::Left`, `Right` için `BarAlignment::Right`).
fn label_origin(
    alignment: BarAlignment,
    cross: f32,
    base: f32,
    value: f32,
    band_width: f32,
) -> Point<Pixels> {
    match alignment {
        BarAlignment::Bottom => {
            let cx = cross + band_width / 2.;
            // Normal: value < base (bar grows up). Etiket above bar end.
            if value <= base {
                point(px(cx), px(value - TEXT_HEIGHT))
            } else {
                point(px(cx), px(value + TEXT_GAP))
            }
        }
        BarAlignment::Top => {
            let cx = cross + band_width / 2.;
            // Normal: value > base (bar grows down). Etiket below bar end.
            if value >= base {
                point(px(cx), px(value + TEXT_GAP))
            } else {
                point(px(cx), px(value - TEXT_HEIGHT))
            }
        }
        BarAlignment::Left => {
            // Vertical centering: text origin is the top of the glyph cell.
            let cy = cross + band_width / 2. - TEXT_SIZE / 2.;
            // Normal: value > base (bar grows right). Etiket to the right of bar end.
            if value >= base {
                point(px(value + TEXT_GAP), px(cy))
            } else {
                point(px(value - TEXT_GAP), px(cy))
            }
        }
        BarAlignment::Right => {
            let cy = cross + band_width / 2. - TEXT_SIZE / 2.;
            // Normal: value < base (bar grows left). Etiket to the left of bar end.
            if value <= base {
                point(px(value - TEXT_GAP), px(cy))
            } else {
                point(px(value + TEXT_GAP), px(cy))
            }
        }
    }
}
