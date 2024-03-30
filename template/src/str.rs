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