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
            let _ = x.data.remove(i);
        }
    }
}

/// **compare_slices** -- compares Digit slices by comparing _y_ slice with _x_ directly
/// _x_ -- bigger one
/// _y_ -- has to have length that is less or equal to _x_ one
pub(crate) fn compare_slices(x: &[Digit], y: &[Digit]) -> Ordering {
    let mut i = 0;
    for d in y.iter() {
        if x[i] == *d {
            i += 1
        }
    }
    if i == x.len() {
        Ordering::Equal
    } else {
        match y.get(i) {
            None => Ordering::Greater,
            Some(d) => {
                if x[i] > *d {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

/// **partial_cmp** -- compares both BigUint values
pub(crate) fn partial_cmp(x: &BigUint, y: &BigUint) -> Option<Ordering> {
    if x != y {
        Some(if x.data.len() > y.data.len() {
            compare_slices(&x.data, &y.data)
        } else {
            match compare_slices(&y.data, &x.data) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        })
    } else {
        Some(Ordering::Equal)
    }
}
