use serde::{Deserialize, Serialize};

// ── AniList API Response Types ─────────────────────────
// AniList returns camelCase JSON. We use rename_all = "camelCase"
// except where the field is PascalCase (like "Page").

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AniListResponse {
    pub data: Option<AniListData>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AniListData {
    #[serde(rename = "Page")]
    pub page: Option<AniListPage>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AniListPage {
    pub page_info: Option<PageInfo>,
    pub media: Option<Vec<AniMedia>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub current_page: Option<i32>,
    pub has_next_page: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AniMedia {
    pub id: i32,
    pub title: Option<AniTitle>,
    pub description: Option<String>,
    pub cover_image: Option<AniCover>,
    pub banner_image: Option<String>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub chapters: Option<i32>,
    pub genres: Option<Vec<String>>,
    pub average_score: Option<i32>,
    pub site_url: Option<String>,
    pub country_of_origin: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AniTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AniCover {
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

impl AniMedia {
    pub fn display_title(&self) -> String {
        self.title.as_ref()
            .and_then(|t| t.english.clone().or(t.romaji.clone()))
            .unwrap_or_else(|| "Untitled".into())
    }

    pub fn cover_url(&self) -> String {
        self.cover_image.as_ref()
            .and_then(|c| c.extra_large.clone().or(c.large.clone()))
            .unwrap_or_else(|| "".into())
    }
}

// ── App State Types ───────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Tab { Home, Explore, Library, Settings }

impl Default for Tab {
    fn default() -> Self { Tab::Home }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub media_id: i32,
    pub media_title: String,
    pub media_cover: String,
    pub chapter: i32,
    pub scroll_pos: f64,
    pub last_read: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarkItem {
    pub media_id: i32,
    pub media_title: String,
    pub media_cover: String,
    pub added_at: i64,
}
