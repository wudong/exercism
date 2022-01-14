use std::cmp::Ordering::*;
use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let cmp = _first_list.len().cmp(&_second_list.len());

    match cmp {
        Equal => {
            if check_contain(_first_list, _second_list) && check_contain(_second_list, _first_list)
            {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Greater => {
            if check_contain(_first_list, _second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Less => {
            if check_contain(_second_list, _first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

pub fn check_contain<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _second_list.len() == 0 && _first_list.len() >= 0 {
        return true;
    }

    let mut iter1 = _first_list.iter().skip_while(|&x| *x != _second_list[0]);
    let mut iter2 = _second_list.iter();

    let c1 = loop {
        let (e1, e2) = (iter1.next(), iter2.next());
        let keep_going = match (e1, e2) {
            (Some(v1), Some(v2)) if *v1 == *v2 => true,
            _ => false,
        };
        if keep_going {
            continue;
        } else {
            break e2.is_none();
        }
    };

    let mut iter1 = _first_list
        .iter()
        .rev()
        .skip_while(|&x| *x != _second_list[_second_list.len() - 1]);
    let mut iter2 = _second_list.iter().rev();

    let c2 = loop {
        let (e1, e2) = (iter1.next(), iter2.next());
        let keep_going = match (e1, e2) {
            (Some(v1), Some(v2)) if *v1 == *v2 => true,
            _ => false,
        };
        if keep_going {
            continue;
        } else {
            break e2.is_none();
        }
    };

    return c1 || c2;
}
