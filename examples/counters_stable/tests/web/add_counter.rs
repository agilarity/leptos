use super::*;

#[wasm_bindgen_test]
fn should_increase_the_number_of_counters() {
    // Given
    view_counters();

    // When
    add_counter();
    add_counter();
    add_counter();

    // Then
    assert_eq!(total(), 0);
    assert_eq!(counters(), 3);
}
