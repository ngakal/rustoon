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

    if !loaded() {
        loaded.set(true);
        spawn(async move {
            state.loading.set(true);
            state.trending.set(anilist::fetch_popular(1).await);
            state.manhwa.set(anilist::fetch_manhwa(1).await);
            state.loading.set(false);
        });
    }

    let has_sel = state.selected_media.read().is_some();

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
