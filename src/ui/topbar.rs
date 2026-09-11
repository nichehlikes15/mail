use gpui::{Context, Window, div, prelude::*, px, rgb, svg};
use crate::models::Theme;

pub struct TopBar {
    pub theme: Theme,
}

impl Render for TopBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(35.0))
            .flex()
            .items_center()
            .bg(rgb(Theme::color(&self.theme.topbar_background)))
            //.border_b(px(1.0))
            //.border_color(rgb(0x2a2a2a))
            // Active Selection
            // Account 2
            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(Theme::color(&self.theme.topbar_inactive_text)))
                    .text_size(px(15.0))
                    .child("MailBox")
            )
            .child(div().flex_1())
            .child(
                div()
                    .h_full()
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .child(
                        svg()
                            .data(include_bytes!("../../assets/images/settings.svg"))
                            .text_color(rgb(Theme::color(&self.theme.topbar_inactive_text)))
                            .w(px(18.0))
                            .h(px(18.0)),
                    ),
            )
            .into_any_element()
    }
}
