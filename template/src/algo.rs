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