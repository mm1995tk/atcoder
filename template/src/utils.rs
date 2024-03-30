fn usize_to_vec<T: std::str::FromStr>(n: usize) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut res = vec![];
    for c in n.to_string().as_str().chars() {
        res.push(
            c.to_string()
                .parse()
                .expect(format!("{c}を型Tに変換できない").as_str()),
        );
    }

    res
}

/// コールバック関数fがtrueとなる要素が何連続しているかを数える
fn count_sequence<T, Collection: FromIterator<usize>>(
    vector: &Vec<T>,
    f: impl Fn(&T) -> bool,
) -> Collection {
    let mut local = 0;
    let mut ans = vec![];

    for i in vector {
        let b = f(i);

        if !b {
            ans.push(local);
            local = 0;
            continue;
        }

        local += 1;
    }
    ans.push(local);

    ans.into_iter().collect()
}

fn count_by_item<T: Eq + Hash>(xs: Vec<T>) -> HashMap<T, usize> {
    let mut resp = HashMap::new();

    for x in xs {
        resp.entry(x)
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);
    }

    resp
}
