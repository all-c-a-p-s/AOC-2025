const INPUT: &'static str = include_str!("./day06.in");

fn get() -> (Vec<Vec<i128>>, Vec<String>) {
    let lines = INPUT.lines();

    let mut nums = vec![];
    let mut ops = vec![];

    for l in lines {
        let l = l.trim_start();
        if l.chars().nth(0).unwrap().is_ascii_digit() {
            let ns = l.split_whitespace().map(|y| y.parse::<i128>().unwrap());

            for (i, n) in ns.enumerate() {
                if nums.len() <= i {
                    nums.push(vec![]);
                }

                nums[i].push(n);
            }
        } else {
            for op in l.split_whitespace() {
                ops.push(op.to_string());
            }
        }
    }

    (nums, ops)
}

pub fn part_one() -> i128 {
    let (nums, ops) = get();
    let n = ops.len();

    (0..n)
        .map(|i| {
            let (f, id): (Box<dyn Fn(i128, i128) -> i128>, i128) = match ops[i].as_str() {
                "+" => (Box::new(|a, b| a + b), 0),
                "*" => (Box::new(|a, b| a * b), 1),
                _ => unreachable!(),
            };

            nums[i].iter().fold(id, |acc, &x| f(acc, x))
        })
        .sum()
}

fn get_two() -> Vec<Vec<char>> {
    let vcs = INPUT
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (0..vcs[0].len())
        .map(|i| (0..vcs.len()).map(|j| vcs[j][i]).collect())
        .collect()
}

pub fn part_two() -> i128 {
    let mut cols = get_two();
    let spcs = (0..cols[0].len()).map(|_| ' ').collect();
    cols.insert(0, spcs);

    let mt = |col: &[char]| col.iter().filter(|&&x| x != ' ').count() == 0;

    let mut op = "+";
    let mut ns = vec![];

    let mut t = 0;

    for c in cols.iter().rev() {
        if mt(c) {
            let (f, id): (Box<dyn Fn(i128, i128) -> i128>, i128) = match op {
                "+" => (Box::new(|a, b| a + b), 0),
                "*" => (Box::new(|a, b| a * b), 1),
                _ => unreachable!(),
            };

            t += ns.iter().fold(id, |acc, &x| f(acc, x));
            ns = vec![];
            continue;
        }

        let mut s = c.iter().filter(|&&x| x != ' ').collect::<String>();
        if s.ends_with("+") {
            op = "+";
            s.pop();
        } else if s.ends_with("*") {
            op = "*";
            s.pop();
        }
        ns.push(s.parse().unwrap());
    }

    t
}
