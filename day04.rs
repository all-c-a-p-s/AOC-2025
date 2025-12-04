use std::collections::VecDeque;

const INPUT: &'static str = include_str!("./day04.in");

fn get() -> Vec<Vec<char>> {
    INPUT.lines().map(|x| x.chars().collect()).collect()
}

fn is_valid(r: i64, c: i64, lenr: usize, lenc: usize) -> bool {
    r >= 0 && c >= 0 && r < lenr as i64 && c < lenc as i64
}

const D8: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one() -> i64 {
    let grid = get();
    let (rows, cols) = (grid.len() as i64, grid[0].len() as i64);

    let mut t = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] != '@' {
                continue;
            }
            let mut cnt = 0;
            for d in D8 {
                let (nr, nc) = (r + d.0, c + d.1);
                if !is_valid(nr, nc, rows as usize, cols as usize) {
                    continue;
                }
                cnt += i64::from(grid[nr as usize][nc as usize] == '@');
            }

            t += i64::from(cnt < 4);
        }
    }

    t
}

pub fn part_two() -> i64 {
    let grid = get();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut q = VecDeque::new();

    let mut adj_list = vec![vec![]; rows * cols];

    for r in 0..rows as i64 {
        for c in 0..cols as i64 {
            if grid[r as usize][c as usize] != '@' {
                continue;
            }
            for d in D8 {
                let (nr, nc) = (r + d.0, c + d.1);
                if !is_valid(nr, nc, rows as usize, cols as usize) {
                    continue;
                }

                let (f, nf) = (
                    r as usize * cols + c as usize,
                    nr as usize * cols + nc as usize,
                );

                if grid[nr as usize][nc as usize] == '@' {
                    adj_list[f].push(nf);
                }
            }
        }
    }

    let mut deg = vec![0; rows * cols];

    for (i, v) in adj_list.iter().enumerate() {
        if grid[i / rows][i % rows] != '@' {
            continue;
        }

        deg[i] = v.len();
        if deg[i] < 4 {
            q.push_back(i);
        }
    }

    let mut t = 0;

    while let Some(u) = q.pop_front() {
        t += 1;

        for &v in &adj_list[u] {
            if deg[v] == 4 {
                q.push_back(v);
            }
            deg[v] -= 1;
        }
    }

    t
}
