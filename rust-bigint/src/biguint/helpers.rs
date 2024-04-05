use std::cmp::Ordering;
use crate::{BigUint, Digit};

pub(crate) fn fit(x: &mut BigUint){
    let mut cleanup_index = None;
    'index_search :for (i, x) in x.data.iter().enumerate().rev(){
        if *x == 0 {
            let _ = cleanup_index.insert(i);
        }else{
            break 'index_search
        }
    }
    if let Some(index) = cleanup_index{
        for i in (index..x.data.len()).rev(){
            let _ = x.data.remove(i);
        }
    }
}

pub (crate) fn compare_slices(x: &[Digit], y: &[Digit]) -> Ordering{

    Ordering::Greater
}

pub(crate) fn partial_cmp(x: &BigUint, y: &BigUint) -> Option<Ordering>{
    if x != y{
        Some(if x.data.len() > y.data.len(){
            compare_slices(&x.data, &y.data)
        }else{
            match compare_slices(&y.data, &x.data) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        })
    }else{
        Some(Ordering::Equal)
    }
}