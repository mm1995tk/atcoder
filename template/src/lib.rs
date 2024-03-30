mod algo;
mod axis;
mod bit;
mod graph;
mod grid;
mod int;
mod str;

use itertools::Itertools;
use num::Integer;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};
// lower_bound(&x) x以上のiterator => xより小さい個数が何個あるかわかる
// upper_bound(&x) xより大きいiterator => x以下の個数が何個あるかわかる
use superslice::Ext;

const INF: usize = usize::max_value();

fn main() {
    input! {
        n:usize,
    }
}

fn print_yes_or_no(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}
