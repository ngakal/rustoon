use rustoon_shared::models::{BookmarkItem, ReadingProgress};
use serde::de::DeserializeOwned;
use serde::Serialize;

const KEY_HISTORY: &str = "rustoon_history";
const KEY_BOOKMARKS: &str = "rustoon_bookmarks";

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn save<T: Serialize + ?Sized>(key: &str, value: &T) {
    if let Some(s) = local_storage() {
        if let Ok(json) = serde_json::to_string(value) {
            let _ = s.set_item(key, &json);
        }
    }
}

fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let s = local_storage()?;
    let json = s.get_item(key).ok()??;
    serde_json::from_str(&json).ok()
}

pub fn get_history() -> Vec<ReadingProgress> {
    load::<Vec<ReadingProgress>>(KEY_HISTORY).unwrap_or_default()
}

pub fn save_history(h: &[ReadingProgress]) { save(KEY_HISTORY, h) }

pub fn clear_history() { save_history(&[]) }

pub fn get_bookmarks() -> Vec<BookmarkItem> {
    load::<Vec<BookmarkItem>>(KEY_BOOKMARKS).unwrap_or_default()
}

pub fn save_bookmarks(b: &[BookmarkItem]) { save(KEY_BOOKMARKS, b) }

pub fn clear_bookmarks() { save_bookmarks(&[]) }
