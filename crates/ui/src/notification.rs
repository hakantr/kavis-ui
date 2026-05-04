use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
    rc::Rc,
    time::Duration,
};

use crate::ham_gpui::{
    Anchor, Animation, AnimationExt, AnyElement, App, AppContext, ClickEvent, Context,
    DismissEvent, ElementId, Entity, EventEmitter, InteractiveElement as _, IntoElement,
    ParentElement as _, Pixels, Render, SharedString, StatefulInteractiveElement, StyleRefinement,
    Styled, Subscription, Window, div, prelude::FluentBuilder, px,
};

use crate::{
    Boyutlandirilabilir as _, Edges, EtkinTema as _, Simge, SimgeAdi, StilUzantisi,
    TITLE_BAR_HEIGHT,
    animation::cubic_bezier,
    button::{Dugme, DugmeVaryantlari as _},
    h_flex, v_flex,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum BildirimTuru {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl BildirimTuru {
    fn icon(&self, cx: &App) -> Simge {
        match self {
            Self::Info => Simge::new(SimgeAdi::Info).text_color(cx.theme().info),
            Self::Success => Simge::new(SimgeAdi::CircleCheck).text_color(cx.theme().success),
            Self::Warning => Simge::new(SimgeAdi::TriangleAlert).text_color(cx.theme().warning),
            Self::Error => Simge::new(SimgeAdi::CircleX).text_color(cx.theme().danger),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) enum BildirimKimligi {
    Id(TypeId),
    IdAndElementId(TypeId, ElementId),
}

impl From<TypeId> for BildirimKimligi {
    fn from(type_id: TypeId) -> Self {
        Self::Id(type_id)
    }
}

impl From<(TypeId, ElementId)> for BildirimKimligi {
    fn from((type_id, id): (TypeId, ElementId)) -> Self {
        Self::IdAndElementId(type_id, id)
    }
}

/// Bir bildirim öğe.
pub struct Bildirim {
    /// Bildirim kimliğini benzersiz yapmak için kullanılır.
    /// Aynı id ile yeni bildirim gönderirseniz önceki bildirim değiştirilir.
    ///
    /// None, bildirimin listenin sonuna ekleneceği anlamına gelir.
    id: BildirimKimligi,
    style: StyleRefinement,
    type_: Option<BildirimTuru>,
    title: Option<SharedString>,
    message: Option<SharedString>,
    icon: Option<Simge>,
    autohide: bool,
    action_builder: Option<Rc<dyn Fn(&mut Self, &mut Window, &mut Context<Self>) -> Dugme>>,
    content_builder: Option<Rc<dyn Fn(&mut Self, &mut Window, &mut Context<Self>) -> AnyElement>>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    closing: bool,
}

impl From<String> for Bildirim {
    fn from(s: String) -> Self {
        Self::new().message(s)
    }
}

impl From<SharedString> for Bildirim {
    fn from(s: SharedString) -> Self {
        Self::new().message(s)
    }
}

impl From<&'static str> for Bildirim {
    fn from(s: &'static str) -> Self {
        Self::new().message(s)
    }
}

impl From<(BildirimTuru, &'static str)> for Bildirim {
    fn from((type_, content): (BildirimTuru, &'static str)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

impl From<(BildirimTuru, SharedString)> for Bildirim {
    fn from((type_, content): (BildirimTuru, SharedString)) -> Self {
        Self::new().message(content).with_type(type_)
    }
}

struct DefaultIdType;

impl Bildirim {
    /// Yeni bir bildirim oluşturur.
    ///
    /// Varsayılan id rastgele bir UUID değeridir.
    pub fn new() -> Self {
        let id: SharedString = uuid::Uuid::new_v4().to_string().into();
        let id = (TypeId::of::<DefaultIdType>(), id.into());

        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            title: None,
            message: None,
            type_: None,
            icon: None,
            autohide: true,
            action_builder: None,
            content_builder: None,
            on_click: None,
            on_close: None,
            closing: false,
        }
    }

    /// Bildirim mesajını ayarlar. Varsayılan None değeridir.
    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Oluşturur bir bilgi bildirim ile verilen mesaj.
    pub fn info(message: impl Into<SharedString>) -> Self {
        Self::new().message(message).with_type(BildirimTuru::Info)
    }

    /// Bir başarı bildirim ile verilen mesaj. oluşturur.
    pub fn success(message: impl Into<SharedString>) -> Self {
        Self::new()
            .message(message)
            .with_type(BildirimTuru::Success)
    }

    /// Bir uyarı bildirim ile verilen mesaj. oluşturur.
    pub fn warning(message: impl Into<SharedString>) -> Self {
        Self::new()
            .message(message)
            .with_type(BildirimTuru::Warning)
    }

    /// Oluşturur bir hata bildirim ile verilen mesaj.
    pub fn error(message: impl Into<SharedString>) -> Self {
        Self::new().message(message).with_type(BildirimTuru::Error)
    }

    /// tip için unique identification bildirim ayarlar.
    ///
    /// ```rs
    /// struct MyNotificationKind;
    /// let notification = Bildirim::new("Hello").id::<MyNotificationKind>();
    /// ```
    pub fn id<T: Sized + 'static>(mut self) -> Self {
        self.id = TypeId::of::<T>().into();
        self
    }

    /// tip ve id bildirim, için kullanılır uniquely identify bildirim ayarlar.
    pub fn id1<T: Sized + 'static>(mut self, key: impl Into<ElementId>) -> Self {
        self.id = (TypeId::of::<T>(), key.into()).into();
        self
    }

    /// Bildirim başlığını ayarlar. Varsayılan None değeridir.
    ///
    /// Başlık None ise bildirimin başlığı olmaz.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// simge bildirim ayarlar.
    ///
    /// Simge None ise bildirim türünün varsayılan simgesi kullanılır.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Bildirim türünü ayarlar. Varsayılan BildirimTuru::Info değeridir.
    pub fn with_type(mut self, type_: BildirimTuru) -> Self {
        self.type_ = Some(type_);
        self
    }

    /// Bildirimin otomatik gizlenip gizlenmeyeceğini ayarlar. Varsayılan true değeridir.
    pub fn autohide(mut self, autohide: bool) -> Self {
        self.autohide = autohide;
        self
    }

    /// tıklama geri çağrı bildirim ayarlar.
    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// kapatır geri çağrı bildirim ayarlar.
    ///
    /// Bildirim herhangi bir nedenle kapatıldığında tetiklenir.
    /// (kapatır düğme, middle-tıklama, autohide, tıklama işleyici, veya programmatic kapatır).
    pub fn on_close(mut self, on_close: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(on_close));
        self
    }

    /// eylem düğme bildirim ayarlar.
    ///
    /// Bir eylem ayarlandığında bildirim otomatik gizlenmez.
    pub fn action<F>(mut self, action: F) -> Self
    where
        F: Fn(&mut Self, &mut Window, &mut Context<Self>) -> Dugme + 'static,
    {
        self.action_builder = Some(Rc::new(action));
        self.autohide = false;
        self
    }

    /// Dismiss bildirim.
    pub fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.closing {
            return;
        }
        self.closing = true;
        cx.notify();

        let on_close = self.on_close.clone();
        // Dismiss the notification after 0.15s to show the animation.
        cx.spawn_in(window, async move |view, cx| {
            cx.background_executor()
                .timer(Duration::from_secs_f32(0.15))
                .await;
            _ = view.update_in(cx, |view, _, cx| {
                view.closing = false;
                cx.emit(DismissEvent);
            });
            if let Some(on_close) = on_close {
                _ = cx.update(|window, cx| on_close(window, cx));
            }
        })
        .detach();
    }

    /// içerik bildirim ayarlar.
    pub fn content(
        mut self,
        content: impl Fn(&mut Self, &mut Window, &mut Context<Self>) -> AnyElement + 'static,
    ) -> Self {
        self.content_builder = Some(Rc::new(content));
        self
    }
}

impl EventEmitter<DismissEvent> for Bildirim {}
impl FluentBuilder for Bildirim {}
impl Styled for Bildirim {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for Bildirim {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = self
            .content_builder
            .clone()
            .map(|builder| builder(self, window, cx));
        let action = self
            .action_builder
            .clone()
            .map(|builder| builder(self, window, cx).small().mr_3p5());

        let closing = self.closing;
        let icon = match self.type_ {
            None => self.icon.clone(),
            Some(type_) => Some(type_.icon(cx)),
        };
        let has_icon = icon.is_some();
        let placement = cx.theme().notification.placement;

        h_flex()
            .id("notification")
            .group("")
            .occlude()
            .relative()
            .w_112()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().popover)
            .rounded(cx.theme().radius_lg)
            .shadow_md()
            .py_3p5()
            .px_4()
            .gap_3()
            .refine_style(&self.style)
            .when_some(icon, |this, icon| {
                this.child(div().absolute().top(px(18.)).left_4().child(icon))
            })
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .when(has_icon, |this| this.pl_6())
                    .when_some(self.title.clone(), |this, title| {
                        this.child(div().text_sm().font_semibold().child(title))
                    })
                    .when_some(self.message.clone(), |this, message| {
                        this.child(div().text_sm().child(message))
                    })
                    .when_some(content, |this, content| this.child(content)),
            )
            .when_some(action, |this, action| this.child(action))
            .child(
                div()
                    .absolute()
                    .top_1()
                    .right_1()
                    .invisible()
                    .group_hover("", |this| this.visible())
                    .child(
                        Dugme::new("close")
                            .icon(SimgeAdi::Close)
                            .ghost()
                            .xsmall()
                            .on_click(cx.listener(|this, _, window, cx| this.dismiss(window, cx))),
                    ),
            )
            .when_some(self.on_click.clone(), |this, on_click| {
                this.on_click(cx.listener(move |view, event, window, cx| {
                    view.dismiss(window, cx);
                    on_click(event, window, cx);
                }))
            })
            .on_aux_click(cx.listener(move |view, event: &ClickEvent, window, cx| {
                if event.is_middle_click() {
                    view.dismiss(window, cx);
                }
            }))
            .with_animation(
                ElementId::NamedInteger("slide-down".into(), closing as u64),
                Animation::new(Duration::from_secs_f64(0.25))
                    .with_easing(cubic_bezier(0.4, 0., 0.2, 1.)),
                move |this, delta| {
                    if closing {
                        let opacity = 1. - delta;
                        let that = this
                            .shadow_none()
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none());
                        match placement {
                            Anchor::TopRight | Anchor::BottomRight => {
                                let x_offset = px(0.) + delta * px(45.);
                                that.left(px(0.) + x_offset)
                            }
                            Anchor::TopLeft | Anchor::BottomLeft => {
                                let x_offset = px(0.) - delta * px(45.);
                                that.left(px(0.) + x_offset)
                            }
                            Anchor::TopCenter => {
                                let y_offset = px(0.) - delta * px(45.);
                                that.top(px(0.) + y_offset)
                            }
                            Anchor::BottomCenter => {
                                let y_offset = px(0.) + delta * px(45.);
                                that.top(px(0.) + y_offset)
                            }
                            _ => that,
                        }
                    } else {
                        let y_offset = match placement {
                            Anchor::TopLeft | Anchor::TopRight | Anchor::TopCenter => {
                                px(-45.) + delta * px(45.)
                            }
                            Anchor::BottomLeft | Anchor::BottomRight | Anchor::BottomCenter => {
                                px(45.) - delta * px(45.)
                            }
                            _ => px(0.),
                        };
                        let opacity = delta;
                        this.top(px(0.) + y_offset)
                            .opacity(opacity)
                            .when(opacity < 0.85, |this| this.shadow_none())
                    }
                },
            )
    }
}

/// ayarlar için bildirimler.
#[derive(Debug, Clone)]
pub struct BildirimAyarlari {
    /// yerleşim bildirim, varsayılan: [`Anchor::TopRight`]
    pub placement: Anchor,
    /// margins bildirim ile respect için pencere edges.
    pub margins: Edges<Pixels>,
    /// Aynı anda gösterilecek en yüksek bildirim sayısı. Varsayılan: 10.
    pub max_items: usize,
}

impl Default for BildirimAyarlari {
    fn default() -> Self {
        let offset = px(16.);
        Self {
            placement: Anchor::TopRight,
            margins: Edges {
                top: TITLE_BAR_HEIGHT + offset, // avoid overlap with title bar
                right: offset,
                bottom: offset,
                left: offset,
            },
            max_items: 10,
        }
    }
}

/// Bir liste bildirimler.
pub struct BildirimListesi {
    /// Bildirimler otomatik gizlenir.
    pub(crate) notifications: VecDeque<Entity<Bildirim>>,
    expanded: bool,
    _subscriptions: HashMap<BildirimKimligi, Subscription>,
}

impl BildirimListesi {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            notifications: VecDeque::new(),
            expanded: false,
            _subscriptions: HashMap::new(),
        }
    }

    pub fn push(
        &mut self,
        notification: impl Into<Bildirim>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let notification = notification.into();
        let id = notification.id.clone();
        let autohide = notification.autohide;

        // Remove the notification by id, for keep unique.
        self.notifications.retain(|note| note.read(cx).id != id);

        let notification = cx.new(|_| notification);

        self._subscriptions.insert(
            id.clone(),
            cx.subscribe(&notification, move |view, _, _: &DismissEvent, cx| {
                view.notifications.retain(|note| id != note.read(cx).id);
                view._subscriptions.remove(&id);
            }),
        );

        self.notifications.push_back(notification.clone());
        if autohide {
            // Sleep for 5 seconds to autohide the notification
            cx.spawn_in(window, async move |_, cx| {
                cx.background_executor().timer(Duration::from_secs(5)).await;

                if let Err(err) =
                    notification.update_in(cx, |note, window, cx| note.dismiss(window, cx))
                {
                    tracing::error!("bildirim otomatik gizlenemedi: {:?}", err);
                }
            })
            .detach();
        }
        cx.notify();
    }

    pub(crate) fn close(
        &mut self,
        id: impl Into<BildirimKimligi>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id: BildirimKimligi = id.into();
        if let Some(n) = self.notifications.iter().find(|n| n.read(cx).id == id) {
            n.update(cx, |note, cx| note.dismiss(window, cx))
        }
        cx.notify();
    }

    /// Close tüm bildirimler whose id matches verilen [`TypeId`], regardless nin
    /// olup olmadığını they idi registered aracılığıyla [`Bildirim::id`] veya [`Bildirim::id1`].
    pub(crate) fn close_by_type(
        &mut self,
        type_id: TypeId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let matched: Vec<_> = self
            .notifications
            .iter()
            .filter(|n| match &n.read(cx).id {
                BildirimKimligi::Id(t) | BildirimKimligi::IdAndElementId(t, _) => *t == type_id,
            })
            .cloned()
            .collect();
        for n in matched {
            n.update(cx, |note, cx| note.dismiss(window, cx));
        }
        cx.notify();
    }

    pub fn clear(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.notifications.clear();
        cx.notify();
    }

    pub fn notifications(&self) -> Vec<Entity<Bildirim>> {
        self.notifications.iter().cloned().collect()
    }
}

impl Render for BildirimListesi {
    fn render(
        &mut self,
        window: &mut crate::ham_gpui::Window,
        cx: &mut crate::ham_gpui::Context<Self>,
    ) -> impl IntoElement {
        let size = window.viewport_size();
        let items = self.notifications.iter().rev().take(10).rev().cloned();

        let placement = cx.theme().notification.placement;
        let margins = &cx.theme().notification.margins;

        v_flex()
            .id("notification-list")
            .max_h(size.height)
            .pt(margins.top)
            .pb(margins.bottom)
            .gap_3()
            .when(
                matches!(placement, Anchor::TopRight),
                |this| this.pr(margins.right), // ignore left
            )
            .when(
                matches!(placement, Anchor::TopLeft),
                |this| this.pl(margins.left), // ignore right
            )
            .when(
                matches!(placement, Anchor::BottomLeft),
                |this| this.flex_col_reverse().pl(margins.left), // ignore right
            )
            .when(
                matches!(placement, Anchor::BottomRight),
                |this| this.flex_col_reverse().pr(margins.right), // ignore left
            )
            .when(matches!(placement, Anchor::BottomCenter), |this| {
                this.flex_col_reverse()
            })
            .on_hover(cx.listener(|view, hovered, _, cx| {
                view.expanded = *hovered;
                cx.notify()
            }))
            .children(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ham_gpui::{TestAppContext, VisualTestContext};
    use crate::theme::Tema;

    struct FooKind;
    struct BarKind;

    struct TestRoot {
        list: Entity<BildirimListesi>,
    }

    impl Render for TestRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.list.clone()
        }
    }

    fn ids(list: &Entity<BildirimListesi>, cx: &mut VisualTestContext) -> Vec<BildirimKimligi> {
        list.read_with(cx, |l, cx| {
            l.notifications
                .iter()
                .map(|n| n.read(cx).id.clone())
                .collect()
        })
    }

    /// Drive dismiss animasyon zamanlayıcı + propagate resulting `DismissEvent`
    /// Böylece kapalı bildirimler listeden kaldırılır.
    fn flush_dismiss(cx: &mut VisualTestContext) {
        cx.background_executor
            .advance_clock(Duration::from_millis(200));
        cx.run_until_parked();
    }

    #[crate::ham_gpui::test]
    fn close_by_type_removes_id_and_all_id1_of_same_type(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Tema::default()));
        let (root, cx) = cx.add_window_view(|window, cx| TestRoot {
            list: cx.new(|cx| BildirimListesi::new(window, cx)),
        });
        let list = root.read_with(cx, |r, _| r.list.clone());

        list.update_in(cx, |list, window, cx| {
            list.push(
                Bildirim::info("plain").id::<FooKind>().autohide(false),
                window,
                cx,
            );
            list.push(
                Bildirim::info("a").id1::<FooKind>(1).autohide(false),
                window,
                cx,
            );
            list.push(
                Bildirim::info("b").id1::<FooKind>(2).autohide(false),
                window,
                cx,
            );
            list.push(
                Bildirim::info("bar").id::<BarKind>().autohide(false),
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(ids(&list, cx).len(), 4);

        list.update_in(cx, |list, window, cx| {
            list.close_by_type(TypeId::of::<FooKind>(), window, cx);
        });
        flush_dismiss(cx);

        let remaining = ids(&list, cx);
        assert_eq!(
            remaining,
            vec![BildirimKimligi::Id(TypeId::of::<BarKind>())],
            "only the BarKind notification should survive"
        );
    }

    #[crate::ham_gpui::test]
    fn close_with_id_and_element_id_removes_only_matching_key(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Tema::default()));
        let (root, cx) = cx.add_window_view(|window, cx| TestRoot {
            list: cx.new(|cx| BildirimListesi::new(window, cx)),
        });
        let list = root.read_with(cx, |r, _| r.list.clone());

        list.update_in(cx, |list, window, cx| {
            list.push(
                Bildirim::info("a").id1::<FooKind>(1).autohide(false),
                window,
                cx,
            );
            list.push(
                Bildirim::info("b").id1::<FooKind>(2).autohide(false),
                window,
                cx,
            );
            list.push(
                Bildirim::info("plain").id::<FooKind>().autohide(false),
                window,
                cx,
            );
        });

        list.update_in(cx, |list, window, cx| {
            list.close(
                (TypeId::of::<FooKind>(), ElementId::from(1usize)),
                window,
                cx,
            );
        });
        flush_dismiss(cx);

        let remaining = ids(&list, cx);
        assert_eq!(remaining.len(), 2);
        assert!(remaining.contains(&BildirimKimligi::IdAndElementId(
            TypeId::of::<FooKind>(),
            ElementId::from(2usize),
        )));
        assert!(remaining.contains(&BildirimKimligi::Id(TypeId::of::<FooKind>())));
    }

    #[crate::ham_gpui::test]
    fn close_with_only_type_id_does_not_match_id1_entries(cx: &mut TestAppContext) {
        // The plain `close(TypeId)` form (used by the legacy code path) must keep
        // its narrow semantics: it only matches `BildirimKimligi::Id`, not
        // `BildirimKimligi::IdAndElementId`. The new `close_by_type` is the broad form.
        cx.update(|cx| cx.set_global(Tema::default()));
        let (root, cx) = cx.add_window_view(|window, cx| TestRoot {
            list: cx.new(|cx| BildirimListesi::new(window, cx)),
        });
        let list = root.read_with(cx, |r, _| r.list.clone());

        list.update_in(cx, |list, window, cx| {
            list.push(
                Bildirim::info("a").id1::<FooKind>(1).autohide(false),
                window,
                cx,
            );
        });

        list.update_in(cx, |list, window, cx| {
            list.close(TypeId::of::<FooKind>(), window, cx);
        });
        flush_dismiss(cx);

        assert_eq!(ids(&list, cx).len(), 1, "id1 entry should remain untouched");
    }

    #[crate::ham_gpui::test]
    fn close_by_type_with_no_match_is_noop(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Tema::default()));
        let (root, cx) = cx.add_window_view(|window, cx| TestRoot {
            list: cx.new(|cx| BildirimListesi::new(window, cx)),
        });
        let list = root.read_with(cx, |r, _| r.list.clone());

        list.update_in(cx, |list, window, cx| {
            list.push(
                Bildirim::info("bar").id::<BarKind>().autohide(false),
                window,
                cx,
            );
        });

        list.update_in(cx, |list, window, cx| {
            list.close_by_type(TypeId::of::<FooKind>(), window, cx);
        });
        flush_dismiss(cx);

        assert_eq!(ids(&list, cx).len(), 1);
    }
}
