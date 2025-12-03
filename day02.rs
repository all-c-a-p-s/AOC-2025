use std::collections::HashSet;

const INPUT: &'static str = include_str!("./day02.in");

fn get() -> Vec<(i128, i128)> {
    INPUT
        .split(",")
        .map(|x| {
            let t = x
                .split("-")
                .map(|y| y.trim().parse::<i128>().unwrap())
                .collect::<Vec<_>>();
            (t[0], t[1])
        })
        .collect()
}

fn digits(mut n: i128) -> Vec<i128> {
    let mut ds = vec![];
    while n > 0 {
        ds.push(n % 10);
        n /= 10;
    }
    ds.reverse();
    ds
}

fn digits_to_num(ds: &[i128]) -> i128 {
    ds.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + x * 10i128.pow(i as u32))
}

pub fn part_one() -> i128 {
    let ranges = get();
    let mut t = 0;
    for (x, y) in ranges {
        let d_x = digits(x);

        let lo = if d_x.len() % 2 == 0 {
            x
        } else {
            10i128.pow((x * 10).ilog10() as u32)
        };

        let d_l = digits(lo);
        let halved = &d_l[0..d_l.len() / 2];

        let mut i = digits_to_num(halved);

        loop {
            let ds = digits(i);
            let d_n = vec![ds.clone(), ds].concat();
            let n = digits_to_num(&d_n);

            if n >= x && n <= y {
                t += n;
            } else if n > y {
                break;
            }

            i += 1;
        }
    }
    t
}

pub fn part_two() -> i128 {
    let ranges = get();
    let mut t = 0;
    for (x, y) in ranges {
        let (d_x, d_y) = (digits(x), digits(y));

        let mut h = HashSet::new();

        for p in 2..=d_y.len() {
            let lo = if d_x.len() % p == 0 {
                x
            } else {
                let mut u = 10i128.pow(x.ilog10() as u32);
                while digits(u).len() % p > 0 {
                    u *= 10
                }
                u
            };

            let d_l = digits(lo);
            let part = &d_l[0..d_l.len() / p];

            let mut i = digits_to_num(part);

            loop {
                let ds = digits(i);
                let d_n = (0..p).map(|_| ds.clone()).collect::<Vec<_>>().concat();
                let n = digits_to_num(&d_n);

                if n >= x && n <= y && !h.contains(&n) {
                    h.insert(n);

                    t += n;
                } else if n > y {
                    break;
                }

                i += 1;
            }
        }
    }
    t
}
