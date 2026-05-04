use crate::ham_gpui::{AnyElement, ParentElement, RenderOnce, StyleRefinement};
use smallvec::SmallVec;

use crate::{Secilebilir, StilUzantisi, list::ListeOgesi};

pub struct ListeAyiriciOgesi {
    style: StyleRefinement,
    children: SmallVec<[AnyElement; 2]>,
}

impl ListeAyiriciOgesi {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ListeAyiriciOgesi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Secilebilir for ListeAyiriciOgesi {
    fn selected(self, _: bool) -> Self {
        self
    }

    fn is_selected(&self) -> bool {
        false
    }
}

impl RenderOnce for ListeAyiriciOgesi {
    fn render(
        self,
        _: &mut crate::ham_gpui::Window,
        _: &mut crate::ham_gpui::App,
    ) -> impl crate::ham_gpui::IntoElement {
        ListeOgesi::new("separator")
            .refine_style(&self.style)
            .children(self.children)
            .disabled(true)
    }
}
