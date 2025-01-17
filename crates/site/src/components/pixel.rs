use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
    html! {
        (DOCTYPE)
        meta charset="UTF-8";
        title { "Pixel Art" }
        link rel="stylesheet" href="/s/autopixel/style.css" {};
    }
}

pub fn pixel_art_view(hash: u64) -> Markup {
    let png_path = format!("/pixel/sketches/{:x}.png", hash);
    let js_path = format!("/pixel/sketches/{:x}.js", hash);
    let png_name = format!("{:x}.png", hash);
    let js_name = format!("{:x}.js", hash);

    html! {
        (header())
        a href="/pixel" {
            h2 { "Back" }
        }
        a href=(png_path) download=(png_name) {
            "Download Image"
        }
        br {}
        a href=(js_path) download=(js_name) {
            "Download Javascript"
        }
        br {}
        script src="/s/autopixel/p5.min.js" {};
        script src=(js_path) {};
    }
}
