use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<i64>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1_f64;
        let tree_size: usize = 2_f64.powf(tree_depth) as usize;

        SegmentTree {
            tree: vec![0; tree_size],
        }
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, index: usize, diff: i64) {
        if index < left || index > right {
            return;
        }

        if left == right {
            self.tree[cur] += diff;
            return;
        }

        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, index, diff);
        self.update(cur * 2 + 1, mid + 1, right, index, diff);
        self.tree[cur] = std::cmp::max(self.tree[cur * 2], self.tree[cur * 2 + 1]);
    }

    fn query(&self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> i64 {
        if start > right || end < left {
            return 0;
        }
        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return std::cmp::max(self.query(cur * 2, left, mid, start, end), self.query(cur * 2 + 1, mid + 1, right, start, end));
    }
}

pub struct HLD {
    n: usize,
    size: Vec<usize>,
    depth: Vec<usize>,
    parent: Vec<usize>,
    top: Vec<usize>,
    inn: Vec<usize>,
    out: Vec<usize>,
    graph: Vec<Vec<usize>>,
    cost: Vec<i64>,
    adj_list: Vec<Vec<(usize, i64)>>,
    dfs_check: Vec<bool>,
    segment_tree: SegmentTree,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            n: n,
            size: vec![0; n],
            depth: vec![0; n],
            parent: vec![0; n],
            top: vec![0; n],
            inn: vec![0; n],
            out: vec![0; n],
            graph: vec![vec![]; n],
            cost: vec![0; n],
            adj_list: vec![vec![]; n],
            dfs_check: vec![false; n],
            segment_tree: SegmentTree::init(n),
        }
    }

    fn dfs(&mut self, node: usize) {
        self.dfs_check[node] = true;
        for (i, cost) in self.adj_list[node].clone().into_iter() {
            if self.dfs_check[i] {
                continue;
            }
            self.dfs_check[node] = true;
            self.graph[node].push(i);
            self.cost[i] = cost;
            self.dfs(i);
        }
    }

    fn dfs1(&mut self, node: usize) {
        self.size[node] = 1;
        for index in 0..self.graph[node].len() {
            let i = self.graph[node][index];
            self.depth[i] = self.depth[node] + 1;
            self.parent[i] = node;
            self.dfs1(i);
            self.size[node] += self.size[i];
            if self.size[i] > self.size[self.graph[node][0]] {
                self.graph[node][index] = self.graph[node][0];
                self.graph[node][0] = i;
            }
        }
    }

    fn dfs2(&mut self, node: usize, prev: &mut usize) {
        *prev += 1;
        self.inn[node] = *prev;
        for index in 0..self.graph[node].len() {
            let i = self.graph[node][index];
            if i == self.graph[node][0] {
                self.top[i] = self.top[node];
            } else {
                self.top[i] = i;
            }
            self.dfs2(i, prev);
        }
        self.out[node] = *prev;
    }

    fn setup(&mut self) {
        for i in 1..self.n {
            self.segment_tree.update(1, 1, self.n, self.inn[i], self.cost[i]);
        }
    }

    fn update(&mut self, index: usize, value: i64) {
        self.segment_tree.update(1, 1, self.n, self.inn[index], value - self.cost[index]);
        self.cost[index] = value;
    }

    fn query(&self, mut a: usize, mut b: usize) -> i64 {
        let mut result: i64 = 0;
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            let start = self.top[a];
            result = std::cmp::max(result, self.segment_tree.query(1, 1, self.n, self.inn[start], self.inn[a]));
            a = self.parent[start];
        }
        
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        result = std::cmp::max(result, self.segment_tree.query(1, 1, self.n, self.inn[a] + 1, self.inn[b]));
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
    let mut edges: Vec<(usize, usize, i64)> = vec![];
    for _ in 1..n {
        let (s, e, w): (usize, usize, i64) = (next().parse().unwrap(), next().parse().unwrap(), next().parse().unwrap());
        hld.adj_list[s].push((e, w));
        hld.adj_list[e].push((s, w));
        edges.push((s, e, w));
    }
    hld.dfs(1);
    hld.dfs1(1);
    hld.dfs2(1, &mut 0);
    hld.setup();

    let m: usize = next().parse().unwrap();
    for _ in 0..m {
        let t: usize = next().parse().unwrap();
        if t == 1 {
            let (i, c): (usize, i64) = (next().parse().unwrap(), next().parse().unwrap());
            let (s, e, _) = edges[i - 1];
            if hld.parent[e] == s {
                hld.update(e, c);
            } else {
                hld.update(s, c);
            }
        } else {
            let (u, v): (usize, usize) = (next().parse().unwrap(), next().parse().unwrap());
            writeln!(stdout, "{}", hld.query(u, v)).unwrap();
        }
    }

    print!("{stdout}");
}