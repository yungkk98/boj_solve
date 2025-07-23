use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![0; tree_size],
            lazy: vec![-1; tree_size],
        }
    }

    fn propagate(&mut self, cur: usize, left: usize, right: usize) {
        if self.lazy[cur] == -1 {
            return;
        }
        self.tree[cur] = self.lazy[cur] * (right - left + 1) as i64;
        if left != right {
            self.lazy[cur * 2] = self.lazy[cur];
            self.lazy[cur * 2 + 1] = self.lazy[cur];
        }
        self.lazy[cur] = -1;
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize, value: i64) {
        self.propagate(cur, left, right);
        if left > end || start > right {
            return;
        }

        if start <= left && right <= end {
            self.lazy[cur] = value;
            self.propagate(cur, left, right);
            return;
        }
        
        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, start, end, value);
        self.update(cur * 2 + 1, mid + 1, right, start, end, value);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> i64 {
        self.propagate(cur, left, right);
        if left > end || start > right {
            return 0;
        }

        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return self.query(cur * 2, left, mid, start, end) + self.query(cur * 2 + 1, mid + 1, right, start, end);
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
            segment_tree: SegmentTree::init(n),
        }
    }

    fn dfs1(&mut self, cur: usize) {
        self.size[cur] = 1;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            self.parent[next] = cur;
            self.depth[next] = self.depth[cur] + 1;
            self.dfs1(next);
            self.size[cur] += self.size[next];
            if self.size[next] > self.size[self.graph[cur][0]] {
                self.graph[cur][index] = self.graph[cur][0];
                self.graph[cur][0] = next;
            }
        }
    }

    fn dfs2(&mut self, cur: usize, pv: &mut usize) {
        *pv += 1;
        self.inn[cur] = *pv;
        for index in 0..self.graph[cur].len() {
            let next = self.graph[cur][index];
            if index == 0 {
                self.top[next] = self.top[cur];
            } else {
                self.top[next] = next;
            }
            self.dfs2(next, pv);
        }
        self.out[cur] = *pv;
    }

    fn update_path(&mut self, mut u: usize, mut v: usize, value: i64) {
        while self.top[u] != self.top[v] {
            if self.depth[self.top[u]] < self.depth[self.top[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            
            self.segment_tree.update(1, 1, self.size[1], self.inn[self.top[u]], self.inn[u], value);
            u = self.parent[self.top[u]];
        }

        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.segment_tree.update(1, 1, self.size[1], self.inn[u] + 1, self.inn[v], value);
    }

    fn query_path(&mut self, mut u: usize, mut v: usize) -> i64 {
        let mut result: i64 = 0;
        while self.top[u] != self.top[v] {
            if self.depth[self.top[u]] < self.depth[self.top[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            
            result += self.segment_tree.query(1, 1, self.size[1], self.inn[self.top[u]], self.inn[u]);
            u = self.parent[self.top[u]];
        }

        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        result += self.segment_tree.query(1, 1, self.size[1], self.inn[u] + 1, self.inn[v]);
        return result;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let mut hld: HLD = HLD::init(n + 1);
    for _ in 1..n {
        let a: usize = next().parse().unwrap();
        let b: usize = next().parse().unwrap();
        hld.graph[std::cmp::min(a, b)].push(std::cmp::max(a, b));
    }

    hld.dfs1(1);
    hld.dfs2(1, &mut 0);
    for i in 1..=n {
        hld.update_path(hld.parent[i], i, 1);
    }

    let m: usize = next().parse().unwrap();
    for _ in 0..n+m-1 {
        let t = next();
        let u: usize = next().parse().unwrap();
        if t == "W" {
            writeln!(stdout, "{}", hld.query_path(1, u)).unwrap();
        } else {
            let v: usize = next().parse().unwrap();
            hld.update_path(u, v, 0);
        }
    }

    print!("{stdout}");
}