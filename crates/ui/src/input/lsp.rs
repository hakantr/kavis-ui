use crate::ham_gpui::{App, Context, Hsla, MouseMoveEvent, Task, Window};
use anyhow::Result;
use ropey::Rope;
use std::rc::Rc;

use crate::input::{InputState, RopeExt, popovers::BaglamMenusu};

mod code_actions;
mod completions;
mod definitions;
mod document_colors;
mod hover;

pub use code_actions::*;
pub use completions::*;
pub use definitions::*;
pub use document_colors::*;
pub use hover::*;

/// LSP ServerCapabilities
///
/// https://microsoft.github.io/dil-server-protocol/specifications/lsp/3.17/specification/#serverCapabilities
pub struct Lsp {
    /// tamamlama sağlayıcı.
    pub completion_provider: Option<Rc<dyn CompletionProvider>>,
    /// kod eylem sağlayıcılar.
    pub code_action_providers: Vec<Rc<dyn CodeActionProvider>>,
    /// üzerine gelme sağlayıcı.
    pub hover_provider: Option<Rc<dyn HoverProvider>>,
    /// tanım sağlayıcı.
    pub definition_provider: Option<Rc<dyn DefinitionProvider>>,
    /// document renk sağlayıcı.
    pub document_color_provider: Option<Rc<dyn DocumentColorProvider>>,

    document_colors: Vec<(lsp_types::Range, Hsla)>,
    _hover_task: Task<Result<()>>,
    _document_color_task: Task<()>,
}

impl Default for Lsp {
    fn default() -> Self {
        Self {
            completion_provider: None,
            code_action_providers: vec![],
            hover_provider: None,
            definition_provider: None,
            document_color_provider: None,
            document_colors: vec![],
            _hover_task: Task::ready(Ok(())),
            _document_color_task: Task::ready(()),
        }
    }
}

impl Lsp {
    /// LSP olduğunda metin değişimler. günceller.
    pub(crate) fn update(
        &mut self,
        text: &Rope,
        window: &mut Window,
        cx: &mut Context<InputState>,
    ) {
        self.update_document_colors(text, window, cx);
    }

    /// Sıfırlar tüm LSP states.
    pub(crate) fn reset(&mut self) {
        self.document_colors.clear();
        self._hover_task = Task::ready(Ok(()));
        self._document_color_task = Task::ready(());
    }
}

impl InputState {
    pub(crate) fn hide_context_menu(&mut self, cx: &mut Context<Self>) {
        self.context_menu_content = None;
        self._context_menu_task = Task::ready(Ok(()));
        cx.notify();
    }

    pub(crate) fn is_context_menu_open(&self, cx: &App) -> bool {
        let Some(menu) = self.context_menu_content.as_ref() else {
            return false;
        };

        menu.is_open(cx)
    }

    /// İşler bir eylem için tamamlama menü, ise onu exists.
    ///
    /// Eylem işlendiyse true, aksi halde false döndürür.
    pub fn handle_action_for_context_menu(
        &mut self,
        action: Box<dyn crate::ham_gpui::Action>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(menu) = self.context_menu_content.as_ref() else {
            return false;
        };

        let mut handled = false;

        match menu {
            BaglamMenusu::Completion(menu) => {
                _ = menu.update(cx, |menu, cx| {
                    handled = menu.handle_action(action, window, cx)
                });
            }
            BaglamMenusu::CodeAction(menu) => {
                _ = menu.update(cx, |menu, cx| {
                    handled = menu.handle_action(action, window, cx)
                });
            }
            BaglamMenusu::RightClick(..) => {}
        };

        handled
    }

    /// bir liste [`lsp_types::TextEdit`] için mutate metin. uygular.
    pub fn apply_lsp_edits(
        &mut self,
        text_edits: &Vec<lsp_types::TextEdit>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for edit in text_edits {
            let start = self.text.position_to_offset(&edit.range.start);
            let end = self.text.position_to_offset(&edit.range.end);

            let range_utf16 = self.range_to_utf16(&(start..end));
            self.replace_text_in_range_silent(Some(range_utf16), &edit.new_text, window, cx);
        }
    }

    pub(super) fn handle_mouse_move(
        &mut self,
        offset: usize,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<InputState>,
    ) {
        if event.modifiers.secondary() {
            self.handle_hover_definition(offset, window, cx);
        } else {
            self.hover_definition.clear();
            self.handle_hover_popover(offset, window, cx);
        }
        cx.notify();
    }

    pub(crate) fn clear_hover_state(&mut self, cx: &mut Context<InputState>) {
        self.hover_definition.clear();
        self.hover_popover = None;
        self.lsp._hover_task = Task::ready(Ok(()));
        cx.notify();
    }
}
