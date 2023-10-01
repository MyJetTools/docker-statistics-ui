use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::*;

pub fn search_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "gray", icon: BsSearch } })
}

pub fn server_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 32, height: 32, icon: BsPc } })
}

pub fn server_icon_16(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, icon: BsPc } })
}

pub fn memory_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, icon: BsMemory } })
}

pub fn cpu_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, icon: BsCpu } })
}
