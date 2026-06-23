use dioxus::prelude::*;
use rustoon_shared::models::{AniMedia, BookmarkItem, ReadingProgress};

use crate::icons::*;
use crate::state::AppState;

#[component]
pub fn DetailPage() -> Element {
    let mut state = use_context::<AppState>();
    let sel = state.selected_media.read().clone();

    let media = sel.and_then(|id| {
        let t = state.trending.read();
        let m = state.manhwa.read();
        let s = state.search_results.read();
        t.iter().chain(m.iter()).chain(s.iter())
            .find(|c| c.id == id).cloned()
    });

    match media {
        None => rsx! {
            div { class: "empty-state",
                h3 { "Comic not found" }
                button { class: "btn btn-primary",
                    onclick: move |_| state.selected_media.set(None),
                    "Go Back"
                }
            }
        },
        Some(comic) => rsx! { DetailInner { comic } },
    }
}

#[component]
fn DetailInner(comic: AniMedia) -> Element {
    let mut state = use_context::<AppState>();
    let title = comic.display_title();
    let cover = comic.cover_url();
    let banner = comic.banner_image.clone().unwrap_or_default();
    let desc = comic.description.clone().unwrap_or_default();
    let ch_count = comic.chapters.unwrap_or(0);
    let score = comic.average_score.unwrap_or(0);
    let mid = comic.id;
    let is_bm = state.bookmarks.read().iter().any(|b| b.media_id == mid);

    rsx! {
        div { class: "detail-page",
            // Hero
            div { class: "detail-hero",
                img { src: "{banner}", alt: "" }
                div { class: "detail-hero-overlay",
                    img { class: "detail-hero-cover", src: "{cover}", alt: "{title}" }
                    div { class: "detail-hero-info",
                        h1 { "{title}" }
                        if let Some(fmt) = &comic.format {
                            p { class: "author", "{fmt}" }
                        }
                        div { class: "meta-row",
                            if score > 0 { span { "★ {score}" } }
                            if ch_count > 0 { span { "{ch_count} ch" } }
                        }
                    }
                }
            }

            // Actions
            div { class: "detail-actions",
                { let t = title.clone(); let c = cover.clone();
                  rsx! {
                    button { class: "btn btn-primary",
                        onclick: move |_| {
                            save_reading(&mut state, mid, &t, &c, 1);
                        },
                        IconBookOpen { class: String::from("btn-icon") }
                        " Start Reading"
                    }
                }}
                { let t = title.clone(); let c = cover.clone();
                  rsx! {
                    button { class: if is_bm { "btn-bookmark active" } else { "btn-bookmark" },
                        onclick: move |_| {
                            toggle_bookmark(&mut state, mid, &t, &c);
                        },
                        IconBookmark { class: String::from("btn-icon") }
                    }
                }}
            }

            // Synopsis
            div { class: "detail-synopsis",
                if let Some(genres) = &comic.genres {
                    div { class: "genres",
                        for g in genres.iter() {
                            span { class: "genre-tag", "{g}" }
                        }
                    }
                }
                p { "{desc}" }
            }

            // Chapters
            div { class: "chapter-list",
                h3 { "Chapters" }
                for i in 1..=ch_count.min(30) {
                    { let t = title.clone(); let c = cover.clone();
                      rsx! {
                        div { class: "chapter-item",
                            onclick: move |_| {
                                save_reading(&mut state, mid, &t, &c, i);
                            },
                            h3 { "Chapter {i}" }
                            IconChevronRight { class: String::from("ch-arrow") }
                        }
                    }}
                }
            }

            // Back
            div { class: "detail-actions",
                button { class: "btn btn-secondary",
                    onclick: move |_| state.selected_media.set(None),
                    IconArrowLeft { class: String::from("btn-icon") }
                    " Back"
                }
            }
        }
    }
}

fn save_reading(state: &mut AppState, mid: i32, title: &str, cover: &str, ch: i32) {
    let p = ReadingProgress {
        media_id: mid, media_title: title.into(), media_cover: cover.into(),
        chapter: ch, scroll_pos: 0.0, last_read: js_sys::Date::now() as i64,
    };
    let mut h = state.history.read().clone();
    h.insert(0, p);
    crate::components::storage::save_history(&h);
    state.history.set(h);
}

fn toggle_bookmark(state: &mut AppState, mid: i32, title: &str, cover: &str) {
    let mut bms = state.bookmarks.read().clone();
    if bms.iter().any(|b| b.media_id == mid) {
        bms.retain(|b| b.media_id != mid);
    } else {
        bms.insert(0, BookmarkItem {
            media_id: mid, media_title: title.into(),
            media_cover: cover.into(), added_at: js_sys::Date::now() as i64,
        });
    }
    crate::components::storage::save_bookmarks(&bms);
    state.bookmarks.set(bms);
}
