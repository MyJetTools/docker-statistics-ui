use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::*;

pub fn search_icon() -> Element {
    Icon(BsSearch, 16, 16, "gray")
}

pub fn server_icon() -> Element {
    Icon(BsPc, 32, 32, "")
}

pub fn server_icon_16() -> Element {
    Icon(BsPc, 16, 16, "")
}

pub fn memory_icon() -> Element {
    Icon(BsMemory, 16, 16, "")
}

pub fn cpu_icon() -> Element {
    Icon(BsCpu, 16, 16, "")
}
