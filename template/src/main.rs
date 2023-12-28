use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    vec,
};

fn main() {
    input! {
        n:usize,
        edges: [(usize, usize);n]
    }
}

fn print_yes_or_no(&b: &bool) {
    println!("{}", if b { "Yes" } else { "No" });
}

fn run_length_encoding(str: &str) -> Vec<(char, usize)> {
    let chars = str.chars().collect_vec();

    let mut vec = vec![(chars[0], 0usize)];

    for c in chars {
        let x = vec.last_mut().unwrap();
        if c == x.0 {
            x.1 += 1;
        } else {
            vec.push((c, 1));
        }
    }
    vec
}

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

fn dijkstra_heap<'a, F: FnMut(usize, &'a Edge)>(
    g: &'a Graph,
    start: usize,
    mut callback: Option<F>,
) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut heap = BinaryHeap::with_capacity(n);

    dist[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for e in &g[v] {
            let tmp = dist[v] + (e.cost as usize);
            if dist[e.to] > tmp {
                dist[e.to] = tmp;
                heap.push((Reverse(dist[e.to]), e.to));
                if let Some(ref mut f) = callback {
                    f(v, e);
                }
            }
        }
    }
    dist
}

fn dijkstra_heap2<F: Fn(usize, &Edge, &Vec<usize>) -> T, T>(
    g: &Graph,
    start: usize,
    callback: F,
) -> (Vec<usize>, Vec<T>) {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut heap = BinaryHeap::with_capacity(n);

    dist[start] = 0;
    heap.push((Reverse(0), start));

    let mut result: Vec<T> = vec![];

    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for e in &g[v] {
            let tmp = dist[v] + (e.cost as usize);
            if dist[e.to] > tmp {
                dist[e.to] = tmp;
                heap.push((Reverse(dist[e.to]), e.to));
                result.push(callback(v, e, &dist));
            }
        }
    }
    (dist, result)
}

fn mk_graph(n: usize, edges: &Vec<(usize, usize)>, is_directed: bool) -> Graph {
    let mut g: Graph = vec![vec![]; n];
    for id in 0..edges.len() {
        let &(a, b) = &edges[id];
        g[a].push(Edge { to: b, cost: 1, id });
        if !is_directed {
            g[b].push(Edge { to: a, cost: 1, id });
        }
    }

    g
}

fn mk_costed_graph(n: usize, edges: &Vec<(usize, usize, isize)>, is_directed: bool) -> Graph {
    let mut g: Graph = vec![vec![]; n];
    for id in 0..edges.len() {
        let &(a, b, cost) = &edges[id];
        g[a].push(Edge { to: b, cost, id });
        if !is_directed {
            g[b].push(Edge { to: a, cost, id });
        }
    }
    g
}

/// 生えている辺の集合を持つ形式のグラフ
type Graph = Vec<Vec<Edge>>;

#[derive(Debug, Clone, Copy)]
/// 辺
struct Edge {
    /// 辺の識別子
    id: usize,

    /// 向いている頂点
    to: usize,

    /// 重み
    cost: isize,
}

// 素数判定
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

/// bit全探索のイテレータ
struct BitPatternIterator<'a, T> {
    vec: &'a Vec<T>,
    current: usize,
    max: usize,
}

impl<'a, T> BitPatternIterator<'a, T> {
    fn new(vec: &'a Vec<T>) -> Self {
        let n = vec.len();
        Self {
            vec,
            current: 0,
            max: 1 << n,
        }
    }
}

impl<'a, T> Iterator for BitPatternIterator<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }

        let mut tmp: Vec<&'a T> = vec![];
        for i in 0..self.max {
            if self.current & (1 << i) != 0 {
                tmp.push(&self.vec[i]);
            }
        }

        self.current += 1;

        Some(tmp)
    }
}

#[derive(Debug, Clone)]
struct GridZipper {
    focus: (isize, isize),
    history: Vec<(isize, isize)>,
}

impl GridZipper {
    fn go_to_left(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 -= n as isize;
        self
    }

    fn go_to_upper_left(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 -= n as isize;
        self.focus.0 -= n as isize;

        self
    }

    fn go_to_lower_left(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 -= n as isize;
        self.focus.0 += n as isize;

        self
    }

    fn go_to_upper_right(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 += n as isize;
        self.focus.0 -= n as isize;

        self
    }

    fn go_to_lower_right(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 += n as isize;
        self.focus.0 += n as isize;

        self
    }

    fn go_to_right(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.1 += n as isize;
        self
    }

    fn go_up(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.0 -= n as isize;
        self
    }

    fn go_down(&mut self, n: usize) -> &mut Self {
        self.history.push(self.focus);
        self.focus.0 += n as isize;
        self
    }

    fn go_back(&mut self) -> &mut Self {
        let pre = self.history.pop();
        match pre {
            None => self,
            Some(v) => {
                self.focus = v;
                self
            }
        }
    }
}

const INF: usize = usize::max_value();
