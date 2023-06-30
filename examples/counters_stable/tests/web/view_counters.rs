use super::*;
use pretty_assertions::assert_eq;

#[wasm_bindgen_test]
fn should_see_the_initial_values() {
    // When
    view_counters();

    // Then
    assert_eq!(total(), 0);
    assert_eq!(counters(), 0);
}
