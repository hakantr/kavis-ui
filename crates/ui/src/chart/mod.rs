mod area_chart;
mod bar_chart;
mod candlestick_chart;
mod line_chart;
mod pie_chart;

pub use area_chart::AreaChart;
pub use bar_chart::BarChart;
pub use candlestick_chart::CandlestickChart;
pub use line_chart::LineChart;
pub use pie_chart::PieChart;

use gpui::{Hsla, SharedString, TextAlign};

use crate::plot::{
    AxisText,
    scale::{Scale, ScaleBand, ScalePoint},
};

/// Oluşturur x-eksen etiketler için nokta-temelli scales (`LineChart`, `AreaChart`).
///
/// Nokta ölçekleri öğeleri aralık boyunca eşit konumlara yerleştirir. İlk etiket
/// sola, son etiket sağa hizalanır; kalanlar ortalanır.
pub(crate) fn build_point_x_labels<T, X>(
    data: &[T],
    x_fn: &dyn Fn(&T) -> X,
    x_scale: &ScalePoint<X>,
    tick_margin: usize,
    color: Hsla,
) -> Vec<AxisText>
where
    X: PartialEq + Into<SharedString>,
{
    let data_len = data.len();
    data.iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if (i + 1) % tick_margin != 0 {
                return None;
            }
            x_scale.tick(&x_fn(d)).map(|x_tick| {
                let align = match i {
                    0 if data_len == 1 => TextAlign::Center,
                    0 => TextAlign::Left,
                    i if i == data_len - 1 => TextAlign::Right,
                    _ => TextAlign::Center,
                };
                // Call x_fn again to get an owned value for the label text.
                AxisText::new(x_fn(d).into(), x_tick, color).align(align)
            })
        })
        .collect()
}

/// Oluşturur eksen etiketler için band-temelli scales (`BarChart`, `CandlestickChart`).
///
/// Band scales place öğeler içinde eşit şekilde sized bands. döndürülen `tick`
/// Koordinat, band ekseni boyunca her bandın merkezidir; çağıran
/// decides olup olmadığını için feed result için `PlotAxis::x_label` (dikey
/// charts) veya `PlotAxis::y_label` (yatay charts).
pub(crate) fn build_band_labels<T, X>(
    data: &[T],
    x_fn: &dyn Fn(&T) -> X,
    x_scale: &ScaleBand<X>,
    band_width: f32,
    tick_margin: usize,
    color: Hsla,
) -> Vec<AxisText>
where
    X: PartialEq + Into<SharedString>,
{
    data.iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if (i + 1) % tick_margin != 0 {
                return None;
            }
            x_scale.tick(&x_fn(d)).map(|x_tick| {
                // Call x_fn again to get an owned value for the label text.
                AxisText::new(x_fn(d).into(), x_tick + band_width / 2., color)
                    .align(TextAlign::Center)
            })
        })
        .collect()
}
