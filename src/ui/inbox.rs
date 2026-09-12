use crate::app::AppState;
use crate::models::{Email, Theme, get_mail};
use crate::ui::EmailView;
use gpui::{Context, Entity, Render, Task, Window, div, prelude::*, px, rgb};
use reqwest_eventsource::{Event, EventSource};
use futures_util::StreamExt;

pub struct Inbox {
    pub emails: Vec<Email>,
    pub loading: bool,
    pub state: Entity<AppState>,
    pub theme: Theme,
    pub email_view: Entity<EmailView>,
    mail_task: Option<Task<Result<(), anyhow::Error>>>,
    active_account_id: Option<String>,
}

impl Inbox {
    pub fn new(state: Entity<AppState>, email_view: Entity<EmailView>, theme: Theme, cx: &mut Context<Self>) -> Inbox {
        let inbox = Self {
            emails: Vec::new(),
            loading: false,
            state: state.clone(),
            theme,
            email_view,
            mail_task: None,
            active_account_id: None,
        };

        cx.observe(&state, |this , state, cx| {
            let account = {
                let state = state.read(cx);

                state.selected_email.and_then(|index| state.temp_email.get(index)).cloned()
            };

            if let Some(account) = account {
                if this.active_account_id.as_deref() != Some(account.id.as_str()) {
                    this.start_mail_listener(account, cx);
                }
            } else {
                this.mail_task = None;
                this.active_account_id = None;
                this.emails.clear();
                this.email_view.update(cx, |email_view, _cx| {
                    email_view.email = None;
                });
                state.update(cx, |state, _cx| {
                    state.selected_message = None;
                });
                this.loading = false;
                cx.notify();
            }
        })
        .detach();

        inbox
    }

    fn start_mail_listener(&mut self, account: crate::models::TempEmail, cx: &mut Context<Self>) {
        self.mail_task = None;
        self.active_account_id = Some(account.id.clone());
        self.emails.clear();
        self.email_view.update(cx, |email_view, _cx| {
            email_view.email = None;
        });
        self.state.update(cx, |state, _cx| {
            state.selected_message = None;
        });
        self.loading = true;
        cx.notify();

        let task = cx.spawn(async move |this, cx| {
            match get_mail(&account).await {
                Ok(emails) => {
                    this.update(cx, |inbox, cx| {
                        inbox.merge_emails(emails);
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
                                    inbox.merge_emails(emails);
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

            Ok::<(), anyhow::Error>(())
        });

        self.mail_task = Some(task);
    }

    fn merge_emails(&mut self, emails: Vec<Email>) {
        for email in emails {
            if let Some(existing) = self.emails.iter_mut().find(|existing| existing.id == email.id) {
                *existing = email;
            } else {
                self.emails.push(email);
            }
        }

        self.emails
            .sort_by(|left, right| right.created_at.cmp(&left.created_at));
    }
}

impl Render for Inbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(Theme::color(&self.theme.inbox_background)))
            .flex()
            .flex_col()
  
            /*.child(
                div()
                    .w_full()
                    .h(px(64.0))
                    .px(px(24.0))
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(rgb(Theme::color(&self.theme.inbox_header_border)))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(rgb(Theme::color(&self.theme.inbox_header_text)))
                            .child(format!(
                                "Inbox - {}",
                                self.state.read(cx).selected_email.and_then(|index| {
                                        self.state.read(cx).temp_email.get(index).map(|email| email.address.as_str())
                                    })
                                    .unwrap_or("")
                            )),
                    )
            )*/

            .child(
                div()
                    .w_full()
                    .flex_1()
                    .flex()
                    .flex_col()

                    .when(self.state.read(cx).selected_email.is_none(), |this| {
                        this
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(0x777777))
                                .child("Select an inbox"),
                        )
                    })


                    .when(
                        self.state.read(cx).selected_email.is_some() && self.emails.is_empty(),|this| {
                            this
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(0x777777))
                                    .child("No messages"),
                            )
                        },
                    )

                    .when(self.state.read(cx).selected_email.is_some() && !self.emails.is_empty(),|this| {
                        this.children(self.emails.iter().map(|email| {
                            div()
                                .w_full()
                                .h(px(64.0))
                                .px(px(24.0))
                                .flex()
                                .items_center()
                                .border_b_1()
                                .border_color(rgb(Theme::color(&self.theme.inbox_border)))
                                .id(format!("email-{}", email.id))
                                .cursor_pointer()
                                
                                .on_click({
                                    let state = self.state.clone();
                                    let email_view = self.email_view.clone();
                                    let email = email.clone();
                                    
                                    move |_event, _window, cx| {
                                        email_view.update(cx, |email_view, _cx| {
                                            email_view.email = Some(email.clone());
                                        });
                                        state.update(cx, |state, cx| {
                                            state.selected_message = Some(email.clone());
                                            cx.notify();
                                        });
                                    }
                                })

                                // Sender
                                .child(
                                    div()
                                        .w(px(400.0))
                                        .text_size(px(14.0))
                                        .text_color(rgb(Theme::color(&self.theme.inbox_text)))
                                        .child(email.from.clone()),
                                )

                                // Subject
                                .child(
                                    div()
                                        .ml_auto()
                                        .text_size(px(14.0))
                                        .text_color(rgb(Theme::color(&self.theme.inbox_text)))
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
                        }))
                    }),
            )
    }
}
