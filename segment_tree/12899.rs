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

    fn add_number(&mut self, cur: usize, left: usize, right: usize, value: usize) {
        if left > value || right < value {
            return;
        }

        if left == right {
            self.tree[cur] += 1;
            return;
        }

        let mid = (left + right) / 2;
        self.add_number(cur * 2, left, mid, value);
        self.add_number(cur * 2 + 1, mid + 1, right, value);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn query_and_remove(&mut self, cur: usize, left: usize, right: usize, k: usize) -> usize {
        if left == right {
            self.tree[cur] -= 1;
            return left;
        }

        let mid = (left + right) / 2;
        let result: usize;
        if self.tree[cur * 2] >= k {
            result = self.query_and_remove(cur * 2, left, mid, k);
        } else {
            result = self.query_and_remove(cur * 2 + 1, mid + 1, right, k - self.tree[cur * 2]);
        }

        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
        return result;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();
    
    let n: usize = next().parse().unwrap();
    let mut segment_tree: SegmentTree = SegmentTree::init(2000001);
    for _ in 0..n {
        let t: usize = next().parse().unwrap();
        let x: usize = next().parse().unwrap();
        if t == 1 {
            segment_tree.add_number(1, 1, 2000001, x);
        } else {
            writeln!(stdout, "{}", segment_tree.query_and_remove(1, 1, 2000001, x)).unwrap();
        }
    }

    print!("{stdout}");
}