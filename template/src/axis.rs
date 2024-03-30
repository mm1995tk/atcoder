/// ユークリッド距離の2乗
fn euclidean_distance<
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Clone,
>(
    a: (T, T),
    b: (T, T),
) -> T {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    x.clone() * x + y.clone() * y
}
