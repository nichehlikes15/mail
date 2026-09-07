use gpui::{Entity, Window, div, prelude::*, px, rgb, svg};

use crate::models::{TempEmail, create_account};
pub struct Sidebar {
    pub state: Entity<crate::app::AppState>,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, root_cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.state.clone();
        div()
            .w(px(360.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(0x111111))
            .border_l(px(1.0))
            .border_color(rgb(0x2a2a2a))
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
                            .text_color(rgb(0xffffff))
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
                            .child(
                                svg()
                                    .path(include_str!("../assets/images/email.svg"))
                                    .w(px(18.0))
                                    .h(px(18.0)),
                            )
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
                            .border_color(rgb(0x3a3a3a))
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xaaaaaa))
                                    .child("oliver@gmail.com"),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xaaaaaa))
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
                            .text_color(rgb(0xffffff))
                            .child("Temp Emails"),
                    )
                    // Tree
                    .child(
                        div()
                            .ml(px(8.0))
                            .pl(px(14.0))
                            .border_l(px(1.0))
                            .border_color(rgb(0x3a3a3a))
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xaaaaaa))
                                    .bg(rgb(0x181818))
                                    .child("metropolitanemelyne@web-library.net"),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xaaaaaa))
                                    .child("email2@mail.tm"),
                            ),
                    ),
            )
            .child(
                div()
                    .id("generate-email")
                    .p(px(10.0))
                    .bg(rgb(0x222222))
                    .cursor_pointer()
                    .on_click(root_cx.listener(move |_this, _event, _window, cx| {
                        println!("Generate temporary email clicked!");
                        let app_state = app_state.clone();
                        cx.spawn(async move |_this, cx2| {
                            match create_account().await {
                                Ok(email) => {
                                    // cx2.update(|cx| {
                                    //     cx.set_global::<crate::app::TempEmailToken>(
                                    //         TempEmailToken(std::sync::Arc::new(email.clone())),
                                    //     );
                                    //     cx.notify(entity_id);
                                    // });
                                    app_state.update(cx2, |state, thecx| {
                                        state.temp_email = Some(TempEmail {
                                            address: email.address,
                                            password: email.password,
                                        });
                                        thecx.notify();
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
