use crate::{ItemHandle, Pane, StatusItemView};
use gpui::{
    AnyView, App, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
};
use ui::{h_flex, prelude::*};

trait EditorStatusItemViewHandle: Send {
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    );
}

/// A floating status display for editor-specific information like cursor position,
/// language, and vim mode. Rendered as an overlay in the bottom-right of the active pane.
pub struct EditorPaneStatus {
    items: Vec<Box<dyn EditorStatusItemViewHandle>>,
    active_pane: Entity<Pane>,
    _observe_active_pane: Subscription,
}

impl Render for EditorPaneStatus {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let bg_color = colors.editor_background;

        h_flex()
            .absolute()
            .bottom_2()
            .right_2()
            .gap_2()
            .p_1()
            .bg(bg_color)
            .rounded_md()
            .children(self.items.iter().map(|item| item.to_any()))
    }
}

impl EditorPaneStatus {
    pub fn new(active_pane: &Entity<Pane>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            items: Default::default(),
            active_pane: active_pane.clone(),
            _observe_active_pane: cx.observe_in(active_pane, window, |this, _, window, cx| {
                this.update_active_pane_item(window, cx)
            }),
        };
        this.update_active_pane_item(window, cx);
        this
    }

    pub fn add_item<T>(&mut self, item: Entity<T>, window: &mut Window, cx: &mut Context<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), window, cx);

        self.items.push(Box::new(item));
        cx.notify();
    }

    pub fn set_active_pane(
        &mut self,
        active_pane: &Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_pane = active_pane.clone();
        self._observe_active_pane = cx.observe_in(active_pane, window, |this, _, window, cx| {
            this.update_active_pane_item(window, cx)
        });
        self.update_active_pane_item(window, cx);
    }

    fn update_active_pane_item(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let active_pane_item = self.active_pane.read(cx).active_item();
        for item in &self.items {
            item.set_active_pane_item(active_pane_item.as_deref(), window, cx);
        }
    }
}

impl<T: StatusItemView> EditorStatusItemViewHandle for Entity<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, window, cx)
        });
    }
}
