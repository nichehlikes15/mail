use gpui::{Context, Window, div, prelude::*, px, rgb};
use crate::models::Theme;

pub struct MailTopBar {
    pub theme: Theme,
}

impl Render for MailTopBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(35.0))
            .flex()
            .items_center()
            .bg(rgb(Theme::color(&self.theme.mailtopbar_background)))

            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_active_text)))
                    .text_size(px(15.0))
                    .bg(rgb(Theme::color(&self.theme.mailtopbar_active_background)))
                    .mb(px(-1.0))
                    .pb(px(1.0))
                    .child("inbox")
                    .border_r(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )

            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_inactive_text)))
                    .text_size(px(15.0))
                    .child("starred")
                    .border_b(px(1.0))
                    .border_r(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )

            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_inactive_text)))
                    .text_size(px(15.0))
                    .child("drafts")
                    .border_b(px(1.0))
                    .border_r(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )

            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_inactive_text)))
                    .text_size(px(15.0))
                    .child("sent")
                    .border_b(px(1.0))
                    .border_r(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )

            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_inactive_text)))
                    .text_size(px(15.0))
                    .child("trash")
                    .border_b(px(1.0))
                    .border_r(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )
            
            .child(
                div()
                    .h_full()
                    .px(px(15.0))
                    .flex()
                    .items_center()
                    .text_size(px(20.0))
                    .text_color(rgb(Theme::color(&self.theme.mailtopbar_inactive_text)))
                    .child("+")
                    .border_b(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .border_b(px(1.0))
                    .border_color(rgb(Theme::color(&self.theme.mailtopbar_border))),
            )
            .into_any_element()
    }
}
