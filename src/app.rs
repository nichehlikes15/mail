use gpui::{App, Context, Entity, Window, WindowOptions, div, prelude::*, px, rgb, size};
//use std::{ops::{Deref, DerefMut},sync::Arc,};

use crate::models::{TempEmail, Theme};
use crate::ui::{Inbox, Sidebar, TopBar};

pub struct MailApp {
    pub sidebar: Entity<Sidebar>,
    pub topbar: Entity<TopBar>,
    pub inbox: Entity<Inbox>,
    pub state: Entity<AppState>,
    pub theme: Theme,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub temp_email: Vec<TempEmail>, 
    pub current_temp_email: Option<usize>,
    pub selected_sidebar_email: Option<SidebarEmail>,
    pub creating_temp_email: bool,
    pub temp_email_spinner_frame: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SidebarEmail {
    Mail(usize),
    Temp(usize),
}

impl MailApp {
    pub fn open(cx: &mut App) {
        let font = include_bytes!("../assets/fonts/Lilex[wght].ttf");

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
                let theme = Theme::load();
                let state = cx.new(|_| AppState {
                    temp_email: Vec::new(),
                    current_temp_email: None,
                    selected_sidebar_email: None,
                    creating_temp_email: false,
                    temp_email_spinner_frame: 0,
                });
                let sidebar = cx.new(|_| Sidebar {
                    state: state.clone(),
                    theme: theme.clone(),
                    spinner_task: None,
                });
                let topbar = cx.new(|_| TopBar { theme: theme.clone() });

                let inbox = cx.new(|cx| Inbox::new(state.clone(), theme.clone(), cx));

                cx.new(|_| MailApp {
                    sidebar,
                    topbar,
                    inbox,
                    state,
                    theme,
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
            .bg(rgb(Theme::color(&self.theme.inbox_background)))
            .text_color(rgb(Theme::color(&self.theme.inbox_text)))
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
