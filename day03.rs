const INPUT: &'static str = include_str!("./day03.in");

pub fn get() -> impl Iterator<Item = Vec<u8>> {
    INPUT
        .lines()
        .map(|x| x.bytes().map(|y| y - b'0').collect::<Vec<_>>())
}

fn r(x: &[u8], i: usize, k: usize) -> i64 {
    let n = x.len();
    let m = (i..n - k + 1).map(|j| x[j]).max().unwrap();

    if k == 1 {
        return m as i64;
    }

    let j = i + (i..n - k + 1).position(|y| x[y] == m).unwrap();

    m as i64 * 10i64.pow(k as u32 - 1) + r(x, j + 1, k - 1)
}

pub fn part_one() -> i64 {
    let ls = get();

    ls.map(|x| r(&x, 0, 2)).sum()
}

pub fn part_two() -> i64 {
    let ls = get();

    ls.map(|x| r(&x, 0, 12)).sum()
}
