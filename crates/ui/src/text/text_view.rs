use std::sync::Arc;

use crate::ham_gpui::prelude::FluentBuilder as _;
use crate::ham_gpui::{
    AnyElement, App, Bounds, Element, ElementId, Entity, GlobalElementId, Hitbox, HitboxBehavior,
    InspectorElementId, InteractiveElement, IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, ParentElement, Pixels, SharedString, StyleRefinement, Styled, Window, div,
};

use crate::StilUzantisi;
use crate::scroll::KaydirilabilirOge;
use crate::text::TextViewFormat;
use crate::text::node::CodeBlock;
use crate::text::state::MetinGorunumuDurumu;
use crate::{global_state::KureselDurum, text::MetinGorunumuStili};

/// Type için kod block eylemler generator fonksiyon.
pub(crate) type CodeBlockActionsFn =
    dyn Fn(&CodeBlock, &mut Window, &mut App) -> AnyElement + Send + Sync;

/// Markdown veya HTML çizebilen metin görünümü.
///
/// ## Goals
///
/// - Markdown veya HTML gibi içerikler için zengin metin çizim bileşeni sağlar,
/// için kullanılır gösterim rich metin içinde GPUI uygulama (e.g., Help messages, Release notalar)
/// - En yaygın işaretlemeleri göstermek için Markdown GFM ve HTML destekler (Safari Reader Mode benzeri basit HTML).
/// - Heading, Paragraph, Bold, Italic, StrikeThrough, kod, Link, Image, Blockquote, Liste, Tablo, HorizontalRule ve CodeBlock desteklenir.
///
/// ## Değil Goals
///
/// - Karmaşık stil özelleştirmesi (bazı basit stiller desteklenir)
/// - As bir markdown düzenleyici veya viewer (Eğer siz isterseniz için gibi bu, siz olmalıdır fork your version).
/// - HTML görüntüleyici olarak CSS desteklenmez; içerik okuyucu için yalnızca temel HTML etiketleri desteklenir.
///
/// Bakınız ayrıca [`MarkdownElement`], [`HtmlElement`]
#[derive(Clone)]
pub struct MetinGorunumu {
    id: ElementId,
    format: Option<TextViewFormat>,
    text: Option<SharedString>,
    pub(crate) state: Option<Entity<MetinGorunumuDurumu>>,
    text_view_style: MetinGorunumuStili,
    style: StyleRefinement,
    selectable: bool,
    scrollable: bool,
    code_block_actions: Option<Arc<CodeBlockActionsFn>>,
}

impl Styled for MetinGorunumu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl MetinGorunumu {
    /// Oluşturur yeni MetinGorunumu ile yönetilir durum.
    pub fn new(state: &Entity<MetinGorunumuDurumu>) -> Self {
        Self {
            id: ElementId::Name(state.entity_id().to_string().into()),
            state: Some(state.clone()),
            format: None,
            text: None,
            text_view_style: MetinGorunumuStili::default(),
            style: StyleRefinement::default(),
            selectable: false,
            scrollable: false,
            code_block_actions: None,
        }
    }

    /// Yeni bir markdown metin görünüm oluşturur.
    pub fn markdown(id: impl Into<ElementId>, markdown: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            format: Some(TextViewFormat::Markdown),
            text: Some(markdown.into()),
            text_view_style: MetinGorunumuStili::default(),
            style: StyleRefinement::default(),
            state: None,
            selectable: false,
            scrollable: false,
            code_block_actions: None,
        }
    }

    /// Yeni bir html metin görünüm oluşturur.
    pub fn html(id: impl Into<ElementId>, html: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            format: Some(TextViewFormat::Html),
            text: Some(html.into()),
            text_view_style: MetinGorunumuStili::default(),
            style: StyleRefinement::default(),
            state: None,
            selectable: false,
            scrollable: false,
            code_block_actions: None,
        }
    }

    /// [`MetinGorunumuStili`] ayarlar.
    pub fn style(mut self, style: MetinGorunumuStili) -> Self {
        self.text_view_style = style;
        self
    }

    /// Metin görünümünün seçilebilir olup olmadığını ayarlar. Varsayılan false değeridir.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Metin görünümünün kaydırılabilir olup olmadığını ayarlar. Varsayılan false değeridir.
    ///
    /// ## Eğer true için `scrollable`
    ///
    /// `scrollable` mod için kullanılır büyük içerik,
    /// Kaydırma çubuğunu gösterir; bunun için üst öğenin sabit yüksekliği olmalıdır,
    /// ve kullanım [`crate::ham_gpui::list`] çizmek için içerik içinde bir virtualized yol.
    ///
    /// ## Eğer false için fit içerik
    ///
    /// MetinGorunumu tüm içeriğe sığacak şekilde genişler; kaydırma çubuğu olmaz.
    /// Bu mod birkaç satır metin veya etiket gibi küçük içerikler için uygundur.
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// özel block eylemler için kod blocks ayarlar.
    ///
    /// kapanış receives [`CodeBlock`],
    /// ve döndürür bir öğe göstermek için.
    pub fn code_block_actions<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&CodeBlock, &mut Window, &mut App) -> E + Send + Sync + 'static,
        E: IntoElement,
    {
        self.code_block_actions = Some(Arc::new(move |code_block, window, cx| {
            f(&code_block, window, cx).into_any_element()
        }));
        self
    }
}

impl IntoElement for MetinGorunumu {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct MetinGorunumuYerlesimDurumu {
    state: Entity<MetinGorunumuDurumu>,
    element: AnyElement,
}

impl Element for MetinGorunumu {
    type RequestLayoutState = MetinGorunumuYerlesimDurumu;
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let state = if let Some(state) = self.state.clone() {
            state
        } else {
            let default_format = self.format.unwrap_or(TextViewFormat::Markdown);
            let default_text = self.text.clone().unwrap_or_default();

            let state = window.use_keyed_state(
                SharedString::from(format!("{}/state", self.id)),
                cx,
                move |_, cx| {
                    if default_format == TextViewFormat::Markdown {
                        MetinGorunumuDurumu::markdown(default_text.as_str(), cx)
                    } else {
                        MetinGorunumuDurumu::html(default_text.as_str(), cx)
                    }
                },
            );
            self.state = Some(state.clone());
            state
        };

        state.update(cx, |state, cx| {
            state.code_block_actions = self.code_block_actions.clone();
            state.selectable = self.selectable;
            state.scrollable = self.scrollable;
            state.text_view_style = self.text_view_style.clone();

            if let Some(text) = self.text.clone() {
                state.set_text(text.as_str(), cx);
            }
        });

        let focus_handle = state.read(cx).focus_handle.clone();
        let list_state = state.read(cx).list_state.clone();

        let mut el = div()
            .key_context("MetinGorunumu")
            .track_focus(&focus_handle)
            .when(self.scrollable, |this| {
                this.size_full().vertical_scrollbar(&list_state)
            })
            .relative()
            .on_action(window.listener_for(&state, MetinGorunumuDurumu::on_action_copy))
            .child(state.clone())
            .refine_style(&self.style)
            .into_any_element();
        let layout_id = el.request_layout(window, cx);
        (
            layout_id,
            MetinGorunumuYerlesimDurumu { state, element: el },
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        request_layout.element.prepaint(window, cx);
        window.insert_hitbox(bounds, HitboxBehavior::Normal)
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let state = &request_layout.state;
        KureselDurum::global_mut(cx)
            .text_view_state_stack
            .push(state.clone());
        request_layout.element.paint(window, cx);
        KureselDurum::global_mut(cx).text_view_state_stack.pop();

        if self.selectable {
            let is_selecting = state.read(cx).is_selecting;
            let has_selection = state.read(cx).has_selection();
            let parent_view_id = window.current_view();

            window.on_mouse_event({
                let state = state.clone();
                let hitbox = hitbox.clone();
                move |event: &MouseDownEvent, phase, window, cx| {
                    if !phase.bubble() || !hitbox.is_hovered(window) {
                        return;
                    }

                    state.update(cx, |state, _| {
                        state.start_selection(event.position);
                    });
                    cx.notify(parent_view_id);
                }
            });

            if is_selecting {
                // move to update end position.
                window.on_mouse_event({
                    let state = state.clone();
                    move |event: &MouseMoveEvent, phase, _, cx| {
                        if !phase.bubble() {
                            return;
                        }

                        state.update(cx, |state, _| {
                            state.update_selection(event.position);
                        });
                        cx.notify(parent_view_id);
                    }
                });

                // up to end selection
                window.on_mouse_event({
                    let state = state.clone();
                    move |_: &MouseUpEvent, phase, _, cx| {
                        if !phase.bubble() {
                            return;
                        }

                        state.update(cx, |state, _| {
                            state.end_selection();
                        });
                        cx.notify(parent_view_id);
                    }
                });
            }

            if has_selection {
                // down outside to clear selection
                window.on_mouse_event({
                    let state = state.clone();
                    let hitbox = hitbox.clone();
                    move |_: &MouseDownEvent, _, window, cx| {
                        if hitbox.is_hovered(window) {
                            return;
                        }

                        state.update(cx, |state, _| {
                            state.clear_selection();
                        });
                        cx.notify(parent_view_id);
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MetinGorunumu;
    use crate::text::MetinGorunumuDurumu;
    use crate::ham_gpui::{
        AppContext as _, Context, Entity, IntoElement, Modifiers, MouseButton, ParentElement as _,
        Render, Styled as _, TestAppContext, VisualTestContext, Window, div, point, px,
    };

    struct TextViewTestRoot {
        text_view: Entity<MetinGorunumuDurumu>,
    }

    impl TextViewTestRoot {
        fn new(text: &str, cx: &mut Context<Self>) -> Self {
            let text = text.to_string();
            let text_view = cx.new(|cx| MetinGorunumuDurumu::markdown(&text, cx));
            Self { text_view }
        }
    }

    impl Render for TextViewTestRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(160.))
                .child(
                    div()
                        .h(px(24.))
                        .overflow_hidden()
                        .child(MetinGorunumu::new(&self.text_view).selectable(true)),
                )
                .child(div().h(px(40.)).child("footer"))
        }
    }

    #[crate::ham_gpui::test]
    fn clipped_markdown_link_does_not_open(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (_, cx) = cx.add_window_view(|_, cx| {
            TextViewTestRoot::new("visible\n\n[hidden](https://example.com)", cx)
        });
        let cx: &mut VisualTestContext = cx;

        cx.simulate_click(point(px(10.), px(34.)), Modifiers::default());

        assert_eq!(cx.opened_url(), None);
    }

    #[crate::ham_gpui::test]
    fn clipped_markdown_cannot_start_selection(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let (view, cx) = cx
            .add_window_view(|_, cx| TextViewTestRoot::new("visible\n\nhidden selection text", cx));
        let cx: &mut VisualTestContext = cx;

        cx.simulate_mouse_down(
            point(px(10.), px(34.)),
            MouseButton::Left,
            Modifiers::default(),
        );
        cx.simulate_mouse_move(
            point(px(90.), px(34.)),
            Some(MouseButton::Left),
            Modifiers::default(),
        );
        cx.simulate_mouse_up(
            point(px(90.), px(34.)),
            MouseButton::Left,
            Modifiers::default(),
        );

        let selected_text = view.read_with(cx, |root, cx| root.text_view.read(cx).selected_text());
        assert!(
            selected_text.is_empty(),
            "unexpected selection: {selected_text:?}"
        );
    }
}
