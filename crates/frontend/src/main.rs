mod anilist;
mod app;
mod components;
mod icons;
mod pages;
mod state;

use dioxus::prelude::*;

fn main() {
    launch(app::App);
}
