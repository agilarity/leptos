use counters_stable::Counters;
use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

// Test Suites
pub mod add_1k_counters;
pub mod add_counter;
pub mod clear_counters;
pub mod view_counters;

wasm_bindgen_test_configure!(run_in_browser);

// Actions

pub fn add_1k_counters() {
    find_by_text("Add 1000 Counters").click();
}

pub fn add_counter() {
    find_by_text("Add Counter").click();
}

pub fn clear_counters() {
    find_by_text("Clear Counters").click();
}

pub fn view_counters() {
    remove_existing_counters();
    mount_to_body(|cx| view! { cx,  <Counters/> });
}

fn remove_existing_counters() {
    if let Some(counter) =
        leptos::document().query_selector("body div").unwrap()
    {
        counter.remove();
    }
}

// Queries

pub fn total() -> i32 {
    data_test_id("total").parse::<i32>().unwrap()
}

pub fn counters() -> i32 {
    data_test_id("counters").parse::<i32>().unwrap()
}

fn data_test_id(id: &str) -> String {
    let selector = format!("[data-testid=\"{}\"]", id);
    leptos::document()
        .query_selector(&selector)
        .unwrap()
        .expect("counters not found")
        .text_content()
        .unwrap()
}

pub fn find_by_text(text: &str) -> HtmlElement {
    let xpath = format!("//*[text()='{}']", text);
    let document = leptos::document();
    document
        .evaluate(&xpath, &document)
        .unwrap()
        .iterate_next()
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
}
