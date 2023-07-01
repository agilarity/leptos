use super::*;

#[wasm_bindgen_test]
fn should_increase_the_total_count() {
    // Given
    view_counters();
    add_counter();

    // When
    increment_counter(1);
    increment_counter(1);
    increment_counter(1);

    // Then
    assert_eq!(total(), 3);
    assert_eq!(counters(), 1);
}
