use gpui::{
    AnyElement, App, ClickEvent, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    Pixels, RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
};

use crate::{
    PencereUzantisi as _, StyledExt as _,
    dialog::{
        IletisimAciklamasi, IletisimAltligi, IletisimBasligi, IletisimBaslikMetni,
        IletisimDugmesiOzellikleri, IletisimKutusu,
    },
    h_flex, v_flex,
};

/// UyariIletisimKutusu, önemli içerikle kullanıcı akışını kesen modal bir iletişim kutusudur.
/// ve expects bir response.
///
/// IletisimKutusu bileşeni üzerine kuruludur ve belirli varsayılanlar kullanır:
/// - Alt bilgi düğmeleri ortalanır (IletisimKutusu içinde sağa hizalanır)
/// - Simge isteğe bağlıdır (varsayılan olarak kapalıdır, `.show_icon(true)` ile açılır)
/// - Simplified API için ortak uyarı scenarios
/// - Deklaratif IletisimBasligi, IletisimBaslikMetni, IletisimAciklamasi ve IletisimAltligi bileşenlerini kullanır
/// - Supports ikisi de imperative ve declarative API stiller
///
/// # Örnekler
///
/// ## Imperative API (kullanarak PencereUzantisi)
///
/// ```ignore
/// use kavis_ui::{UyariIletisimKutusu, alert::UyariVaryanti};
///
/// // Using PencereUzantisi trait
/// window.open_alert_dialog(cx, |alert, _, _| {
///     alert
///         .title("Unsaved Changes")
///         .description("You have unsaved changes. Are you sure you want to leave?")
///         .show_cancel(true)
/// });
/// ```
///
/// ## Declarative API (kullanarak tetikleyici ve içerik)
///
/// ```ignore
/// use kavis_ui::{UyariIletisimKutusu, IletisimBasligi, IletisimBaslikMetni, IletisimAciklamasi, IletisimAltligi};
///
/// UyariIletisimKutusu::new(cx)
///     .trigger(Dugme::new("delete").label("Delete"))
///     .content(|content, _, cx| {
///         content
///             .child(
///                 IletisimBasligi::new()
///                     .items_center()
///                     .child(IletisimBaslikMetni::new().child("Delete File"))
///                     .child(IletisimAciklamasi::new().child("Are you sure?"))
///             )
///             .child(
///                 IletisimAltligi::new()
///                     .justify_center()
///                     .child(Dugme::new("cancel").label("Cancel"))
///                     .child(Dugme::new("confirm").label("Delete"))
///             )
///     })
/// ```
#[derive(IntoElement)]
pub struct UyariIletisimKutusu {
    base: IletisimKutusu,
    trigger: Option<AnyElement>,
    icon: Option<AnyElement>,
    title: Option<AnyElement>,
    description: Option<AnyElement>,
    button_props: IletisimDugmesiOzellikleri,
    children: Vec<AnyElement>,
}

impl UyariIletisimKutusu {
    /// Yeni bir UyariIletisimKutusu oluşturur.
    ///
    /// Varsayılan olarak iletişim kutusu kaplamayla kapatılamaz ve OK düğmesi kullanır.
    ///
    /// Bunu `.overlay_closable(true)` ile değiştirebilirsiniz.
    pub fn new(cx: &mut App) -> Self {
        Self {
            base: IletisimKutusu::new(cx)
                .overlay_closable(false)
                .close_button(false),
            trigger: None,
            icon: None,
            title: None,
            description: None,
            button_props: IletisimDugmesiOzellikleri::default(),
            children: Vec::new(),
        }
    }

    /// OK ve Cancel düğmeleriyle onay iletişim kutusu olarak kullanmayı ayarlar.
    ///
    /// Varsayılan [`UyariIletisimKutusu`] OK düğmesine sahiptir.
    pub fn confirm(mut self) -> Self {
        self.button_props.show_cancel = true;
        self
    }

    /// tetikleyici öğe için uyarı iletişim kutusu ayarlar.
    ///
    /// Tetikleyici ayarlanırsa iletişim kutusu, tıklandığında açan bir tetikleyici öğe olarak çizilir.
    ///
    /// **Note**: Olduğunda kullanarak `.trigger()`, siz olmalı ayrıca kullanım `.içerik()` için define iletişim kutusu içerik
    /// declaratively bunun yerine kullanarak `.başlık()`, `.açıklama()`, etc.
    ///
    /// `.trigger()` ile birlikte kullanıldığında `başlık`, `açıklama`, `simge` ve `button_props` yok sayılır.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// içerik oluşturucu için declarative API ayarlar.
    ///
    /// Olduğunda kullanarak bu yöntem, siz define iletişim kutusu içerik kullanarak declarative bileşenler gibi
    /// `IletisimBasligi`, `IletisimBaslikMetni`, `IletisimAciklamasi`, ve `IletisimAltligi`.
    ///
    /// Bu yöntem genellikle tamamen deklaratif bir API için `.trigger()` ile birlikte kullanılır.
    ///
    /// # Örnekler
    ///
    /// ```ignore
    /// UyariIletisimKutusu::new(cx)
    ///     .trigger(Dugme::new("delete").label("Delete"))
    ///     .content(|content, _, cx| {
    ///         content
    ///             .child(IletisimBasligi::new().child(IletisimBaslikMetni::new().child("Confirm")))
    ///             .child(IletisimAltligi::new().child(Dugme::new("ok").label("OK")))
    ///     })
    /// ```
    pub fn content<F>(mut self, builder: F) -> Self
    where
        F: Fn(
                crate::dialog::IletisimIcerigi,
                &mut Window,
                &mut App,
            ) -> crate::dialog::IletisimIcerigi
            + 'static,
    {
        self.base = self.base.content(builder);
        self
    }

    /// alt bilgi oluşturucu için declarative API ayarlar.
    ///
    /// Bu için kullanılır define alt bilgi içerik kullanarak declarative bileşenler gibi `IletisimAltligi`.
    ///
    /// Ayarlanmazsa OK ve isteğe bağlı Cancel düğmesi içeren varsayılan alt bilgi kullanılır.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.base = self.base.footer(footer);
        self
    }

    #[track_caller]
    fn debug_assert_no_trigger(&self) {
        debug_assert!(
            self.trigger.is_none() && self.base.content_builder.is_none(),
            "Cannot set this property when trigger is used. Use content() to define dialog content instead."
        );
    }

    /// Uyarı iletişim kutusunun simgesini ayarlar. Varsayılan None değeridir.
    #[track_caller]
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.debug_assert_no_trigger();
        self.icon = Some(icon.into_any_element());
        self
    }

    /// başlık uyarı iletişim kutusu ayarlar.
    #[track_caller]
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.debug_assert_no_trigger();
        self.title = Some(title.into_any_element());
        self
    }

    /// açıklama uyarı iletişim kutusu ayarlar.
    #[track_caller]
    pub fn description(mut self, description: impl IntoElement) -> Self {
        self.debug_assert_no_trigger();
        self.description = Some(description.into_any_element());
        self
    }

    /// Uyarı iletişim kutusu düğme özelliklerini ayarlar.
    ///
    /// Kullanım bu için configure düğme metin, variants, ve görünürlük.
    ///
    /// # Örnekler
    ///
    /// ```ignore
    /// alert.button_props(
    ///     IletisimDugmesiOzellikleri::default()
    ///         .ok_text("Delete")
    ///         .ok_variant(DugmeVaryanti::Danger)
    ///         .cancel_text("Keep")
    ///         .show_cancel(true)
    /// )
    /// ```
    #[track_caller]
    pub fn button_props(mut self, button_props: IletisimDugmesiOzellikleri) -> Self {
        self.debug_assert_no_trigger();
        self.button_props = button_props;
        self
    }

    /// genişlik uyarı iletişim kutusu, varsayılan değer 420px ayarlar.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.base = self.base.width(width);
        self
    }

    /// Cancel düğmesinin gösterilip gösterilmeyeceğini ayarlar. Varsayılan false değeridir.
    pub fn show_cancel(mut self, show_cancel: bool) -> Self {
        self.button_props = self.button_props.show_cancel(show_cancel);
        self
    }

    /// kaplama kapatılabilir uyarı iletişim kutusu, varsayılan değer `false` ayarlar.
    ///
    /// Kaplama tıklandığında iletişim kutusu kapanır.
    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.base = self.base.overlay_closable(overlay_closable);
        self
    }

    /// kapatır düğme uyarı iletişim kutusu, varsayılan değer `false` ayarlar.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.base = self.base.close_button(close_button);
        self
    }

    /// olup olmadığını için destek klavye esc için kapatır iletişim kutusu, varsayılan değer `true` ayarlar.
    pub fn keyboard(mut self, keyboard: bool) -> Self {
        self.base = self.base.keyboard(keyboard);
        self
    }

    /// Uyarı iletişim kutusu kapandığında çağrılacak geri çağrıyı ayarlar.
    ///
    /// [`Self::on_action`] veya [`Self::on_cancel`] geri çağrısından sonra çağrılır.
    pub fn on_close(
        mut self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_close(on_close);
        self
    }

    /// OK/eylem düğmesi tıklandığında çağrılacak geri çağrıyı ayarlar.
    ///
    /// geri çağrı olmalı döndürür `true` için kapatır iletişim kutusu, ise döndürür `false` iletişim kutusu olmayacak olmak kapalı.
    pub fn on_ok(
        mut self,
        on_ok: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.button_props = self.button_props.on_ok(on_ok);
        self
    }

    /// Uyarı iletişim kutusu iptal edildiğinde çağrılacak geri çağrıyı ayarlar.
    ///
    /// geri çağrı olmalı döndürür `true` için kapatır iletişim kutusu, ise döndürür `false` iletişim kutusu olmayacak olmak kapalı.
    pub fn on_cancel(
        mut self,
        on_cancel: impl Fn(&ClickEvent, &mut Window, &mut App) -> bool + 'static,
    ) -> Self {
        self.button_props = self.button_props.on_cancel(on_cancel);
        self
    }

    /// UyariIletisimKutusu içine bir configured IletisimKutusu. dönüştürür.
    pub(crate) fn into_dialog(self, window: &mut Window, cx: &mut App) -> IletisimKutusu {
        let button_props = self.button_props.clone();
        let has_title = self.icon.is_some() || self.title.is_some();
        let has_header = has_title || self.description.is_some();
        let has_footer = self.base.footer.is_some();

        self.base
            .button_props(button_props.clone())
            .when(has_header, |this| {
                this.header(
                    IletisimBasligi::new().child(
                        h_flex()
                            .gap_2()
                            .items_start()
                            .when_some(self.icon, |row, icon| row.child(icon))
                            .child(
                                v_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .gap_1()
                                    .when_some(self.title, |this, title| {
                                        this.child(IletisimBaslikMetni::new().child(title))
                                    })
                                    .when_some(self.description, |this, desc| {
                                        this.child(IletisimAciklamasi::new().child(desc))
                                    }),
                            ),
                    ),
                )
            })
            .children(self.children)
            .when(!has_footer, |this| {
                // Default footer for UyariIletisimKutusu if user doesn't provide one, with OK and optional Cancel button
                this.footer(
                    IletisimAltligi::new()
                        .when(button_props.show_cancel, |this| {
                            this.child(button_props.render_cancel(window, cx))
                        })
                        .child(button_props.render_ok(window, cx)),
                )
            })
    }
}

impl Styled for UyariIletisimKutusu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.base.style
    }
}

impl ParentElement for UyariIletisimKutusu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl UyariIletisimKutusu {
    fn render_trigger(self, trigger: AnyElement, _: &mut Window, _: &mut App) -> AnyElement {
        let content_builder = self.base.content_builder.clone();
        let style = self.base.style.clone();
        let props = self.base.props.clone();
        let button_props = self.button_props.clone();

        div()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                let content_builder = content_builder.clone();
                let style = style.clone();
                let props = props.clone();
                let button_props = button_props.clone();
                window.open_dialog(cx, move |dialog, _, _| {
                    dialog
                        .refine_style(&style)
                        .button_props(button_props.clone())
                        .with_props(props.clone())
                        .when_some(content_builder.clone(), |this, content_builder| {
                            this.content(move |content, window, cx| {
                                content_builder(content, window, cx)
                            })
                        })
                });
                cx.stop_propagation();
            })
            .child(trigger)
            .into_any_element()
    }
}

impl RenderOnce for UyariIletisimKutusu {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(trigger) = self.trigger.take() {
            // If a trigger is provided, render the trigger element that opens the dialog
            self.render_trigger(trigger, window, cx)
        } else {
            // Otherwise, render the dialog content directly
            self.into_dialog(window, cx).into_any_element()
        }
    }
}
