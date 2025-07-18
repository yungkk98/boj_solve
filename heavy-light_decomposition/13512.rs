use std::fmt::Write;

pub struct SegmentTree {
    tree: Vec<usize>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;
        SegmentTree {
            tree: vec![std::usize::MAX; tree_size],
        }
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, index: usize, value: usize) {
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
        self.tree[cur] = std::cmp::min(self.tree[cur * 2], self.tree[cur * 2 + 1]);
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> usize {
        if start > right || end < left {
            return std::usize::MAX;
        }
        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return std::cmp::min(
            self.query(cur * 2, left, mid, start, end),
            self.query(cur * 2 + 1, mid + 1, right, start, end)
        )
    }
}

pub struct HLD {
    size: Vec<usize>,
    parent: Vec<usize>,
    top: Vec<usize>,
    depth: Vec<usize>,
    inn: Vec<usize>,
    rev_inn: Vec<usize>,
    out: Vec<usize>,
    is_white: Vec<bool>,
    graph: Vec<Vec<usize>>,
    adj_list: Vec<Vec<usize>>,
    dfs_check: Vec<bool>,
    segment_tree: SegmentTree,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            size: vec![0; n],
            parent: vec![0; n],
            top: vec![0; n],
            depth: vec![0; n],
            inn: vec![0; n],
            rev_inn: vec![0; n],
            out: vec![0; n],
            is_white: vec![true; n],
            graph: vec![vec![]; n],
            adj_list: vec![vec![]; n],
            dfs_check: vec![false; n],
            segment_tree: SegmentTree::init(n),
        }
    }

    fn dfs(&mut self, cur: usize) {
        self.dfs_check[cur] = true;
        for index in 0..self.adj_list[cur].len() {
            let next = self.adj_list[cur][index];
            if !self.dfs_check[next] {
                self.dfs_check[next] = true;
                self.graph[cur].push(next);
                self.dfs(next);
            }
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
                self.graph[cur].swap(0, index);
            }
        }
    }

    fn dfs2(&mut self, cur: usize, pv: &mut usize) {
        *pv += 1;
        self.inn[cur] = *pv;
        self.rev_inn[*pv] = cur;
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

    fn update(&mut self, index: usize) {
        if self.is_white[index] {
            self.segment_tree.update(1, 1, self.size[1], self.inn[index], self.inn[index]);
        } else {
            self.segment_tree.update(1, 1, self.size[1], self.inn[index], std::usize::MAX);
        }
        self.is_white[index] = !self.is_white[index];
    }

    fn query_path(&mut self, mut a: usize, mut b: usize) -> usize {
        let mut result: usize = std::usize::MAX;
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }

            result = std::cmp::min(result, self.segment_tree.query(1, 1, self.size[1], self.inn[self.top[a]], self.inn[a]));
            a = self.parent[self.top[a]];
        }

        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        result = std::cmp::min(result, self.segment_tree.query(1, 1, self.size[1], self.inn[a], self.inn[b]));
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
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        hld.adj_list[u].push(v);
        hld.adj_list[v].push(u);
    }

    hld.dfs(1);
    hld.dfs1(1);
    hld.dfs2(1, &mut 0);

    let m: usize = next().parse().unwrap();
    for _ in 0..m {
        let t: usize = next().parse().unwrap();
        let i: usize = next().parse().unwrap();
        if t == 1 {
            hld.update(i);
        } else {
            let result = hld.query_path(1, i);
            if result == std::usize::MAX {
                writeln!(stdout, "-1").unwrap();
            } else {
                writeln!(stdout, "{}", hld.rev_inn[result]).unwrap();
            }
        }
    }

    print!("{stdout}");
}