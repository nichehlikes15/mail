mod app;
mod assets;
mod models;
mod ui;

use app::MailApp;
use assets::Assets;
use gpui_platform::application;

#[tokio::main]
async fn main() {
    application()
        .with_assets(Assets)
        .run(|cx| {
            MailApp::open(cx);
        });
}