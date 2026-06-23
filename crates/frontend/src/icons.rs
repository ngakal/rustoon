use dioxus::prelude::*;

// Lucide-style SVG icons — inline, zero dependencies

#[component]
pub fn IconHome(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" }
        path { d: "M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
    }}
}

#[component]
pub fn IconSearch(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        circle { cx: "11", cy: "11", r: "8" }
        path { d: "m21 21-4.3-4.3" }
    }}
}

#[component]
pub fn IconBookOpen(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "M12 7v14" }
        path { d: "M16 3h-2a4 4 0 0 0-4 4 4 4 0 0 0-4-4H4" }
        path { d: "M16 3v14a4 4 0 0 0-4-4 4 4 0 0 0-4 4V3" }
    }}
}

#[component]
pub fn IconSettings(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        circle { cx: "12", cy: "12", r: "3" }
        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
    }}
}

#[component]
pub fn IconStar(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "currentColor",
        stroke: "currentColor", stroke_width: "2",
        path { d: "M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.612 1.88l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.974 0L6.35 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.612-1.88L2.114 9.794a.53.53 0 0 1 .294-.904l5.166-.756a2.123 2.123 0 0 0 1.595-1.16z" }
    }}
}

#[component]
pub fn IconBookmark(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" }
    }}
}

#[component]
pub fn IconArrowLeft(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "m12 19-7-7 7-7" }
        path { d: "M19 12H5" }
    }}
}

#[component]
pub fn IconChevronRight(class: String) -> Element {
    rsx! { svg { class: "{class}", view_box: "0 0 24 24", fill: "none",
        stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
        path { d: "m9 18 6-6-6-6" }
    }}
}
