mod anilist;
mod app;
mod components;
mod icons;
mod pages;
mod state;

use dioxus::prelude::*;

fn main() {
    // Install panic hook so errors are visible in WebView console
    // instead of silently showing a white screen
    console_error_panic_hook::set_once();

    launch(app::App);
}
