use crate::app::AppState;
use crate::{models::{Email, get_mail},};
use gpui::{Context, Entity, Render, Window, div, prelude::*, px, rgb, svg};
use reqwest_eventsource::{Event, EventSource};
use futures_util::StreamExt;

pub struct Inbox {
    pub emails: Vec<Email>,
    pub loading: bool,
    pub state: Entity<AppState>,
}

impl Inbox {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Inbox {
        let inbox = Self {
            emails: Vec::new(),
            loading: false,
            state: state.clone()
        };

        cx.observe(&state, |this , state, cx| {
            let account = state.read(cx).temp_email.clone();

            if let Some(account) = account {
                this.start_mail_listener(account, cx);
            }
        })
        .detach();

        inbox
    }

    fn start_mail_listener(&mut self, account: crate::models::TempEmail, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();

        let state = self.state.clone();

        cx.spawn(async move |this, cx| {
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

            let url = format!("https://mercure.mail.tm/.well-known/mercure?topic=/accounts/{}", account.id);

            println!("Conntecting to Mercure..");
            println!("Topic: /accounts/{}", account.id);

            let client = reqwest::Client::new();

            let request = client.get(&url).header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", account.token)
            ).header(
                reqwest::header::ACCEPT,
                "text/event-stream"
            );

            let mut events = EventSource::new(request)?;

            while let Some(event) = events.next().await {
                match event {
                    Ok(Event::Open) => {
                        println!("Mercure connection opened");
                    }

                    Ok(Event::Message(message)) => {
                        println!("Mercure event received: {}", message.event);
                        println!("Mercure data: {}", message.data);

                        match get_mail(&account).await {
                            Ok(emails) => {
                                this.update(cx, |inbox, cx| {
                                    inbox.emails = emails;
                                    inbox.loading = false;
                                    cx.notify();
                                })?;
                            }

                            Err(error) => {
                                eprintln!("Failed to refresh mail after mercure event: {}", error);
                            }
                        }
                    }

                    Err(error) => {
                        eprintln!("Mercure connection error: {}", error);
                    }
                }
            }
            println!("Mercure connection closed");

            let _ = state;
            Ok::<(), anyhow::Error>(())
        })
        .detach();
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
                    )
                    .child(
                        svg()
                            .path(include_str!("../../assets/images/email.svg"))
                            .w(px(18.0))
                            .h(px(18.0)),
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
