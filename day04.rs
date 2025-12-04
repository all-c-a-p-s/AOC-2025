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

fn h(grid: &mut Vec<Vec<char>>) -> i64 {
    let (rows, cols) = (grid.len() as i64, grid[0].len() as i64);

    let mut is = vec![];

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

            if cnt < 4 {
                t += 1;
                is.push((r, c));
            }
        }
    }

    for (r, c) in is {
        grid[r as usize][c as usize] = '.';
    }

    t
}

pub fn part_one() -> i64 {
    let mut grid = get();

    h(&mut grid)
}

pub fn part_two() -> i64 {
    let mut grid = get();

    let mut t = 0;

    loop {
        let u = h(&mut grid);
        if u == 0 {
            break;
        }
        t += u;
    }

    t
}
