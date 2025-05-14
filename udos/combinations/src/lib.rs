#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let quantity = arr.len();
    if k > quantity {
        return vec![];
    }
    if k == 0 {
        return vec![vec![]];
    }
    let mut result = combinations(&arr[1..], k - 1);
    
    for comb in &mut result {
        comb.insert(0, arr[0]);
    }
    let second_part = combinations(&arr[1..], k);
    result.extend(second_part);
    return result;
}
