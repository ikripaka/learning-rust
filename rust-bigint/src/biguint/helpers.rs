use crate::{BigUint, Digit};
use std::cmp::Ordering;

pub(crate) fn fit(x: &mut BigUint) {
    let mut cleanup_index = None;
    'index_search: for (i, x) in x.data.iter().enumerate().rev() {
        if *x == 0 {
            let _ = cleanup_index.insert(i);
        } else {
            break 'index_search;
        }
    }
    if let Some(index) = cleanup_index {
        for i in (index..x.data.len()).rev() {
            if x.data.len() == 1 {
                break;
            }
            let _ = x.data.remove(i);
        }
    }
}

pub(crate) fn fit_u8_vec(x: &mut Vec<u8>) {
    let mut cleanup_index = None;
    'index_search: for (i, x) in x.iter().enumerate().rev() {
        if *x == 0 {
            let _ = cleanup_index.insert(i);
        } else {
            break 'index_search;
        }
    }
    if let Some(index) = cleanup_index {
        for i in (index..x.len()).rev() {
            if x.len() == 1 {
                break;
            }
            let _ = x.remove(i);
        }
    }
}

/// **compare_slices** -- compares Digit slices by comparing _y_ slice with _x_ directly
/// _x_ -- bigger one
/// _y_ -- has to have length that is less or equal to _x_ one
fn compare_slices(x: &[Digit], y: &[Digit]) -> Ordering {
    if x.len() > y.len() {
        return Ordering::Greater;
    } else if x.len() < y.len() {
        return Ordering::Less;
    }

    let mut i = y.len() as i64 - 1;
    while match x.get(i as usize) {
        None => 0,
        Some(res) => *res,
    } == y[i as usize]
    {
        i -= 1
    }

    if i == -1 {
        Ordering::Equal
    } else {
        match y.get(i as usize) {
            None => Ordering::Greater,
            Some(d) => {
                if x[i as usize] > *d {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

/// **partial_cmp** -- compares both BigUint values
pub(crate) fn partial_cmp(x: &BigUint, y: &BigUint) -> Ordering {
    if x != y {
        if x.data.len() < y.data.len() {
            match compare_slices(&y.data, &x.data) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        } else {
            compare_slices(&x.data, &y.data)
        }
    } else {
        Ordering::Equal
    }
}
