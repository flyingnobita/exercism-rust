// use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut sublist = false; // first list is sublist of second list
    let mut superlist = false; // first list is superlist of second list

    fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
    where
        for<'a> &'a [T]: PartialEq,
    {
        if haystack.is_empty() {
            if needle.is_empty() {
                Some(0)
            } else {
                None
            }
        } else {
            // haystack is not empty
            if needle.is_empty() {
                Some(0)
            } else {
                haystack
                    .windows(needle.len())
                    .position(|window| window == needle)
            }
        }
    }

    if find_subsequence(_second_list, _first_list).is_some() {
        sublist = true;
    }
    if find_subsequence(_first_list, _second_list).is_some() {
        superlist = true;
    }

    if sublist && superlist {
        Comparison::Equal
    } else if !sublist && !superlist {
        Comparison::Unequal
    } else if sublist && !superlist {
        Comparison::Sublist
    } else {
        Comparison::Superlist
    }
}
