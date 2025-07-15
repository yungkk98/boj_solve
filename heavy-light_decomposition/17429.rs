use std::fmt::Write;

const MOD: u64 = 4294967296;

pub struct SegmentTree {
    tree: Vec<u64>,
    lazy_sum: Vec<u64>,
    lazy_mul: Vec<u64>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![0; tree_size],
            lazy_sum: vec![0; tree_size],
            lazy_mul: vec![1; tree_size],
        }
    }

    fn propagate(&mut self, cur: usize, left: usize, right: usize) {
        self.tree[cur] = (self.tree[cur] * self.lazy_mul[cur]) % MOD;
        self.tree[cur] = (self.tree[cur] + self.lazy_sum[cur] * (right - left + 1) as u64) % MOD;

        if left != right {
            self.lazy_sum[cur * 2] = ((self.lazy_sum[cur * 2] * self.lazy_mul[cur]) % MOD + self.lazy_sum[cur]) % MOD;
            self.lazy_mul[cur * 2] = (self.lazy_mul[cur * 2] * self.lazy_mul[cur]) % MOD;
            self.lazy_sum[cur * 2 + 1] = ((self.lazy_sum[cur * 2 + 1] * self.lazy_mul[cur]) % MOD + self.lazy_sum[cur]) % MOD;
            self.lazy_mul[cur * 2 + 1] = (self.lazy_mul[cur * 2 + 1] * self.lazy_mul[cur]) % MOD;
        }

        self.lazy_mul[cur] = 1;
        self.lazy_sum[cur] = 0;
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize, mul_value: u64, sum_value: u64) {
        self.propagate(cur, left, right);
        if left > end || start > right {
            return;
        }

        if start <= left && right <= end {
            self.lazy_sum[cur] = ((self.lazy_sum[cur] * mul_value) % MOD + sum_value) % MOD;
            self.lazy_mul[cur] = (self.lazy_mul[cur] * mul_value) % MOD;
            self.propagate(cur, left, right);
            return;
        }

        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, start, end, mul_value, sum_value);
        self.update(cur * 2 + 1, mid + 1, right, start, end, mul_value, sum_value);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> u64 {
        self.propagate(cur, left, right);
        if left > end || start > right {
            return 0;
        }
        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return (self.query(cur * 2, left, mid, start, end) + self.query(cur * 2 + 1, mid + 1, right, start, end)) % MOD;
    }
}

pub struct HLD {
    size: Vec<usize>,
    depth: Vec<usize>,
    top: Vec<usize>,
    parent: Vec<usize>,
    inn: Vec<usize>,
    out: Vec<usize>,
    graph: Vec<Vec<usize>>,
    edges: Vec<Vec<usize>>,
    dfs_check: Vec<bool>,
    segment_tree: SegmentTree,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            size: vec![0; n],
            depth: vec![0; n],
            top: vec![0; n],
            parent: vec![0; n],
            inn: vec![0; n],
            out: vec![0; n],
            graph: vec![vec![]; n],
            edges: vec![vec![]; n],
            dfs_check: vec![false; n],
            segment_tree: SegmentTree::init(n),
        }
    }

    fn dfs(&mut self, cur: usize) {
        self.dfs_check[cur] = true;
        for index in 0..self.edges[cur].len() {
            let next = self.edges[cur][index];
            if self.dfs_check[next] {
                continue;
            }
            self.dfs_check[next] = true;
            self.graph[cur].push(next);
            self.dfs(next);
        }
    }

    fn dfs1(&mut self, cur: usize) {
        self.size[cur] = 1;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            self.depth[next] = self.depth[cur] + 1;
            self.parent[next] = cur;
            self.dfs1(next);
            self.size[cur] += self.size[next];
            if self.size[next] > self.size[self.graph[cur][0]] {
                self.graph[cur][index] = self.graph[cur][0];
                self.graph[cur][0] = next;
            }
        }
    }

    fn dfs2(&mut self, cur: usize, prev: &mut usize) {
        *prev += 1;
        self.inn[cur] = *prev;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            if index == 0 {
                self.top[next] = self.top[cur]
            } else {
                self.top[next] = next;
            }
            self.dfs2(next, prev);
        }
        self.out[cur] = *prev;
    }

    fn update_path(&mut self, mut a: usize, mut b: usize, mul_value: u64, sum_value: u64) {
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            self.segment_tree.update(1, 1, self.size[1], self.inn[self.top[a]], self.inn[a], mul_value, sum_value);
            a = self.parent[self.top[a]];
        }

        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.segment_tree.update(1, 1, self.size[1], self.inn[a], self.inn[b], mul_value, sum_value);
    }

    fn query_path(&mut self, mut a: usize, mut b: usize) -> u64{
        let mut result: u64 = 0;
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            result = (result + self.segment_tree.query(1, 1, self.size[1], self.inn[self.top[a]], self.inn[a])) % MOD;
            a = self.parent[self.top[a]];
        }

        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        result = (result + self.segment_tree.query(1, 1, self.size[1], self.inn[a], self.inn[b])) % MOD;
        
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

    for _ in 1..n {
        let (s, e): (usize, usize) = (next().parse().unwrap(), next().parse().unwrap());
        hld.edges[s].push(e);
        hld.edges[e].push(s);
    }

    hld.dfs(1);
    hld.dfs1(1);
    hld.dfs2(1, &mut 0);

    for _ in 0..q {
        let t: i8 = next().parse().unwrap();
        let x: usize = next().parse().unwrap();
        if t == 1 {
            // add v subtree of x
            let v: u64 = next().parse().unwrap();
            hld.segment_tree.update(1, 1, hld.size[1], hld.inn[x], hld.out[x], 1, v);
        } else if t == 2 {
            // add v path from x to y
            let y: usize = next().parse().unwrap();
            let v: u64 = next().parse().unwrap();
            hld.update_path(x, y, 1, v);
        } else if t == 3 {
            // mul v subtree of x
            let v: u64 = next().parse().unwrap();
            hld.segment_tree.update(1, 1, hld.size[1], hld.inn[x], hld.out[x], v, 0);
        } else if t == 4 {
            // mul v path from x to y
            let y: usize = next().parse().unwrap();
            let v: u64 = next().parse().unwrap();
            hld.update_path(x, y, v, 0);
        } else if t == 5 {
            // print sum of subtree x
            writeln!(stdout, "{}", hld.segment_tree.query(1, 1, hld.size[1], hld.inn[x], hld.out[x])).unwrap();
        } else if t == 6 {
            // print sum of path from x to y
            let y: usize = next().parse().unwrap();
            writeln!(stdout, "{}", hld.query_path(x, y)).unwrap();
        }
    }
    print!("{stdout}");
}