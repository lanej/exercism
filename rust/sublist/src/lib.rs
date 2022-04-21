#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(first: &[T], second: &[T]) -> Comparison {
    if first == second {
        return Comparison::Equal;
    }

    if first.is_empty() {
        return Comparison::Sublist;
    }

    if second.is_empty() {
        return Comparison::Superlist;
    }

    if contains(first, second) {
        return Comparison::Sublist;
    }

    if contains(second, first) {
        return Comparison::Superlist;
    }

    return Comparison::Unequal;
}

fn contains<T: PartialEq + std::fmt::Debug>(first: &[T], second: &[T]) -> bool {
    for (i, e) in second.iter().enumerate() {
        if first.first().unwrap() != e {
            continue;
        }

        let sublist = &second.get(i..i + first.len());
        if sublist.is_none() {
            break;
        }

        if sublist.unwrap() == first {
            return true;
        }
    }

    false
}
