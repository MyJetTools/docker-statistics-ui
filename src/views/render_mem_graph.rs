use dioxus::prelude::*;

use crate::format_mem;

const HEIGHT: usize = 70;

use crate::app_ctx::METRICS_HISTORY_SIZE;

#[inline_props]
pub fn render_mem_graph(cx: Scope, values: Vec<i64>) -> Element {
    let max_scale = get_max_scale(&values);

    let max_scale_text = format_mem(max_scale as i64);

    let mut x = METRICS_HISTORY_SIZE - values.len();

    let height_f64 = HEIGHT as f64;

    let mut items = Vec::new();
    for v in values {
        let v = *v as f64;
        let y = v / max_scale;

        let y = height_f64 - y * (height_f64 as f64);
        items.push(rsx! {
            line {
                x1: "{x}",
                x2: "{x}",
                y1: "{y}",
                y2: "{HEIGHT}",
                style: "stroke:rgb(0,0,255);stroke-width:1"
            }
        });
        x += 1;
    }

    render! {
        svg {
            width: "{METRICS_HISTORY_SIZE}",
            height: "{HEIGHT}",
            view_box: "0 0 {METRICS_HISTORY_SIZE} {HEIGHT}",
            rect {
                width: "{METRICS_HISTORY_SIZE}",
                height: "{HEIGHT}",
                style: "fill:none; stroke-width:1;stroke:rgb(0,0,0)"
            }

            items.into_iter(),

            text { x: "1", y: "11", fill: "white", style: "font-size:10px", "{max_scale_text}" }
            text { x: "0", y: "10", fill: "black", style: "font-size:10px", max_scale_text }
        }
    }
}

fn get_max_scale(values: &[i64]) -> f64 {
    let max = *values.iter().max().unwrap_or(&1);

    let max = max as f64;
    max
}
