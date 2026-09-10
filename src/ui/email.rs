use crate::app::AppState;
use crate::models::{Email, Theme};
use gpui::{Context, Entity, Render, Window, div, prelude::*, px, rgb};

pub struct EmailView {
	pub state: Entity<AppState>,
	pub email: Option<Email>,
	pub theme: Theme,
}

impl EmailView {
	pub fn new(state: Entity<AppState>, theme: Theme) -> Self {
		Self {
			state,
			email: None,
			theme,
		}
	}
}

impl Render for EmailView {
	fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
		let Some(email) = self.email.clone() else {
			return div().into_any_element();
		};
		let state = self.state.clone();

		div()
			.w_full()
			.h_full()
			.px(px(24.0))
			.py(px(20.0))
			.flex()
			.flex_col()
			.bg(rgb(Theme::color(&self.theme.inbox_background)))
			.child(
				div()
					.id("back-to-inbox")
					.cursor_pointer()
					.text_color(rgb(Theme::color(&self.theme.inbox_header_text)))
					.on_click(move |_event, _window, cx| {
						state.update(cx, |state, cx| {
							state.selected_message = None;
							cx.notify();
						});
					})
					.child("Back to inbox"),
			)
			.child(
				div()
					.text_size(px(22.0))
					.text_color(rgb(Theme::color(&self.theme.inbox_header_text)))
					.child(email.subject),
			)
			.child(
				div()
					.mt(px(12.0))
					.text_size(px(14.0))
					.text_color(rgb(Theme::color(&self.theme.inbox_text)))
					.child(format!("From: {}", email.from)),
			)
			.child(
				div()
					.mt(px(24.0))
					.text_size(px(15.0))
					.text_color(rgb(Theme::color(&self.theme.inbox_text)))
					.child(email.intro),
			)
			.into_any_element()
	}
}
