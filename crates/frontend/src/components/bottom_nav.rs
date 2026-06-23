use dioxus::prelude::*;
use rustoon_shared::models::Tab;

use crate::icons::*;
use crate::state::AppState;

#[component]
pub fn BottomNav() -> Element {
    let mut state = use_context::<AppState>();
    let tab = state.get_tab();

    rsx! {
        nav { class: "bottom-nav",
            NavBtn { label: "Home", active: tab == Tab::Home,
                onclick: move |_| state.current_tab.set(Tab::Home),
                IconHome { class: String::from("nav-svg") }
            }
            NavBtn { label: "Explore", active: tab == Tab::Explore,
                onclick: move |_| state.current_tab.set(Tab::Explore),
                IconSearch { class: String::from("nav-svg") }
            }
            NavBtn { label: "Library", active: tab == Tab::Library,
                onclick: move |_| state.current_tab.set(Tab::Library),
                IconBookOpen { class: String::from("nav-svg") }
            }
            NavBtn { label: "Settings", active: tab == Tab::Settings,
                onclick: move |_| state.current_tab.set(Tab::Settings),
                IconSettings { class: String::from("nav-svg") }
            }
        }
    }
}

#[component]
fn NavBtn(label: String, active: bool, onclick: EventHandler<()>, children: Element) -> Element {
    rsx! {
        button {
            class: if active { "nav-item active" } else { "nav-item" },
            onclick: move |_| onclick.call(()),
            div { class: "nav-icon", {children} }
            span { "{label}" }
        }
    }
}
