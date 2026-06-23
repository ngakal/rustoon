use dioxus::prelude::*;

use crate::state::AppState;

#[component]
pub fn SettingsPage() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div {
            div { class: "section",
                h2 { "Settings" }
                p { class: "text-secondary", "Customize your RustToon experience" }
            }

            div { class: "settings-group",
                h3 { "Data" }
                div { class: "settings-item",
                    span { "Clear reading history" }
                    button { class: "btn-danger",
                        onclick: move |_| {
                            crate::components::storage::clear_history();
                            state.history.set(vec![]);
                        },
                        "Clear"
                    }
                }
                div { class: "settings-item",
                    span { "Clear bookmarks" }
                    button { class: "btn-danger",
                        onclick: move |_| {
                            crate::components::storage::clear_bookmarks();
                            state.bookmarks.set(vec![]);
                        },
                        "Clear"
                    }
                }
            }

            div { class: "settings-group",
                h3 { "About" }
                div { class: "settings-item",
                    span { "Version" }
                    span { class: "text-muted", "0.3.0" }
                }
                div { class: "settings-item",
                    span { "Stack" }
                    span { class: "text-muted", "Rust + Dioxus + WASM" }
                }
                div { class: "settings-item",
                    span { "Data source" }
                    span { class: "text-muted", "AniList API" }
                }
            }
        }
    }
}
