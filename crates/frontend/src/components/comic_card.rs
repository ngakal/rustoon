use dioxus::prelude::*;
use rustoon_shared::models::AniMedia;

use crate::icons::*;
use crate::state::AppState;

#[component]
pub fn ComicCard(comic: AniMedia) -> Element {
    let mut state = use_context::<AppState>();
    let title = comic.display_title();
    let cover = comic.cover_url();
    let score = comic.average_score.unwrap_or(0);
    let mid = comic.id;

    rsx! {
        div { class: "comic-card",
            onclick: move |_| state.selected_media.set(Some(mid)),
            div { class: "comic-card-img-wrap",
                img { class: "comic-card-img", src: "{cover}", alt: "{title}",
                    loading: "lazy",
                }
                if score > 0 {
                    div { class: "comic-card-score",
                        IconStar { class: String::from("score-star") }
                        " {score}"
                    }
                }
            }
            div { class: "comic-card-title", "{title}" }
            if let Some(fmt) = &comic.format {
                div { class: "comic-card-meta", "{fmt}" }
            }
        }
    }
}
