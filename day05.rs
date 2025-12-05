const INPUT: &'static str = include_str!("./day05.in");

fn get() -> (Vec<(i128, i128)>, Vec<i128>) {
    let mut rs = vec![];
    let mut ns = vec![];

    for l in INPUT.lines() {
        if l.is_empty() {
            continue;
        }
        if l.contains("-") {
            let parts = l.split("-").map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            rs.push((parts[0], parts[1]));
        } else {
            ns.push(l.trim().parse().unwrap());
        }
    }

    (rs, ns)
}

pub fn part_one() -> i128 {
    let (rs, ns) = get();
    ns.iter()
        .map(|&x| {
            for &r in &rs {
                if r.0 <= x && r.1 >= x {
                    return 1;
                }
            }

            0
        })
        .sum()
}
pub fn part_two() -> i128 {
    let (mut rs, _) = get();
    let n = rs.len();

    let mut u = vec![true; n];

    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            if rs[j].0 >= rs[i].0 && rs[j].0 <= rs[i].1 {
                rs[i].1 = rs[i].1.max(rs[j].1);
                u[i] = true;
                u[j] = false;
            } else if rs[j].1 >= rs[i].0 && rs[j].1 <= rs[i].1 {
                rs[i].0 = rs[i].0.min(rs[j].0);
                u[i] = true;
                u[j] = false;
            }
        }
    }

    (0..n)
        .filter(|&i| u[i])
        .map(|i| rs[i].1 - rs[i].0 + 1)
        .sum()
}
