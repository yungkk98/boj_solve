use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<bool>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![false; tree_size],
        }
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, index: usize, value: bool) {
        if index < left || index > right {
            return;
        }

        if left == right {
            self.tree[cur] = value;
            return;
        }
        
        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, index, value);
        self.update(cur * 2 + 1, mid + 1, right, index, value);
        self.tree[cur] = self.tree[cur * 2] | self.tree[cur * 2 + 1];
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> bool {
        if start > right || end < left {
            return false;
        }
        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return self.query(cur * 2, left, mid, start, end) | self.query(cur * 2 + 1, mid + 1, right, start, end);
    }
}

pub struct HLD {
    size: Vec<usize>,
    depth: Vec<usize>,
    parent: Vec<usize>,
    top: Vec<usize>,
    inn: Vec<usize>,
    out: Vec<usize>,
    graph: Vec<Vec<usize>>,
    segment_tree: SegmentTree,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            size: vec![0; n],
            depth: vec![0; n],
            parent: vec![0; n],
            top: vec![0; n],
            inn: vec![0; n],
            out: vec![0; n],
            graph: vec![vec![]; n],
            segment_tree: SegmentTree::init(n),
        }
    }

    fn dfs1(&mut self, cur: usize) {
        // calculate size of subtree and move heavy to 0
        self.size[cur] = 1;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            self.depth[next] = self.depth[cur] + 1;
            self.dfs1(next);
            self.size[cur] += self.size[next];
            if self.size[next] > self.size[self.graph[cur][0]] {
                self.graph[cur].swap(0, index);
            }
        }
    }

    fn dfs2(&mut self, cur: usize, prev: &mut usize) {
        *prev += 1;
        self.inn[cur] = *prev;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            if index == 0 {
                self.top[next] = self.top[cur];
            } else {
                self.top[next] = next;
            }
            self.dfs2(next, prev);
        }
        self.out[cur] = *prev;
    }

    fn update(&mut self, index: usize, value: bool) {
        self.segment_tree.update(1, 1, self.size[1], self.inn[index], value);
    }

    fn query(&mut self, mut a: usize, mut b: usize) -> bool {
        let mut result: bool = false;
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            let start = self.top[a];
            result |= self.segment_tree.query(1, 1, self.size[1], self.inn[start], self.inn[a]);
            a = self.parent[start];
        }
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        result |= self.segment_tree.query(1, 1, self.size[1], self.inn[a] + 1, self.inn[b]);

        return result;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let q: usize = next().parse().unwrap();
    let mut hld: HLD = HLD::init(n + 1);
    for i in 2..=n {
        let p: usize = next().parse().unwrap();
        hld.parent[i] = p;
        hld.graph[p].push(i);
    }

    hld.dfs1(1);
    hld.dfs2(1, &mut 0);

    for _ in 0..q {
        let (b, c, d): (usize, usize, usize) = (next().parse().unwrap(), next().parse().unwrap(), next().parse().unwrap());
        let result = hld.query(b, c);
        if result {
            writeln!(stdout, "NO").unwrap();
            if d == 1 {
                hld.update(c, true);
            }
        } else {
            writeln!(stdout, "YES").unwrap();
            if d == 1 {
                hld.update(b, true);
            }
        }
    }
    print!("{stdout}");
}