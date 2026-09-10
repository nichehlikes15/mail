use std::time::Duration;

use gpui::{Entity, Task, Window, div, prelude::*, px, rgb, svg};

use crate::app::SidebarEmail;
use crate::models::{Theme, create_account};
pub struct Sidebar {
    pub state: Entity<crate::app::AppState>,
    pub theme: Theme,
    pub(crate) spinner_task: Option<Task<()>>,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, root_cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.state.clone();
        let mail_account_0_state = self.state.clone();
        let mail_account_1_state = self.state.clone();
        let creating_temp_email = self.state.read(root_cx).creating_temp_email;
        let spinner_frame = self.state.read(root_cx).temp_email_spinner_frame;
        let selected_sidebar_email = self.state.read(root_cx).selected_sidebar_email;
        let temporary_emails = self.state.read(root_cx).temp_email.iter().enumerate().map(|(index, email)| {
                let app_state = self.state.clone();
                div()
                    .id(format!("temp-email-{index}"))
                    .px(px(8.0))
                    .py(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgb(Theme::color(&self.theme.sidebar_text)))
                    .when(selected_sidebar_email == Some(SidebarEmail::Temp(index)), |row| {
                        row.bg(rgb(Theme::color(&self.theme.sidebar_selected_background)))
                    })
                    .hover(|row| row.text_color(rgb(0xffffff)))
                    .cursor_pointer()
                    .on_click(move |_event, _window, cx| {
                        app_state.update(cx, |state, cx| {
                            state.selected_email = Some(index);
                            state.selected_sidebar_email = Some(SidebarEmail::Temp(index));
                            cx.notify();
                        });
                    })
                    .child(email.address.clone())
            }).collect::<Vec<_>>();

        div()
            .w(px(360.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(Theme::color(&self.theme.sidebar_background)))
            .border_l(px(1.0))
            .border_color(rgb(Theme::color(&self.theme.sidebar_border)))
            .child(
                div()
                    .w_full()
                    .px(px(14.0))
                    .py(px(12.0))
                    .child(
                        div()
                            .h(px(30.0))
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .text_size(px(14.0))
                            .text_color(rgb(Theme::color(&self.theme.sidebar_header_text)))
                            .child("Mail")
                            .child(
                                div()
                                    .h(px(30.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(17.5))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_inactive_text)))
                                    .child("+"),
                            ),
                    )
                    .child(
                        div()
                            .ml(px(8.0))
                            .pl(px(14.0))
                            .border_l(px(1.0))
                            .border_color(rgb(Theme::color(&self.theme.sidebar_tree_border)))
                            .child(
                                div()
                                    .id("mail-account-0")
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(Theme::color(&self.theme.sidebar_text)))
                                    .when(selected_sidebar_email == Some(SidebarEmail::Mail(0)), |row| {
                                        row.bg(rgb(Theme::color(&self.theme.sidebar_selected_background)))
                                    })
                                    .hover(|row| row.text_color(rgb(0xffffff)))
                                    .cursor_pointer()
                                    .on_click(root_cx.listener(move |_this, _event, _window, cx| {
                                        let app_state = mail_account_0_state.clone();
                                        app_state.update(cx, |state, cx| {
                                            state.selected_sidebar_email = Some(SidebarEmail::Mail(0));
                                            cx.notify();
                                        });
                                    }))
                                    .child("oliver@gmail.com"),
                            )
                            .child(
                                div()
                                    .id("mail-account-1")
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(Theme::color(&self.theme.sidebar_text)))
                                    .when(selected_sidebar_email == Some(SidebarEmail::Mail(1)), |row| {
                                        row.bg(rgb(Theme::color(&self.theme.sidebar_selected_background)))
                                    })
                                    .hover(|row| row.text_color(rgb(0xffffff)))
                                    .cursor_pointer()
                                    .on_click(root_cx.listener(move |_this, _event, _window, cx| {
                                        let app_state = mail_account_1_state.clone();
                                        app_state.update(cx, |state, cx| {
                                            state.selected_sidebar_email = Some(SidebarEmail::Mail(1));
                                            cx.notify();
                                        });
                                    }))
                                    .child("rem@googlemail.com"),
                            ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .px(px(14.0))
                    .py(px(12.0))
                    .child(
                        div()
                            .h(px(30.0))
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .text_size(px(14.0))
                            .text_color(rgb(Theme::color(&self.theme.sidebar_header_text)))
                            .child("Temp Emails")
                            .child(
                                div()
                                    .h(px(30.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(17.5))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_inactive_text)))
                                    .child(if creating_temp_email {
                                        [".", "..", "..."][spinner_frame]
                                    } else {
                                        "+"
                                    })
                                    .id("generate-email")
                                    .cursor_pointer()
                                    .on_click(root_cx.listener(move |this, _event, _window, cx| {
                                        if this.state.read(cx).creating_temp_email {
                                            return;
                                        }

                                        let app_state = app_state.clone();
                                        app_state.update(cx, |state, cx| {
                                            state.creating_temp_email = true;
                                            state.temp_email_spinner_frame = 0;
                                            cx.notify();
                                        });

                                        let spinner_state = app_state.clone();
                                        this.spinner_task = Some(cx.spawn(async move |_this, cx2| {
                                            loop {
                                                cx2.background_executor()
                                                    .timer(Duration::from_millis(150))
                                                    .await;

                                                let still_loading = spinner_state.update(cx2, |state, cx| {
                                                    if !state.creating_temp_email {
                                                        return false;
                                                    }

                                                    state.temp_email_spinner_frame =
                                                        (state.temp_email_spinner_frame + 1) % 3;
                                                    cx.notify();
                                                    true
                                                });

                                                if !still_loading {
                                                    break;
                                                }
                                            }
                                        }));

                                        cx.spawn(async move |_this, cx2| {
                                            match create_account().await {
                                                Ok(email) => {
                                                    app_state.update(cx2, |state, cx| {
                                                        state.temp_email.push(email);
                                                        state.creating_temp_email = false;
                                                        cx.notify();
                                                    });
                                                }
                                                Err(error) => {
                                                    println!("Failed: {}", error);
                                                    app_state.update(cx2, |state, cx| {
                                                        state.creating_temp_email = false;
                                                        cx.notify();
                                                    });
                                                }
                                            }
                                            Ok::<(), anyhow::Error>(())
                                        })
                                        .detach();
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .ml(px(8.0))
                            .pl(px(14.0))
                            .border_l(px(1.0))
                            .border_color(rgb(Theme::color(&self.theme.sidebar_tree_border)))
                            .children(temporary_emails),
                    ),
            )
            .into_any_element()
    }
}
