use dioxus::prelude::*;
use rustoon_shared::models::AniMedia;

use crate::anilist;
use crate::components::comic_card::ComicCard;
use crate::state::AppState;

#[component]
pub fn ExplorePage() -> Element {
    let mut state = use_context::<AppState>();
    let query = state.search_query.read().clone();
    let results = state.search_results.read().clone();
    let trending = state.trending.read().clone();

    let display: &Vec<AniMedia> = if results.is_empty() { &trending } else { &results };

    let genres: Vec<String> = trending.iter()
        .filter_map(|m| m.genres.clone())
        .flatten()
        .collect::<std::collections::HashSet<_>>()
        .into_iter().take(10).collect();

    rsx! {
        div {
            div { class: "search-bar",
                input {
                    r#type: "text",
                    placeholder: "Search manga, manhwa...",
                    value: "{query}",
                    oninput: move |e| {
                        let val = e.value();
                        state.search_query.set(val.clone());
                        if val.len() >= 2 {
                            let q = val.clone();
                            spawn(async move {
                                let res = anilist::search_manga(&q).await;
                                state.search_results.set(res);
                            });
                        } else if val.is_empty() {
                            state.search_results.set(vec![]);
                        }
                    },
                }
            }

            if genres.iter().any(|g| !g.is_empty()) {
                div { class: "genre-row",
                    for genre in genres.iter() {
                        { let g = genre.clone();
                          rsx! {
                            span { class: "genre-tag",
                                onclick: move |_| {
                                    state.search_query.set(g.clone());
                                    let q2 = g.clone();
                                    spawn(async move {
                                        let res = anilist::search_manga(&q2).await;
                                        state.search_results.set(res);
                                    });
                                },
                                "{genre}"
                            }
                          }
                        }
                    }
                }
            }

            div { class: "comic-grid",
                for comic in display.iter() {
                    { rsx! { ComicCard { key: "{comic.id}", comic: comic.clone() } } }
                }
            }
        }
    }
}
