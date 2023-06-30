use counters_stable::Counters;
use leptos::*;
use wasm_bindgen_test::*;

// Test Suites
pub mod view_counters;

wasm_bindgen_test_configure!(run_in_browser);

// Actions

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
