#![recursion_limit="512"]
use wasm_bindgen::prelude::*;
use web_sys::*;
mod enums;
#[macro_use]
mod util;
mod bot_logic;
use bot_logic::*;
mod yew_app;
mod checkbox;
use yew_app::*;
use yew::prelude::App;

#[wasm_bindgen(start)]
pub async fn main() {
    log!("Hello World!");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = window().expect("No window");
    let document = window.document().expect("No document");

    let panel = document.create_element("div").unwrap();
    panel.set_attribute("id", "bot_panel").unwrap();

    let style = document.create_element("style").unwrap();
    style.set_attribute("scoped", "").unwrap();
    style.set_inner_html(include_str!("style.css"));

    let panel_container = document.get_elements_by_class_name("incentive-description").item(0).unwrap();
    panel_container.append_child(&style).unwrap();
    panel_container.append_child(&panel).unwrap();

    yew::initialize();
    App::<Model>::new().mount(panel);
}