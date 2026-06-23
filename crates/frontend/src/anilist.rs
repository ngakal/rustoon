use rustoon_shared::models::*;
use wasm_bindgen::prelude::*;

const ANILIST_URL: &str = "https://graphql.anilist.co";

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
    // Try the JS helper first (works in web browser with the injected script)
    if let Some(result) = fetch_via_js_helper(query, vars).await {
        return result;
    }

    // Fallback: use web_sys::Request directly (works in Android WebView)
    if let Some(result) = fetch_via_request_api(query, vars).await {
        return result;
    }

    vec![]
}

/// Method 1: Use the window.__anilist_fetch JS helper function.
/// This works when the HTML <script> tag is loaded (web browser).
async fn fetch_via_js_helper(query: &str, vars: &str) -> Option<Vec<AniMedia>> {
    let window = web_sys::window()?;
    let key = JsValue::from_str("__anilist_fetch");
    let helper_val = js_sys::Reflect::get(&window.into(), &key).ok()?;

    if helper_val.is_undefined() || helper_val.is_null() {
        return None;
    }

    let helper = helper_val.dyn_ref::<js_sys::Function>()?;

    let result = helper.call2(
        &JsValue::NULL,
        &JsValue::from_str(query),
        &JsValue::from_str(vars),
    ).ok()?;

    let promise = js_sys::Promise::from(result);
    let text_val = wasm_bindgen_futures::JsFuture::from(promise).await.ok()?;
    let text = text_val.as_string()?;

    if text.is_empty() { return None; }

    Some(parse_anilist_response(&text))
}

/// Method 2: Use web_sys::Request API directly.
/// This works in Android WebView where the JS helper may not be injected.
async fn fetch_via_request_api(query: &str, vars: &str) -> Option<Vec<AniMedia>> {
    let window = web_sys::window()?;

    let body = format!(r#"{{"query":{},"variables":{}}}"#,
        serde_json::to_string(query).ok()?,
        vars
    );

    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");

    let headers = web_sys::Headers::new().ok()?;
    headers.set("Content-Type", "application/json").ok()?;
    headers.set("Accept", "application/json").ok()?;
    opts.set_headers(&headers.into());

    opts.set_body(&JsValue::from_str(&body));

    let request = web_sys::Request::new_with_str_and_init(ANILIST_URL, &opts).ok()?;
    let promise = window.fetch_with_request(&request);

    let response_val = wasm_bindgen_futures::JsFuture::from(promise).await.ok()?;
    let response: web_sys::Response = response_val.dyn_into().ok()?;

    let text_promise = response.text().ok()?;
    let text_val = wasm_bindgen_futures::JsFuture::from(text_promise).await.ok()?;
    let text = text_val.as_string()?;

    if text.is_empty() { return None; }

    Some(parse_anilist_response(&text))
}

fn parse_anilist_response(text: &str) -> Vec<AniMedia> {
    serde_json::from_str::<AniListResponse>(text)
        .ok()
        .and_then(|r| r.data)
        .and_then(|d| d.page)
        .and_then(|p| p.media)
        .unwrap_or_default()
}
