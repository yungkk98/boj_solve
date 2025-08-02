use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<usize>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![0; tree_size],
        }
    }

    fn build(&mut self, cur: usize, left: usize, right: usize, numbers: &Vec<usize>) {
        if left == right {
            self.tree[cur] = numbers[left];
            return;
        }

        let mid = (left + right) / 2;
        self.build(cur * 2, left, mid, numbers);
        self.build(cur * 2 + 1, mid + 1, right, numbers);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, index: usize, diff: i64) {
        if left > index || right < index {
            return;
        }

        if left == right {
            if diff > 0 {
                self.tree[cur] += diff as usize;
            } else {
                self.tree[cur] -= (-1 * diff) as usize;
            }
            return;
        }
        
        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, index, diff);
        self.update(cur * 2 + 1, mid + 1, right, index, diff);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, index: usize) -> usize {
        if left == right {
            return left;
        }

        let mid = (left + right) / 2;
        if self.tree[cur * 2] >= index {
            return self.query(cur * 2, left, mid, index);
        } else {
            return self.query(cur * 2 + 1, mid + 1, right, index - self.tree[cur * 2]);
        }
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let mut numbers: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        numbers[i] = next().parse().unwrap();
    }

    let mut segment_tree: SegmentTree = SegmentTree::init(n + 1);
    segment_tree.build(1, 1, n, &numbers);

    let m: usize = next().parse().unwrap();
    for _ in 0..m {
        let t: usize = next().parse().unwrap();
        if t == 1 {
            let a: usize = next().parse().unwrap();
            let diff: i64 = next().parse().unwrap();
            segment_tree.update(1, 1, n, a, diff);
        } else {
            let a: usize = next().parse().unwrap();
            writeln!(stdout, "{}", segment_tree.query(1, 1, n, a)).unwrap();
        }
    }
    print!("{stdout}");
}