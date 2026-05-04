use kavis_ui::ham_gpui::*;
use kavis_ui::EtkinTema as _;

#[derive(IntoElement)]
pub struct Checkerboard {
    children: Vec<AnyElement>,
    is_dark: bool,
}

impl Checkerboard {
    pub fn new(is_dark: bool) -> Self {
        Self {
            children: Vec::new(),
            is_dark,
        }
    }
}

impl ParentElement for Checkerboard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Checkerboard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let square_size = px(12.);
        // Use a subtle difference for the checkerboard
        let (c1, c2) = if self.is_dark {
            // Dark mode: dark grey and slightly lighter grey
            (hsla(0., 0., 0.1, 1.), hsla(0., 0., 0.13, 1.))
        } else {
            // Light mode: white and light grey
            (hsla(0., 0., 1.0, 1.), hsla(0., 0., 0.95, 1.))
        };

        div()
            .bg(c1)
            .rounded(cx.theme().radius_lg)
            .overflow_hidden()
            .size_full()
            .child(
                kavis_ui::ham_gpui::canvas(
                    move |_, _, _| (),
                    move |bounds, _, window, _| {
                        let size = square_size;
                        let rows = (bounds.size.height / size).ceil() as i32;
                        let cols = (bounds.size.width / size).ceil() as i32;

                        for row in 0..rows {
                            for col in 0..cols {
                                if (row + col) % 2 == 0 {
                                    let origin = bounds.origin
                                        + kavis_ui::ham_gpui::point(size * (col as f32), size * (row as f32));

                                    window.paint_quad(kavis_ui::ham_gpui::PaintQuad {
                                        bounds: kavis_ui::ham_gpui::Bounds {
                                            origin,
                                            size: kavis_ui::ham_gpui::size(size, size),
                                        },
                                        corner_radii: kavis_ui::ham_gpui::Corners::default(),
                                        background: c2.into(),
                                        border_widths: kavis_ui::ham_gpui::Edges::default(),
                                        border_color: kavis_ui::ham_gpui::transparent_black(),
                                        border_style: kavis_ui::ham_gpui::BorderStyle::default(),
                                    });
                                }
                            }
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
            .children(self.children)
    }
}
