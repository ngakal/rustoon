use dioxus::prelude::*;
use rustoon_shared::models::*;

use crate::components::storage;

#[derive(Clone, Copy)]
pub struct AppState {
    pub trending: Signal<Vec<AniMedia>>,
    pub manhwa: Signal<Vec<AniMedia>>,
    pub search_results: Signal<Vec<AniMedia>>,
    pub current_tab: Signal<Tab>,
    pub history: Signal<Vec<ReadingProgress>>,
    pub bookmarks: Signal<Vec<BookmarkItem>>,
    pub search_query: Signal<String>,
    pub loading: Signal<bool>,
    pub selected_media: Signal<Option<i32>>,
}

impl AppState {
    pub fn get_tab(&self) -> Tab { self.current_tab.read().clone() }
    pub fn is_loading(&self) -> bool { *self.loading.read() }
}

pub fn init_state() -> AppState {
    AppState {
        trending: Signal::new(vec![]),
        manhwa: Signal::new(vec![]),
        search_results: Signal::new(vec![]),
        current_tab: Signal::new(Tab::Home),
        history: Signal::new(storage::get_history()),
        bookmarks: Signal::new(storage::get_bookmarks()),
        search_query: Signal::new(String::new()),
        loading: Signal::new(true),
        selected_media: Signal::new(None),
    }
}
