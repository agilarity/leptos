use super::*;

#[wasm_bindgen_test]
fn should_decrement_the_number_of_counters() {
    // Given
    view_counters();
    add_counter();
    add_counter();
    add_counter();

    // When
    remove_counter(2);

    // Then
    assert_eq!(total(), 0);
    assert_eq!(counters(), 2);
}
