/// 繰り返し二乗法によるa^nをpで割ったあまりを求める
fn mod_pow(a: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return a % p;
    }

    if n % 2 == 1 {
        (a % p * mod_pow(a, n - 1, p)) % p
    } else {
        let t = mod_pow(a, n / 2, p) % p;
        (t * t) % p
    }
}

/// 体がmod(p)の時のa/bの結果を求める
fn div_mod(a: usize, b: usize, p: usize) -> usize {
    a * mod_pow(b, p - 2, p)
}

/// nの約数をタプルで返すイテレータ
struct DivisorIterator {
    n: usize,
    current: usize,

    /// ルートn
    max: usize,
}

impl DivisorIterator {
    fn new(n: usize) -> Self {
        Self {
            current: 1,
            max: (n as f64).sqrt() as usize,
            n,
        }
    }
}

impl Iterator for DivisorIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let result = self.current;
        self.current += 1;

        if self.n % result == 0 {
            Some((result, self.n / result))
        } else {
            self.next()
        }
    }
}
