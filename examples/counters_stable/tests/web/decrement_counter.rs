use super::*;

#[wasm_bindgen_test]
fn should_decrease_the_total_count() {
    // Given
    view_counters();
    add_counter();

    // When
    decrement_counter(1);
    decrement_counter(1);
    decrement_counter(1);

    // Then
    assert_eq!(total(), -3);
    assert_eq!(counters(), 1);
}
