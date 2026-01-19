use gpui::{AnyView, Context, IntoElement, ParentElement, Render, Styled, Window};
use ui::h_flex;

/// Container for status items that should be mirrored to the title bar.
/// This component holds views that are also rendered in the status bar,
/// ensuring they stay in sync automatically.
pub struct TitleBarItems {
    items: Vec<AnyView>,
}

impl TitleBarItems {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: AnyView, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }
}

impl Render for TitleBarItems {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .children(self.items.iter().cloned())
    }
}
