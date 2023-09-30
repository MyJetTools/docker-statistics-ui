use dioxus::prelude::*;

use crate::{states::*, views::*};

pub fn right_panel(cx: Scope) -> Element {
    match &*main_state.read() {
        MainState::Nothing => {
            render!(div {})
        }
    }
}
