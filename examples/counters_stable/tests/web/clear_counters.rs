use super::*;

#[wasm_bindgen_test]
fn should_reset_the_counts() {
    // Given
    view_counters();
    add_counter();
    add_counter();
    add_counter();

    // When
    clear_counters();

    // Then
    assert_eq!(total(), 0);
    assert_eq!(counters(), 0);
}
