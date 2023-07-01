use super::*;

#[wasm_bindgen_test]
fn should_increase_the_number_of_counters() {
    // Given
    view_counters();

    // When
    add_1k_counters();
    add_1k_counters();
    add_1k_counters();

    // Then
    assert_eq!(total(), 0);
    assert_eq!(counters(), 3000);
}
