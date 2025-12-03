const INPUT: &'static str = include_str!("./day01.in");

fn get() -> impl Iterator<Item = i64> {
    INPUT.lines().map(|x| {
        x.chars()
            .skip(1)
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
            * if x.chars().nth(0) == Some('R') { 1 } else { -1 }
    })
}

pub fn part_one() -> i64 {
    let moves = get();

    let mut pos = 50;
    let mut cnt = 0;

    for m in moves {
        pos = (pos + 100 + (m.signum() * (m.abs() % 100))) % 100;
        cnt += i64::from(pos == 0);
    }

    cnt
}

pub fn part_two() -> i64 {
    let moves = get();

    let mut pos = 50;
    let mut cnt = 0;

    for m in moves {
        let mut clicks = if m > 0 { m + pos } else { 100 - pos + m.abs() } / 100;
        clicks -= i64::from(m < 0 && pos == 0);

        pos = (pos + 100 + (m.signum() * (m.abs() % 100))) % 100;
        cnt += clicks;
    }

    cnt
}
