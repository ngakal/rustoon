use dioxus::prelude::*;
use rustoon_shared::models::Tab;

use crate::components::comic_card::ComicCard;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let mut state = use_context::<AppState>();
    let trending = state.trending.read().clone();
    let manhwa = state.manhwa.read().clone();
    let history = state.history.read().clone();

    rsx! {
        div {
            // Trending carousel
            div { class: "section",
                div { class: "section-header",
                    h2 { "Trending Manga" }
                    button { class: "see-all",
                        onclick: move |_| state.current_tab.set(Tab::Explore),
                        "See All"
                    }
                }
            }
            div { class: "hscroll",
                for comic in trending.iter().take(15) {
                    { rsx! { ComicCard { key: "{comic.id}", comic: comic.clone() } } }
                }
            }

            // Continue Reading
            if !history.is_empty() {
                div { class: "section",
                    div { class: "section-header",
                        h2 { "Continue Reading" }
                    }
                    for entry in history.iter().take(5) {
                        div { class: "history-item",
                            img { src: "{entry.media_cover}", alt: "{entry.media_title}" }
                            div { class: "history-info",
                                h3 { "{entry.media_title}" }
                                p { "Ch. {entry.chapter}" }
                            }
                        }
                    }
                }
            }

            // Korean Manhwa
            div { class: "section",
                div { class: "section-header",
                    h2 { "Korean Manhwa" }
                }
            }
            div { class: "hscroll",
                for comic in manhwa.iter().take(15) {
                    { rsx! { ComicCard { key: "kr-{comic.id}", comic: comic.clone() } } }
                }
            }

            // All popular grid
            div { class: "section",
                div { class: "section-header",
                    h2 { "Popular" }
                }
                div { class: "comic-grid",
                    for comic in trending.iter().take(6) {
                        { rsx! { ComicCard { key: "pop-{comic.id}", comic: comic.clone() } } }
                    }
                }
            }
        }
    }
}
