use std::fmt::Write;

pub struct SegmentTree {
    segment_tree: Vec<i64>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth = (n as f64).log2().ceil() + 1_f64;
        let tree_size = 2_f64.powf(tree_depth) as usize;

        SegmentTree {
            segment_tree: vec![0; tree_size],
        }
    }

    fn setup(&mut self, cur: usize, left: usize, right: usize, num: &Vec<i64>) {
        if left == right {
            self.segment_tree[cur] = num[left];
            return;
        }

        let mid = (left + right) / 2;
        self.setup(cur * 2, left, mid, num);
        self.setup(cur * 2 + 1, mid + 1, right, num);
        self.segment_tree[cur] = self.segment_tree[cur * 2] + self.segment_tree[cur * 2 + 1];
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, index: usize, diff: i64) {
        if left == right {
            self.segment_tree[cur] += diff;
            return;
        }
        if index < left || right < index {
            return;
        }

        let mid = (left + right) / 2;
        if mid >= index {
            self.update(cur * 2, left, mid, index, diff);
        } else {
            self.update(cur * 2 + 1, mid + 1, right, index, diff);
        }
        self.segment_tree[cur] = self.segment_tree[cur * 2] + self.segment_tree[cur * 2 + 1];
    }

    fn query(&self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> i64 {
        if start > right || end < left {
            return 0;
        }
        if start <= left && right <= end {
            return self.segment_tree[cur];
        }

        let mid = (left + right) / 2;
        return self.query(cur * 2, left, mid, start, end) + self.query(cur * 2 + 1, mid + 1, right, start, end);
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let q: usize = next().parse().unwrap();
    let mut num: Vec<i64> = (0..n)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();

    let mut tree: SegmentTree = SegmentTree::init(n);
    tree.setup(1, 0, n - 1, &num);

    for _ in 0..q {
        let (x, y, a, b): (usize, usize, usize, i64) = (next().parse().unwrap(), next().parse().unwrap(), next().parse().unwrap(), next().parse().unwrap());
        if x < y {
            writeln!(stdout, "{}", tree.query(1, 0, n - 1, x - 1, y - 1)).unwrap();
        } else {
            writeln!(stdout, "{}", tree.query(1, 0, n - 1, y - 1, x - 1)).unwrap();
        }
        tree.update(1, 0, n - 1, a - 1, b - num[a - 1]);
        num[a - 1] = b;
    }

    print!("{stdout}");
}