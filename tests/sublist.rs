use rust_exercism::enums::sublist::{sublist, Comparison};

#[test]
fn empty_equals_empty() {
    let v: &[u32] = &[];
    assert_eq!(Comparison::Equal, sublist(v, v));
}

#[test]
fn test_empty_is_a_sublist_of_anything() {
    let v1: &[char] = &[];
    let v2: &[char] = &['a', 's', 'd', 'f'];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn test_anything_is_a_superlist_of_empty() {
    let v1: &[char] = &['a', 's', 'd', 'f'];
    let v2: &[char] = &[];
    assert_eq!(Comparison::Superlist, sublist(v1, v2));
}

#[test]
fn test_1_is_not_2() {
    assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
}

#[test]
fn test_compare_larger_equal_lists() {
    use std::iter::repeat;
    let v: Vec<char> = repeat('x').take(1000).collect();
    assert_eq!(Comparison::Equal, sublist(&v, &v));
}

#[test]
fn test_sublist_at_start() {
    let v1: &[u32] = &[1, 2, 3];
    let v2: &[u32] = &[1, 2, 3, 4, 5];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn sublist_in_middle() {
    let v1: &[u32] = &[4, 3, 2];
    let v2: &[u32] = &[5, 4, 3, 2, 1];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn sublist_at_end() {
    let v1: &[u32] = &[3, 4, 5];
    let v2: &[u32] = &[1, 2, 3, 4, 5];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn partially_matching_sublist_at_start() {
    let v1: &[u32] = &[1, 1, 2];
    let v2: &[u32] = &[1, 1, 1, 2];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn sublist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();
    assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
}

#[test]
fn huge_sublist_not_in_huge_list() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();
    assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}

#[test]
fn superlist_at_start() {
    let v1: &[u32] = &[1, 2, 3, 4, 5];
    let v2: &[u32] = &[1, 2, 3];
    assert_eq!(Comparison::Superlist, sublist(v1, v2));
}

#[test]
fn superlist_in_middle() {
    let v1: &[u32] = &[5, 4, 3, 2, 1];
    let v2: &[u32] = &[4, 3, 2];
    assert_eq!(Comparison::Superlist, sublist(v1, v2));
}

#[test]
fn superlist_at_end() {
    let v1: &[u32] = &[1, 2, 3, 4, 5];
    let v2: &[u32] = &[3, 4, 5];
    assert_eq!(Comparison::Superlist, sublist(v1, v2));
}

#[test]
fn second_list_missing_element_from_first_list() {
    let v1: &[u32] = &[1, 2, 3];
    let v2: &[u32] = &[1, 3];
    assert_eq!(Comparison::Unequal, sublist(v1, v2));
}

#[test]
fn superlist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();
    assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
}

#[test]
fn recurring_values_sublist() {
    let v1: &[u32] = &[1, 2, 1, 2, 3];
    let v2: &[u32] = &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1];
    assert_eq!(Comparison::Sublist, sublist(v1, v2));
}

#[test]
fn recurring_values_unequal() {
    let v1: &[u32] = &[1, 2, 1, 2, 3];
    let v2: &[u32] = &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1];
    assert_eq!(Comparison::Unequal, sublist(v1, v2));
}
