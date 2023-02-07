#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (x, y) if x > y => match compare(a, b) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        (x, y) if x < y => match compare(b, a) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        (_, _) => match a == b {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
    }
}

pub fn compare<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|v| v == b)
}
