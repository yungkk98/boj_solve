use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<u32>,
    lazy: Vec<u32>,
}

impl SegmentTree {
    fn create(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1_f64;
        let tree_size: usize = 2_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![1; tree_size],
            lazy: vec![0; tree_size],
        }
    }

    fn propagate(&mut self, cur: usize, left: usize, right: usize) {
        if self.lazy[cur] == 0 {
            return;
        }

        if left == right {
            self.tree[cur] = self.lazy[cur];
        } else {
            self.lazy[cur * 2] = self.lazy[cur];
            self.lazy[cur * 2 + 1] = self.lazy[cur];
            self.tree[cur] = self.lazy[cur];
        }
        self.lazy[cur] = 0;
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize, value: u32) {
        self.propagate(cur, left, right);
        if start > right || end < left {
            return;
        }

        if start <= left && right <= end {
            self.lazy[cur] = value;
            self.propagate(cur, left, right);
        } else {
            let mid = (left + right) / 2;
            self.update(cur * 2, left, mid, start, end, value);
            self.update(cur * 2 + 1, mid + 1, right, start, end, value);
            self.tree[cur] = self.tree[cur * 2] | self.tree[cur * 2 + 1];
        }
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> u32 {
        self.propagate(cur, left, right);
        if start > right || end < left {
            return 0;
        }

        if start <= left && right <= end {
            return self.tree[cur];
        }
        let mid = (left + right) / 2;
        return self.query(cur * 2, left, mid, start, end) | self.query(cur * 2 + 1, mid + 1, right, start, end);
    }

    fn count_color(mut color_bit: u32) -> u32 {
        let mut result: u32 = 0;
        while color_bit > 0 {
            if color_bit % 2 == 1 {
                result += 1;
            }
            color_bit /= 2;
        }
        return result;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let _: usize = next().parse().unwrap();
    let q: usize = next().parse().unwrap();

    let mut segment_tree: SegmentTree = SegmentTree::create(n);
    for _ in 0..q {
        let qt = next();
        if qt == "C" {
            let (a, b, z): (usize, usize, u32) = (next().parse().unwrap(), next().parse().unwrap(), next().parse().unwrap());
            let (x, y): (usize, usize) = (std::cmp::min(a, b), std::cmp::max(a, b));
            segment_tree.update(1, 0, n - 1, x - 1, y - 1, 1_u32 << (z - 1));
        } else {
            let (a, b): (usize, usize) = (next().parse().unwrap(), next().parse().unwrap());
            let (x, y): (usize, usize) = (std::cmp::min(a, b), std::cmp::max(a, b));
            writeln!(stdout, "{}", SegmentTree::count_color(segment_tree.query(1, 0, n - 1, x - 1, y - 1))).unwrap();
        }
    }
    print!("{stdout}");
}