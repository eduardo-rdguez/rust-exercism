use rust_exercism::functions::lasagna;

#[test]
fn expected_minutes_in_oven() {
    assert_eq!(40, lasagna::expected_minutes_in_oven())
}

#[test]
fn ramaining_minutes_in_oven() {
    assert_eq!(15, lasagna::ramaining_minutes_in_oven(25))
}

#[test]
fn preparation_time_in_minutes_for_one_layer() {
    assert_eq!(2, lasagna::preparation_time_in_minutes(1))
}

#[test]
fn preparation_time_in_minutes_for_multiple_layers() {
    assert_eq!(4, lasagna::preparation_time_in_minutes(2))
}

#[test]
fn elapsed_time_in_minutes_for_one_layer() {
    assert_eq!(10, lasagna::elapsed_time_in_minutes(1, 8))
}

#[test]
fn elapsed_time_in_minutes_for_multiple_layers() {
    assert_eq!(16, lasagna::elapsed_time_in_minutes(4, 8))
}
