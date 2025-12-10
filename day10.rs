use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &'static str = include_str!("./day10.in");

fn get() -> impl Iterator<Item = (i64, Vec<i64>, Vec<i64>)> {
    let parse = |x: String, m: &mut i64, ops: &mut Vec<i64>, j: &mut Vec<i64>| {
        let is_bracket = |x: &char| matches!(x, '(' | '[' | '{' | ')' | ']' | '}');
        let inner = x.chars().filter(|x| !is_bracket(x)).collect::<Vec<_>>();

        let nums = |x: String| {
            x.split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        };

        match x.chars().next().unwrap() {
            '(' => {
                let mut u = 0;
                for x in nums(inner.iter().collect()) {
                    u |= 1 << x;
                }
                ops.push(u);
            }
            '[' => {
                let mut u = 0;
                for i in 0..inner.len() {
                    if inner[i] == '#' {
                        u |= 1 << i
                    }
                }
                *m = u;
            }
            '{' => *j = nums(inner.iter().collect()),
            _ => unreachable!(),
        };
    };
    INPUT.lines().map(move |x| {
        let mut mask = 0;
        let mut ops = vec![];
        let mut js = vec![];

        for p in x.split_whitespace() {
            parse(p.to_string(), &mut mask, &mut ops, &mut js);
        }

        (mask, ops, js)
    })
}

fn bfs(m: i64, ops: &[i64]) -> i64 {
    let mut cnt = 0;
    let mut s = vec![m];

    if m == 0 {
        return 0;
    }

    loop {
        cnt += 1;
        let mut seen = HashSet::new();
        let mut ns = vec![];
        for &u in &s {
            for o in ops {
                if u ^ o == 0 {
                    return cnt;
                } else if !seen.contains(&(u ^ o)) {
                    ns.push(u ^ o);
                    seen.insert(u ^ o);
                }
            }
        }

        s = ns;
    }
}

struct TempLabel {
    vertex: Vec<u8>,
    cost: i64,
}

impl Ord for TempLabel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for TempLabel {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for TempLabel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Eq for TempLabel {}

fn lazy_dijkstra(start: Vec<u8>, ops: &[Vec<u8>]) -> i64 {
    let try_sub = |a: &[u8], b: &[u8]| {
        let mut res = a.to_vec();
        for (i, &x) in a.iter().enumerate() {
            if x < b[i] {
                return None;
            }
            res[i] -= b[i];
        }

        Some(res)
    };

    let adj = |s: &[u8]| {
        let mut vs = vec![];
        for o in ops {
            if let Some(v) = try_sub(s, o) {
                vs.push(v);
            }
        }

        vs
    };

    let mut temp_labels = BinaryHeap::new();
    let mut cur = start;
    let mut cost = 0;

    while cur.iter().sum::<u8>() > 0 {
        for u in adj(&cur) {
            temp_labels.push(Reverse(TempLabel {
                vertex: u,
                cost: cost + 1,
            }));
        }

        let next_temp_label = temp_labels.pop().expect("oh dear");
        cur = next_temp_label.0.vertex;
        cost = next_temp_label.0.cost;
    }

    cost
}

pub fn part_one() -> i64 {
    get().map(|(mask, ops, _)| bfs(mask, &ops)).sum()
}

pub fn part_two() -> i64 {
    get()
        .map(|(_, ops, js)| {
            let n = js.len();
            let start = js.iter().map(|&x| x as u8).collect();
            let ops = ops
                .iter()
                .map(|o| {
                    let mut res = vec![0; n];
                    for i in 0..n {
                        if o & (1 << i) > 0 {
                            res[i] = 1;
                        }
                    }
                    res
                })
                .collect::<Vec<_>>();
            lazy_dijkstra(start, &ops)
        })
        .sum()
}
