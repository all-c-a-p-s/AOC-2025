use std::collections::VecDeque;

const INPUT: &'static str = include_str!("./day09.in");

fn get() -> Vec<(i64, i64)> {
    INPUT
        .lines()
        .map(|x| {
            let u = x
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (u[0] - 1, u[1] - 1)
        })
        .collect()
}

pub fn part_one() -> i64 {
    let ps = get();
    let mut ans = 0;
    for &p in &ps {
        for &q in &ps {
            ans = ans.max(((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1));
        }
    }

    ans
}

fn coords_between(a: (i64, i64), b: (i64, i64)) -> Vec<(i64, i64)> {
    if a.0 == b.0 {
        if a.1 > b.1 {
            (b.1..=a.1).map(|x| (a.0, x)).collect()
        } else {
            (a.1..=b.1).map(|x| (a.0, x)).collect()
        }
    } else {
        if a.0 > b.0 {
            (b.0..=a.0).map(|x| (x, a.1)).collect()
        } else {
            (a.0..=b.0).map(|x| (x, a.1)).collect()
        }
    }
}

const GRID_SIZE: usize = 100_000;

fn make_grid(mut v: Vec<(i64, i64)>) -> Vec<Vec<bool>> {
    v.push(v[0]);

    let mut g = (0..GRID_SIZE)
        .map(|_| (0..GRID_SIZE).map(|_| false).collect())
        .collect::<Vec<Vec<bool>>>();

    for w in v.windows(2) {
        let (a, b) = (w[0].clone(), w[1].clone());
        let b = coords_between(a, b);

        for (i, j) in b {
            g[i as usize][j as usize] = true;
        }
    }

    let mut s = (0, 0);

    'outer: for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if j > 1 && !g[i][j] && g[i][j - 1] && !g[i][j - 2] {
                s = (i as i64, j as i64);
                break 'outer;
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back(s);

    let adj = |p: (i64, i64)| {
        vec![
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ]
    };

    while let Some(p) = q.pop_front() {
        if g[p.0 as usize][p.1 as usize] {
            continue;
        }

        g[p.0 as usize][p.1 as usize] = true;

        for x in adj(p) {
            q.push_back(x);
        }
    }

    g
}

pub fn part_two() -> i64 {
    let ps = get();
    let g = make_grid(ps.clone());

    let mut ans = 0;
    for (i, &p) in ps.iter().enumerate() {
        'middle: for &q in ps.iter().skip(i + 1) {
            let sides = vec![
                (p, (p.0, q.1)),
                (p, (q.0, p.1)),
                (q, (p.0, q.1)),
                (q, (q.0, p.1)),
            ];

            for (a, b) in sides {
                let v = coords_between(a, b);
                for x in v {
                    if !g[x.0 as usize][x.1 as usize] {
                        continue 'middle;
                    }
                }
            }
            ans = ans.max(((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1));
        }
    }

    ans
}
