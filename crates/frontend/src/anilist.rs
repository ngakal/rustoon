use rustoon_shared::models::*;
use wasm_bindgen::prelude::*;

pub async fn fetch_popular(page: i32) -> Vec<AniMedia> {
    let query = r#"query($page:Int,$perPage:Int){Page(page:$page,perPage:$perPage){media(type:MANGA,sort:POPULARITY_DESC){id title{romaji english native}description(asHtml:false)coverImage{extraLarge large medium color}bannerImage format status chapters volumes genres averageScore siteUrl countryOfOrigin}}}"#;
    let vars = format!(r#"{{"page":{page},"perPage":20}}"#);
    fetch_query(query, &vars).await
}

pub async fn fetch_manhwa(page: i32) -> Vec<AniMedia> {
    let query = r#"query($page:Int,$perPage:Int){Page(page:$page,perPage:$perPage){media(type:MANGA,countryOfOrigin:"KR",sort:POPULARITY_DESC){id title{romaji english native}description(asHtml:false)coverImage{extraLarge large medium color}bannerImage format status chapters volumes genres averageScore siteUrl countryOfOrigin}}}"#;
    let vars = format!(r#"{{"page":{page},"perPage":20}}"#);
    fetch_query(query, &vars).await
}

pub async fn search_manga(q: &str) -> Vec<AniMedia> {
    let query = r#"query($page:Int,$perPage:Int,$search:String){Page(page:$page,perPage:$perPage){media(type:MANGA,sort:POPULARITY_DESC,search:$search){id title{romaji english native}description(asHtml:false)coverImage{extraLarge large medium color}bannerImage format status chapters volumes genres averageScore siteUrl countryOfOrigin}}}"#;
    let escaped = q.replace('\\', r#"\\"#).replace('"', r#"\""#);
    let vars = format!(r#"{{"page":1,"perPage":20,"search":"{escaped}"}}"#);
    fetch_query(query, &vars).await
}

async fn fetch_query(query: &str, vars: &str) -> Vec<AniMedia> {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return vec![],
    };

    // Get the JS helper function via Reflect
    let key = JsValue::from_str("__anilist_fetch");
    let helper_val = match js_sys::Reflect::get(&window.into(), &key) {
        Ok(v) if !v.is_undefined() && !v.is_null() => v,
        _ => return vec![],
    };

    // Cast to Function using dyn_ref on JsValue
    let helper = match helper_val.dyn_ref::<js_sys::Function>() {
        Some(f) => f,
        None => return vec![],
    };

    // Call: window.__anilist_fetch(query, vars)
    let result = match helper.call2(
        &JsValue::NULL,
        &JsValue::from_str(query),
        &JsValue::from_str(vars),
    ) {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    // Await the Promise
    let promise = js_sys::Promise::from(result);
    let text_val = match wasm_bindgen_futures::JsFuture::from(promise).await {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let text = match text_val.as_string() {
        Some(t) => t,
        None => return vec![],
    };

    if text.is_empty() { return vec![]; }

    serde_json::from_str::<AniListResponse>(&text)
        .ok()
        .and_then(|r| r.data)
        .and_then(|d| d.page)
        .and_then(|p| p.media)
        .unwrap_or_default()
}
