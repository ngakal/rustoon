use dioxus::prelude::*;

use crate::state::AppState;

#[component]
pub fn LibraryPage() -> Element {
    let mut state = use_context::<AppState>();
    let bookmarks = state.bookmarks.read().clone();
    let history = state.history.read().clone();

    rsx! {
        div {
            div { class: "section",
                div { class: "section-header", h2 { "Bookmarks" } }
                if bookmarks.is_empty() {
                    div { class: "empty-state",
                        h3 { "No bookmarks yet" }
                        p { "Tap a comic to see details and bookmark it" }
                    }
                } else {
                    div { class: "comic-grid",
                        for bm in bookmarks.iter() {
                            div { class: "comic-card",
                                img { class: "comic-card-img",
                                    src: "{bm.media_cover}", alt: "{bm.media_title}",
                                    loading: "lazy",
                                }
                                div { class: "comic-card-title", "{bm.media_title}" }
                            }
                        }
                    }
                }
            }

            div { class: "section",
                div { class: "section-header",
                    h2 { "Reading History" }
                    if !history.is_empty() {
                        button { class: "see-all",
                            onclick: move |_| {
                                crate::components::storage::clear_history();
                                state.history.set(vec![]);
                            },
                            "Clear"
                        }
                    }
                }
                if history.is_empty() {
                    div { class: "empty-state",
                        h3 { "No reading history" }
                    }
                } else {
                    for entry in history.iter() {
                        div { class: "history-item",
                            img { src: "{entry.media_cover}", alt: "{entry.media_title}" }
                            div { class: "history-info",
                                h3 { "{entry.media_title}" }
                                p { "Chapter {entry.chapter}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
