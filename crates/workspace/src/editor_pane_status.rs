use gpui::{AnyView, Context, IntoElement, ParentElement, Render, Styled, Window};
use ui::{h_flex, prelude::*};

/// A floating status display rendered as an overlay in the bottom-right of the active pane.
/// This component simply renders existing status item views - it doesn't manage them.
pub struct EditorPaneStatus {
    items: Vec<AnyView>,
}

impl EditorPaneStatus {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: AnyView, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }
}

impl Render for EditorPaneStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        h_flex()
            .absolute()
            .bottom_2()
            .right_2()
            .gap_2()
            .p_2()
            .bg(colors.editor_background)
            .rounded_md()
            .children(self.items.iter().cloned())
    }
}
