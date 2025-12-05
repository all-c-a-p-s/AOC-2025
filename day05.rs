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

fn get_non_overlapping(mut rs: Vec<(i128, i128)>) -> Vec<(i128, i128)> {
    rs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut v = vec![];

    let mut prev = rs[0];

    for x in rs {
        if prev.1 < x.0 {
            v.push(prev);
            prev = x;
            continue;
        }

        prev.1 = prev.1.max(x.1);
    }

    v.push(prev);

    v
}

pub fn part_one() -> i128 {
    let (rs, ns) = get();

    let v = get_non_overlapping(rs);
    let n = v.len();

    let mut cnt = 0;

    for x in ns {
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if v[mid].1 < x {
                lo = mid + 1;
            } else if v[mid].0 > x {
                hi = mid - 1;
            } else {
                lo = mid;
                break;
            }
        }

        cnt += i128::from(v[lo].0 <= x && v[lo].1 >= x);
    }

    cnt
}

pub fn part_two() -> i128 {
    let (rs, _) = get();

    let v = get_non_overlapping(rs);
    v.iter().map(|x| x.1 - x.0 + 1).sum()
}
