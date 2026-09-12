use gpui::{Context, Window, WindowControlArea, div, prelude::*, px, rgb, svg};
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
            
            .child(
                div()
                    .h_full()
                    .px(px(18.0))
                    .flex()
                    .items_center()

                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                    .text_size(px(15.0))
                    .child("MailBox")
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .h(px(26.0))
                            .px(px(8.0))
                            .rounded(px(8.0))
                            .flex()
                            .items_center()

                            .hover(|this| {
                                this.bg(rgb(0x363c46))
                            })

                            .child(
                                svg()
                                    .data(include_bytes!("../../assets/images/settings.svg"))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                                    .w(px(15.0))
                                    .h(px(15.0)),
                            ),
                    )
                    .child(
                        div()
                            .id("minimize-button")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()

                            .hover(|this| {
                                this.bg(rgb(0x303030))
                            })

                            .window_control_area(WindowControlArea::Min)
                            .child(
                                svg()
                                    .data(include_bytes!("../../assets/images/minimize.svg"))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                                    .w(px(18.0))
                                    .h(px(18.0)),
                            ),
                    )
                    .child(
                        div()
                            .id("maximize-button")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            
                            .hover(|this| {
                                this.bg(rgb(0x303030))
                            })
                            
                            .window_control_area(WindowControlArea::Max)
                            .child(if _window.is_maximized() {
                                svg()
                                    .data(include_bytes!("../../assets/images/restore.svg"))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                                    .w(px(18.0))
                                    .h(px(18.0))
                            } else {
                                svg()
                                    .data(include_bytes!("../../assets/images/maximize.svg"))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                                    .w(px(18.0))
                                    .h(px(18.0))
                            }),
                    )
                    .child(
                        div()
                            .id("close-button")
                            .w(px(46.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()

                            .hover(|this| {
                                this.bg(rgb(0xc42b1c))
                                    .text_color(rgb(0xffffff))
                            })

                            .window_control_area(WindowControlArea::Close)
                            .child(
                                svg()
                                    .data(include_bytes!("../../assets/images/close.svg"))
                                    .text_color(rgb(Theme::color(&self.theme.topbar_active_text)))
                                    .w(px(18.0))
                                    .h(px(18.0)),
                            ),
                    ),
            )
            .into_any_element()
    }
}
