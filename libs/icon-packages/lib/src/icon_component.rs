use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> String;
    fn xmlns(&self) -> String;
    fn child_elements(&self) -> Element;
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape>(
    icon: T,
    width: u32,
    height: u32,
    fill: &'static str,
    //class: &'static str,
    //title: &'static str,
) -> Element {
    rsx! {
        svg {
            stroke: "currentColor",
            stroke_width: "0",
            class: format_args!("{}", ""),
            height: format_args!("{}", height),
            width: format_args!("{}", width),
            view_box: format_args!("{}", icon.view_box()),
            xmlns: format_args!("{}", icon.xmlns()),
            fill: format_args!("{}", fill),
            title { "" }
            {icon.child_elements()}
        }
    }
}
