/// 素数判定
fn is_prime_number(n: usize) -> bool {
    match n {
        n if n < 4 => n >= 2,
        _ => {
            for i in 2..=(n as f64).sqrt() as usize {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

///  素因数分解
fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];

    for p in 2..=((n as f64).sqrt() as usize) {
        if n % p != 0 {
            continue;
        }

        let mut e = 0;
        while n % p == 0 {
            e += 1;
            n /= p;
        }

        res.push((p, e));
    }

    if n != 1 {
        res.push((n, 1));
    }

    res
}

/// エラトステネスのふるい
fn eratosthenes(&n: &usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }

        for q in (p * 2..=n).step_by(p) {
            is_prime[q] = false;
        }
    }

    is_prime
}

/// メビウス関数
fn mobius(n: usize) -> isize {
    if n == 1 {
        return 1;
    }
    let vec = prime_factorize(n);

    let mut k = 0usize;

    let mut bool = false;

    for &(_, i) in &vec {
        bool |= i >= 2;
        k += i;
    }

    if bool {
        return 0;
    }

    if k % 2 == 0 {
        1
    } else {
        -1
    }
}
