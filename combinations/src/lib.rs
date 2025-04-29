#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let n = arr.len();
    if n < k {
        return Vec::new();
    }
    if k == 0 {
        return vec![Vec::new()];
    }
    let mut res = Vec::new();
    for i in 0..n {
        let combs = combinations(&arr[i + 1..], k - 1);
        res.extend(combs.into_iter().map(|mut comb| {
            comb.insert(0, arr[i]);
            comb
        }));
    }
    res
}
