use std::collections::{HashSet, VecDeque};

const INPUT: &'static str = include_str!("./day07.in");

fn get() -> Vec<Vec<char>> {
    INPUT.lines().map(|l| l.chars().collect()).collect()
}

fn is_valid(r: i64, c: i64, lenr: usize, lenc: usize) -> bool {
    r >= 0 && c >= 0 && r < lenr as i64 && c < lenc as i64
}

pub fn part_one() -> i64 {
    let lines = get();

    let (rows, cols) = (lines.len(), lines[0].len());
    let mut vis = (0..rows)
        .map(|_| (0..cols).map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut q = VecDeque::new();

    let mut h = HashSet::new();

    for (r, l) in lines.iter().enumerate() {
        for (c, &x) in l.iter().enumerate() {
            if x == 'S' {
                q.push_back((r as i64, c as i64));
                break;
            }
        }
    }

    while let Some(u) = q.pop_front() {
        let v = (u.0 + 1, u.1);

        if !is_valid(v.0, v.1, rows, cols) {
            continue;
        }

        if lines[v.0 as usize][v.1 as usize] == '^' {
            h.insert(v);
            let l = (v.0, v.1 - 1);
            if is_valid(l.0, l.1, rows, cols) && !vis[l.0 as usize][l.1 as usize] {
                vis[l.0 as usize][l.1 as usize] = true;
                q.push_back(l);
            }

            let r = (v.0, v.1 + 1);
            if is_valid(r.0, r.1, rows, cols) && !vis[r.0 as usize][r.1 as usize] {
                vis[r.0 as usize][r.1 as usize] = true;
                q.push_back(r);
            }
        } else if !vis[v.0 as usize][v.1 as usize] {
            vis[v.0 as usize][v.1 as usize] = true;
            q.push_back(v);
        }
    }

    h.len() as i64
}

pub fn part_two() -> i64 {
    let lines = get();

    let (rows, cols) = (lines.len(), lines[0].len());
    let mut dp = (0..rows)
        .map(|_| (0..cols).map(|_| None).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);

    for (r, l) in lines.iter().enumerate() {
        for (c, &x) in l.iter().enumerate() {
            if x == 'S' {
                start = (r as i64, c as i64);
                break;
            }
        }
    }

    fn dfs(u: (i64, i64), dp: &mut Vec<Vec<Option<i64>>>, lines: &[Vec<char>]) {
        let (rows, cols) = (lines.len(), lines[0].len());

        if dp[u.0 as usize][u.1 as usize].is_some() {
            return;
        }

        if lines[u.0 as usize][u.1 as usize] == '^' {
            let mut t = 0;
            let l = (u.0, u.1 - 1);
            if is_valid(l.0, l.1, rows, cols) {
                dfs(l, dp, lines);
                t += dp[l.0 as usize][l.1 as usize].unwrap();
            }

            let r = (u.0, u.1 + 1);
            if is_valid(r.0, r.1, rows, cols) {
                dfs(r, dp, lines);
                t += dp[r.0 as usize][r.1 as usize].unwrap();
            }

            dp[u.0 as usize][u.1 as usize] = Some(t);
        } else {
            let v = (u.0 + 1, u.1);
            if is_valid(v.0, v.1, rows, cols) {
                dfs(v, dp, lines);
                dp[u.0 as usize][u.1 as usize] = dp[v.0 as usize][v.1 as usize];
            } else {
                dp[u.0 as usize][u.1 as usize] = Some(1);
            }
        }
    }

    dfs(start, &mut dp, &lines);

    dp[start.0 as usize][start.1 as usize].unwrap()
}
