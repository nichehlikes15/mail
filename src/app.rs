use gpui::{App, Context, Entity, Global, Window, WindowOptions, div, prelude::*, px, rgb, size};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::models::TempEmail;
use crate::ui::{Inbox, Sidebar, TopBar};

pub struct MailApp {
    pub sidebar: Entity<Sidebar>,
    pub topbar: Entity<TopBar>,
    pub inbox: Entity<Inbox>,
    pub state: Entity<AppState>,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub temp_email: Option<TempEmail>,
}

impl MailApp {
    pub fn open(cx: &mut App) {
        let font = include_bytes!("assets/fonts/Lilex[wght].ttf");

        cx.text_system()
            .add_fonts(vec![std::borrow::Cow::Borrowed(font.as_slice())])
            .expect("Failed to load Lilex font");

        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                    None,
                    size(px(1200.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                let state = cx.new(|_| AppState { temp_email: None });
                let sidebar = cx.new(|_| Sidebar {
                    state: state.clone(),
                });
                let topbar = cx.new(|_| TopBar);

                let inbox = cx.new(|cx| Inbox::new(state.clone(), cx));

                inbox.update(cx, |inbox, cx| {
                    inbox.refresh(cx);
                });

                cx.new(|_| MailApp {
                    sidebar,
                    topbar,
                    inbox,
                    state,
                })
            },
        )
        .unwrap();
    }
}

impl Render for MailApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x000000))
            .text_color(rgb(0xffffff))
            .font_family("Lilex")
            .flex()
            .flex_col()
            .child(self.topbar.clone())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .child(self.inbox.clone())
                    .child(self.sidebar.clone()),
            )
    }
}
