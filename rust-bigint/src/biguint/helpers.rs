use crate::{BigUint, Digit};
use std::cmp::Ordering;
use std::mem::size_of;

const HEX_MASK: u128 = 0xF;
const HEX_BITS: usize = 4;

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

pub(crate) fn extract_hex_vec_from_u128(n: u128) -> Vec<u8> {
    let mut res = Vec::with_capacity(size_of::<u128>() / HEX_BITS);
    for i in 0..res.capacity() {
        res.push(((n & (HEX_MASK << (HEX_BITS * i))) >> (HEX_BITS * i)) as u8)
    }
    fit_u8_vec(&mut res);
    res
}

#[inline]
fn extract_hex_vec_from_u64(x: u64) -> Vec<u8>{
    vec![
        (x & 0x0000_0000_0000_000F) as u8,
        ((x & 0x0000_0000_0000_00F0) >> (HEX_BITS * 1)) as u8,
        ((x & 0x0000_0000_0000_0F00) >> (HEX_BITS * 2)) as u8,
        ((x & 0x0000_0000_0000_F000) >> (HEX_BITS * 3)) as u8,
        ((x & 0x0000_0000_000F_0000) >> (HEX_BITS * 4)) as u8,
        ((x & 0x0000_0000_00F0_0000) >> (HEX_BITS * 5)) as u8,
        ((x & 0x0000_0000_0F00_0000) >> (HEX_BITS * 6)) as u8,
        ((x & 0x0000_0000_F000_0000) >> (HEX_BITS * 7)) as u8,
        ((x & 0x0000_000F_0000_0000) >> (HEX_BITS * 8)) as u8,
        ((x & 0x0000_00F0_0000_0000) >> (HEX_BITS * 9)) as u8,
        ((x & 0x0000_0F00_0000_0000) >> (HEX_BITS * 10)) as u8,
        ((x & 0x0000_F000_0000_0000) >> (HEX_BITS * 11)) as u8,
        ((x & 0x000F_0000_0000_0000) >> (HEX_BITS * 12)) as u8,
        ((x & 0x00F0_0000_0000_0000) >> (HEX_BITS * 13)) as u8,
        ((x & 0x0F00_0000_0000_0000) >> (HEX_BITS * 14)) as u8,
        ((x & 0xF000_0000_0000_0000) >> (HEX_BITS * 15)) as u8,
    ]
}

pub(crate) fn extract_hex_vec_from_biguint(n: &BigUint) -> Vec<u8> {
    let mut res = Vec::with_capacity((size_of::<u64>() / HEX_BITS) * n.data.len());
    for x in n.data.iter() {
        res.extend_from_slice(&extract_hex_vec_from_u64(*x))
    }
    fit_u8_vec(&mut res);
    res
}