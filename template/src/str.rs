use std::collections::BTreeMap;

use itertools::Itertools;

fn run_length_encoding(str: &str) -> Vec<(char, usize)> {
    let mut ans = vec![];

    let chars = str.chars().collect_vec();
    if chars.len() < 1 {
        return ans;
    }

    ans.push((chars[0], 0usize));

    for c in chars {
        let x = ans.last_mut().unwrap();
        if c == x.0 {
            x.1 += 1;
        } else {
            ans.push((c, 1));
        }
    }

    ans
}

/// 回文判定
fn is_kaibun(x: &str) -> bool {
    let chars = x.chars().collect_vec();

    let mut b = true;
    for i in 0..chars.len() / 2 {
        b = b && chars[i] == chars[chars.len() - i - 1];
    }
    b
}

/// 空文字列を含まないsubstringの組み合わせ
fn substrings(s: &str) -> Vec<&str> {
    let mut substrs = vec![];

    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            substrs.push(&s[i..j]);
        }
    }

    substrs
}

/// 文字列中の英小文字の登場回数を数える
fn count_by_ascii_lowercase(s: &str) -> Vec<usize> {
    let mut alpha = vec![0; 26];

    for c in s.chars() {
        let i = c as usize - 97;
        alpha[i] += 1;
    }

    alpha
}
