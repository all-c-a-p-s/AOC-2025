use std::collections::HashMap;

const INPUT: &'static str = include_str!("./day11.in");

fn get() -> HashMap<String, Vec<String>> {
    let mut m = HashMap::new();

    for l in INPUT.lines() {
        let mut k = None;
        let mut vs = vec![];

        for w in l.split_whitespace() {
            if k.is_none() {
                k = w.strip_suffix(":");
            } else {
                vs.push(w.to_string());
            }
        }

        m.insert(k.unwrap().to_string(), vs);
    }

    m
}

fn dfs(cur: String, m: &mut HashMap<String, i64>, g: &HashMap<String, Vec<String>>) {
    if m.contains_key(&cur) {
        return;
    }

    let Some(adj) = g.get(&cur) else {
        unreachable!()
    };

    let mut t = 0;

    for u in adj {
        dfs(u.clone(), m, g);
        t += m.get(u).unwrap_or(&0);
    }

    m.insert(cur, t);
}

pub fn part_one() -> i64 {
    let g = get();
    let mut m = HashMap::new();
    m.insert("out".to_string(), 1);

    dfs("you".to_string(), &mut m, &g);

    m["you"]
}

fn solve_ordered(f: String, t: String, g: &HashMap<String, Vec<String>>) -> i64 {
    let mut m = HashMap::new();
    m.insert(t, 1);

    dfs(f.clone(), &mut m, g);
    m[&f]
}

pub fn part_two() -> i64 {
    let mut g = get();
    g.insert("out".to_string(), vec![]);
    let (s, d, f, o) = (
        "svr".to_string(),
        "dac".to_string(),
        "fft".to_string(),
        "out".to_string(),
    );
    solve_ordered(s.clone(), d.clone(), &g)
        * solve_ordered(d.clone(), f.clone(), &g)
        * solve_ordered(f.clone(), o.clone(), &g)
        + solve_ordered(s.clone(), f.clone(), &g)
            * solve_ordered(f.clone(), d.clone(), &g)
            * solve_ordered(d.clone(), o.clone(), &g)
}
