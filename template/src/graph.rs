use std::{cmp::Reverse, collections::BinaryHeap};
use crate::INF;

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

fn dijkstra_heap<'a>(g: &'a Graph, start: usize) -> Vec<usize> {
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
            }
        }
    }
    dist
}

fn dijkstra_heap_with_callback<'a, F: FnMut(usize, &'a Edge)>(
    g: &'a Graph,
    start: usize,
    // startからの最短距離のリストが更新されるたびに実行される
    mut on_change_dist: F,
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
                on_change_dist(v, e);
            }
        }
    }
    dist
}
/// Examples:
///
/// ```
/// dijkstra_heap_with_callback(&g, 0, |a, e| {
///    prev[e.to] = Some(a);
/// });
/// let p = restore_dijkstra_path(n - 1, &prev);
///
/// ```
fn restore_dijkstra_path(to: usize, prev: &Vec<Option<usize>>) -> Vec<usize> {
    let mut i = to;
    let mut p = vec![to];
    while let Some(&Some(v)) = prev.get(i) {
        p.push(v);
        i = v;
    }
    p.reverse();
    p
}
