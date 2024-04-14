#![recursion_limit = "1024"]

// use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, HtmlInputElement, InputEvent};

#[wasm_bindgen]
pub fn set_event_listener() {
    let oninput = Closure::wrap(Box::new(move |event: InputEvent| {
        let Some(target) = event.target() else { return };
        let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
            return;
        };
        if !input.has_attribute("data-events-input") {
            return;
        }
        refresh();
    }) as Box<dyn FnMut(_)>);
    document()
        .body()
        .unwrap()
        .add_event_listener_with_callback("input", oninput.as_ref().unchecked_ref())
        .unwrap();
    oninput.forget();
}

fn refresh() {
    let name = input_el().value();
    let greeting = if name.is_empty() {
        "".into()
    } else {
        format!("Hello, {}!", input_el().value())
    };
    output_el().set_inner_text(&greeting)
}

fn output_el() -> HtmlElement {
    query_selector("[data-events-output]")
}

fn input_el() -> HtmlInputElement {
    query_selector("[data-events-input]")
}

fn document() -> Document {
    window().and_then(|w| w.document()).expect("no document")
}

fn query_selector<T: wasm_bindgen::JsCast>(query: &str) -> T {
    document()
        .query_selector(query)
        .unwrap()
        .expect(&format!("no element '{query}'"))
        .dyn_into::<T>()
        .unwrap()
}
