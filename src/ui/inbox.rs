use crate::app::AppState;
use crate::{
    app::MailApp,
    models::{Email, TempEmail, get_mail},
};
use gpui::{Context, Entity, Render, Window, div, prelude::*, px, rgb};

pub struct Inbox {
    pub emails: Vec<Email>,
    pub loading: bool,
    pub state: Entity<AppState>,
}

impl Inbox {
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.loading = true;

        cx.notify();

        let acc = self.state.read(cx).temp_email.clone();

        cx.spawn(async move |this, cx| {
            if let Some(account) = acc {
                match get_mail(&account).await {
                    Ok(emails) => {
                        this.update(cx, |inbox, cx| {
                            inbox.emails = emails;
                            inbox.loading = false;

                            cx.notify();
                        })?;
                    }

                    Err(error) => {
                        eprintln!("Failed to retrieve mail: {}", error);

                        this.update(cx, |inbox, cx| {
                            inbox.loading = false;

                            cx.notify();
                        })?;
                    }
                }
            } else {
                this.update(cx, |inbox, cx| {
                    inbox.loading = false;

                    cx.notify();
                })?;
            }

            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }
}

impl Inbox {
    pub fn new(state: Entity<crate::app::AppState>, cx: &mut Context<Self>) -> Inbox {
        cx.observe(&state, |this, state, cx| {
            if state.read(cx).temp_email.is_some() {
                this.refresh(cx);
            }
        })
        .detach();

        Self {
            emails: Vec::new(),

            loading: false,

            state,
        }
    }
}

impl Render for Inbox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(0x111111))
            .flex()
            .flex_col()
            // Header
            .child(
                div()
                    .w_full()
                    .h(px(64.0))
                    .px(px(24.0))
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(rgb(0x252525))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(rgb(0xffffff))
                            .child("Inbox"),
                    ),
            )
            // Email list
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .children(self.emails.iter().map(|email| {
                        div()
                            .w_full()
                            .h(px(64.0))
                            .px(px(24.0))
                            .flex()
                            .items_center()
                            .border_b_1()
                            .border_color(rgb(0x222222))
                            // Sender
                            .child(
                                div()
                                    .w(px(400.0))
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xdce0e5))
                                    .child(email.from.clone()),
                            )
                            // Subject
                            .child(
                                div()
                                    .ml_auto()
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xdce0e5))
                                    .child(email.subject.clone()),
                            )
                            // Date
                            /*.child(
                                div()
                                    .w(px(100.0))
                                    .text_color(rgb(0x777777))
                                    .child(email.created_at.clone())
                            )*/
                            .into_any_element()
                    })),
            )
    }
}
