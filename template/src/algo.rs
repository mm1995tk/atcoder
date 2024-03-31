use crate::INF;

/// 答えで二分探索
fn bi_search_by_answer(f: impl Fn(usize) -> bool) -> (usize, usize) {
    let mut l = 0usize;
    let mut r = INF;
    while r - l > 1 {
        let mid = (l + r) / 2;

        if f(mid) {
            r = mid;
        } else {
            l = mid;
        }
    }
    (l, r)
}

/// 累積和を作成
fn mk_prefix_sum<T: Copy + Clone + Default + std::ops::Add<Output = T>>(xs: &Vec<T>) -> Vec<T> {
    let n = xs.len();
    let mut res = vec![T::default(); n + 1];

    for i in 0..n {
        res[i + 1] = res[i] + xs[i];
    }

    res
}
