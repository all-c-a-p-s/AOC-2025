const INPUT: &'static str = include_str!("./day04.in");

struct IndexedMinHeap {
    heap: Vec<usize>,
    pos: Vec<Option<usize>>,
    keys: Vec<i64>,
}

impl IndexedMinHeap {
    fn new(num_nodes: usize) -> Self {
        Self {
            heap: Vec::new(),
            pos: vec![None; num_nodes],
            keys: vec![0; num_nodes],
        }
    }

    fn push(&mut self, v: usize, key: i64) {
        self.keys[v] = key;
        self.heap.push(v);

        let idx = self.heap.len() - 1;
        self.pos[v] = Some(idx);

        self.heapify_up(idx);
    }

    fn pop(&mut self) -> Option<usize> {
        if self.heap.is_empty() {
            return None;
        }
        let min = self.heap[0];
        self.swap_positions(0, self.heap.len() - 1);

        self.heap.pop();
        self.pos[min] = None;

        if !self.heap.is_empty() {
            self.heapify_down(0);
        }

        Some(min)
    }

    fn decrease_key(&mut self, v: usize) {
        self.keys[v] -= 1;

        if let Some(i) = self.pos[v] {
            self.heapify_up(i);
        }
    }

    fn heapify_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = (i - 1) / 2;
            if self.keys[self.heap[i]] >= self.keys[self.heap[p]] {
                break;
            }
            self.swap_positions(i, p);
            i = p;
        }
    }

    fn heapify_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut smallest = i;

            if l < n && self.keys[self.heap[l]] < self.keys[self.heap[smallest]] {
                smallest = l;
            }
            if r < n && self.keys[self.heap[r]] < self.keys[self.heap[smallest]] {
                smallest = r;
            }

            if smallest == i {
                break;
            }

            self.swap_positions(i, smallest);
            i = smallest;
        }
    }

    fn swap_positions(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);

        let vi = self.heap[i];
        let vj = self.heap[j];

        self.pos[vi] = Some(i);
        self.pos[vj] = Some(j);
    }

    fn key(&self, v: usize) -> i64 {
        self.keys[v]
    }
}

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

    let mut h = IndexedMinHeap::new(rows * cols);
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

    for (i, v) in adj_list.iter().enumerate() {
        if grid[i / rows][i % rows] != '@' {
            continue;
        }
        h.push(i, v.len() as i64);
    }

    let mut t = 0;

    while let Some(u) = h.pop() {
        if h.key(u) >= 4 {
            break;
        }

        for &v in &adj_list[u] {
            h.decrease_key(v);
        }

        t += 1;
    }

    t
}
