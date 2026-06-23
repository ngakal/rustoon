use dioxus::prelude::*;
use rustoon_shared::models::*;

use crate::anilist;
use crate::components::bottom_nav::BottomNav;
use crate::icons::*;
use crate::pages::detail::DetailPage;
use crate::pages::home::HomePage;
use crate::pages::explore::ExplorePage;
use crate::pages::library::LibraryPage;
use crate::pages::settings::SettingsPage;
use crate::state::*;

#[component]
pub fn App() -> Element {
    let mut state = use_context_provider(|| init_state());
    let tab = state.get_tab();
    let mut loaded = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());

    if !loaded() {
        loaded.set(true);
        spawn(async move {
            state.loading.set(true);
            error_msg.set(String::new());

            let trending = anilist::fetch_popular(1).await;
            let manhwa = anilist::fetch_manhwa(1).await;

            state.trending.set(trending.clone());
            state.manhwa.set(manhwa.clone());
            state.loading.set(false);

            // If both are empty, we likely have a network issue
            if trending.is_empty() && manhwa.is_empty() {
                error_msg.set("Could not load manga. Check your internet connection and try again.".into());
            }
        });
    }

    let has_sel = state.selected_media.read().is_some();
    let err = error_msg.read().clone();

    rsx! {
        div {
            if has_sel {
                DetailPage {}
            } else {
                div { class: "top-bar",
                    h1 { "RustToon" }
                    div { class: "top-bar-actions",
                        button { class: "icon-btn",
                            onclick: move |_| state.current_tab.set(Tab::Explore),
                            IconSearch { class: String::from("icon-svg") }
                        }
                    }
                }

                if state.is_loading() {
                    div { class: "loading-screen",
                        div { class: "spinner" }
                        p { "Loading manga..." }
                    }
                } else if !err.is_empty() {
                    div { class: "error-screen",
                        IconSearch { class: String::from("error-icon") }
                        h3 { "Connection Error" }
                        p { "{err}" }
                        button { class: "btn btn-primary",
                            onclick: move |_| {
                                loaded.set(false);
                            },
                            "Retry"
                        }
                    }
                } else {
                    match tab {
                        Tab::Home => rsx! { HomePage {} },
                        Tab::Explore => rsx! { ExplorePage {} },
                        Tab::Library => rsx! { LibraryPage {} },
                        Tab::Settings => rsx! { SettingsPage {} },
                    }
                }

                BottomNav {}
            }
        }
    }
}
