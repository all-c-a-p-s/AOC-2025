use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT: &'static str = include_str!("./day08.in");

const VERTICES: usize = 1000;

#[derive(Clone, Copy, PartialEq, Eq)]
struct P {
    from: usize,
    to: usize,
    dist: i64,
}

impl Ord for P {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn get() -> Vec<(i64, i64, i64)> {
    INPUT
        .lines()
        .map(|x| {
            let mut it = x.split(",").map(|y| y.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn try_push(h: &mut BinaryHeap<P>, v: P) {
    if h.len() < VERTICES {
        h.push(v);
    } else if let Some(&max) = h.peek() {
        if v < max {
            h.pop();
            h.push(v);
        }
    }
}

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (b.0 - a.0) * (b.0 - a.0) + (b.1 - a.1) * (b.1 - a.1) + (b.2 - a.2) * (b.2 - a.2)
}

fn make_graph(h: &mut BinaryHeap<P>, n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; n];
    while let Some(p) = h.pop() {
        adj_list[p.from].push(p.to);
        adj_list[p.to].push(p.from);
    }

    adj_list
}

pub fn part_one() -> i64 {
    let ps = get();

    let mut h = BinaryHeap::new();

    let n = ps.len();
    for i in 1..n {
        for j in 0..i {
            let d = dist(ps[j], ps[i]);

            let p = P {
                from: j,
                to: i,
                dist: d,
            };

            try_push(&mut h, p);
        }
    }

    let g = make_graph(&mut h, n);

    let mut vis = vec![false; n];

    fn dfs(cur: usize, adj: &[Vec<usize>], vis: &mut Vec<bool>) -> i64 {
        if vis[cur] {
            return 0;
        }

        vis[cur] = true;

        let mut t = 1;
        for &v in &adj[cur] {
            t += dfs(v, adj, vis);
        }

        t
    }

    let (mut first, mut second, mut third) = (1, 1, 1);

    for u in 0..n {
        let d = dfs(u, &g, &mut vis);
        if d > first {
            third = second;
            second = first;
            first = d;
        } else if d > second {
            third = second;
            second = d;
        } else if d > third {
            third = d;
        }
    }

    first * second * third
}

struct DSU {
    link: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let link = (0..n).collect();
        let size = (0..n).map(|_| 1).collect();

        Self { link, size }
    }

    fn find(&self, mut i: usize) -> usize {
        while i != self.link[i] {
            i = self.link[i];
        }

        i
    }

    fn same(&self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    fn unite(&mut self, i: usize, j: usize) {
        let (mut a, mut b) = (self.find(i), self.find(j));
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.size[a] += self.size[b];
        self.link[b] = a;
    }
}

pub fn part_two() -> i64 {
    let ps = get();

    let mut h = BinaryHeap::new();

    let n = ps.len();
    for i in 1..n {
        for j in 0..i {
            let d = dist(ps[j], ps[i]);

            let p = P {
                from: j,
                to: i,
                dist: d,
            };

            h.push(Reverse(p));
        }
    }

    let mut d = DSU::new(n);

    let mut ans = 0;
    let mut cnt = 0;

    while let Some(p) = h.pop() {
        let (i, j) = (p.0.to, p.0.from);
        if d.same(i, j) {
            continue;
        }

        cnt += 1;

        ans = ps[i].0 * ps[j].0;
        d.unite(i, j);

        if cnt == VERTICES - 1 {
            break;
        }
    }

    ans
}
