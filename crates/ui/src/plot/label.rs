use std::fmt::Debug;

use crate::ham_gpui::{
    App, Bounds, FontWeight, Hsla, Pixels, Point, ShapedLine, SharedString, TextAlign, TextRun,
    Window, point, px,
};

use super::origin_point;

pub const TEXT_SIZE: f32 = 10.;
pub const TEXT_GAP: f32 = 2.;
pub const TEXT_HEIGHT: f32 = TEXT_SIZE + TEXT_GAP;

fn shape_label(
    text: &SharedString,
    font_size: Pixels,
    color: Hsla,
    window: &mut Window,
) -> ShapedLine {
    let text_run = TextRun {
        len: text.len(),
        font: window.text_style().font(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(text.clone(), font_size, &[text_run], None)
}

/// `font_size` kullanarak `metin` değerinin çizilmiş genişliğini pencere üzerinden döndürür.
/// geçerli metin stil. İçin kullanılır yerleşim calculations olan gerekir için reserve
/// Ölçek aralıklarından önce etiketler için boşluk sabitlenir.
pub fn measure_text_width(text: &SharedString, font_size: Pixels, window: &mut Window) -> f32 {
    if text.is_empty() {
        return 0.;
    }
    shape_label(
        text,
        font_size,
        Hsla {
            h: 0.,
            s: 0.,
            l: 0.,
            a: 1.,
        },
        window,
    )
    .width()
    .as_f32()
}

pub struct Text {
    pub text: SharedString,
    pub origin: Point<Pixels>,
    pub color: Hsla,
    pub font_size: Pixels,
    pub font_weight: FontWeight,
    pub align: TextAlign,
}

impl Text {
    pub fn new<T>(text: impl Into<SharedString>, origin: Point<T>, color: Hsla) -> Self
    where
        T: Default + Clone + Copy + Debug + PartialEq + Into<Pixels>,
    {
        let origin = point(origin.x.into(), origin.y.into());

        Self {
            text: text.into(),
            origin,
            color,
            font_size: TEXT_SIZE.into(),
            font_weight: FontWeight::NORMAL,
            align: TextAlign::Left,
        }
    }

    /// font boyut metin ayarlar.
    pub fn font_size(mut self, font_size: impl Into<Pixels>) -> Self {
        self.font_size = font_size.into();
        self
    }

    /// font weight metin ayarlar.
    pub fn font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }

    /// hizalama metin ayarlar.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

impl<I> From<I> for PlotLabel
where
    I: Iterator<Item = Text>,
{
    fn from(items: I) -> Self {
        Self::new(items.collect())
    }
}

#[derive(Default)]
pub struct PlotLabel(Vec<Text>);

impl PlotLabel {
    pub fn new(items: Vec<Text>) -> Self {
        Self(items)
    }

    /// Etiket. çizer.
    pub fn paint(&self, bounds: &Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        for Text {
            text,
            origin,
            color,
            font_size,
            font_weight: _,
            align,
        } in self.0.iter()
        {
            let origin = origin_point(origin.x, origin.y, bounds.origin);

            let line = shape_label(text, *font_size, *color, window);
            let origin = match align {
                TextAlign::Left => origin,
                TextAlign::Right => origin - point(line.width(), px(0.)),
                _ => origin - point(line.width() / 2., px(0.)),
            };
            let _ = line.paint(origin, *font_size, *align, None, window, cx);
        }
    }
}
