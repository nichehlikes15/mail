use gpui::{Entity, Window, div, prelude::*, px, rgb, svg};

use crate::models::{Theme, create_account};
pub struct Sidebar {
    pub state: Entity<crate::app::AppState>,
    pub theme: Theme,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, root_cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.state.clone();
        let temporary_emails = self.state.read(root_cx).temp_email.iter().map(|email| {
                div()
                    .px(px(8.0))
                    .py(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgb(self.theme.sidebar_text))
                    .bg(rgb(self.theme.sidebar_selected_background))
                    .child(email.address.clone())
            }).collect::<Vec<_>>();

        div()
            .w(px(360.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(self.theme.sidebar_background))
            .border_l(px(1.0))
            .border_color(rgb(self.theme.sidebar_border))
            /* Header
            .child(
                div()
                    .w_full()
                    .h(px(55.0))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .border_b(px(1.0))
                    .border_color(rgb(0x2a2a2a))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .text_color(rgb(self.theme.sidebar_header_text))
                            .child("Mail"),
                    ),
            ) */
            // Mail section
            .child(
                div()
                    .w_full()
                    .px(px(14.0))
                    .py(px(12.0))
                    // Mail title
                    .child(
                        div()
                            .h(px(30.0))
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            /*.child(
                                svg()
                                    .path(include_str!("../../assets/images/test.svg"))
                                    .w(px(18.0))
                                    .h(px(18.0)),
                            )*/
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xffffff))
                                    .child("Mail"),
                            ),
                    )
                    // Tree
                    .child(
                        div()
                            .ml(px(8.0))
                            .pl(px(14.0))
                            .border_l(px(1.0))
                            .border_color(rgb(self.theme.sidebar_tree_border))
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(self.theme.sidebar_text))
                                    .child("oliver@gmail.com"),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(self.theme.sidebar_text))
                                    .child("rem@googlemail.com"),
                            ),
                    ),
            )
            // Temp Emails section
            .child(
                div()
                    .w_full()
                    .px(px(14.0))
                    .py(px(12.0))
                    // Temp Emails title
                    .child(
                        div()
                            .h(px(30.0))
                            .flex()
                            .items_center()
                            .text_size(px(14.0))
                            .text_color(rgb(self.theme.sidebar_header_text))
                            .child("Temp Emails"),
                    )

                    // Temp Emails tree
                    .child(
                        div()
                            .ml(px(8.0))
                            .pl(px(14.0))
                            .border_l(px(1.0))
                            .border_color(rgb(self.theme.sidebar_tree_border))
                            .children(temporary_emails),
                    ),
            )
            .child(
                div()
                    .id("generate-email")
                    .p(px(10.0))
                    .bg(rgb(self.theme.sidebar_button_background))
                    .cursor_pointer()
                    .on_click(root_cx.listener(move |_this, _event, _window, cx| {
                        println!("Generate temporary email clicked!");

                        let app_state = app_state.clone();
                        cx.spawn(async move |_this, cx2| {
                            match create_account().await {
                                Ok(email) => {
                                    app_state.update(cx2, |state, cx| {
                                        state.temp_email.push(email);
                                        state.current_temp_email = Some(state.temp_email.len() - 1);
                                        cx.notify();
                                    });
                                }
                                Err(error) => {
                                    println!("Failed: {}", error);
                                }
                            }

                            Ok::<(), anyhow::Error>(())
                        })
                        .detach();
                    }))
                    .child("generate temporary email"),
            )
            .into_any_element()
    }
}
