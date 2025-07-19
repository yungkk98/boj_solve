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
            lazy: vec![0; tree_size],
        }
    }

    fn setup(&mut self, cur: usize, left: usize, right: usize, array: &Vec<i64>) {
        if left == right {
            self.tree[cur] = array[left];
            return;
        }

        let mid = (left + right) / 2;
        self.setup(cur * 2, left, mid, array);
        self.setup(cur * 2 + 1, mid + 1, right, array);
        self.tree[cur] = std::cmp::min(self.tree[cur * 2], self.tree[cur * 2 + 1]);
    }

    fn propagate(&mut self, cur: usize, left: usize, right: usize) {
        if self.lazy[cur] == 0 {
            return;
        }
        self.tree[cur] += self.lazy[cur];
        if left != right {
            self.lazy[cur * 2] += self.lazy[cur];
            self.lazy[cur * 2 + 1] += self.lazy[cur];
        }
        self.lazy[cur] = 0;
    }

    fn update(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize, diff: i64) {
        self.propagate(cur, left, right);
        if start > right || end < left {
            return;
        }
        if start <= left && right <= end {
            self.lazy[cur] += diff;
            self.propagate(cur, left, right);
            return;
        }

        let mid = (left + right) / 2;
        self.update(cur * 2, left, mid, start, end, diff);
        self.update(cur * 2 + 1, mid + 1, right, start, end, diff);
        self.tree[cur] = std::cmp::min(self.tree[cur * 2], self.tree[cur * 2 + 1]);
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> i64 {
        self.propagate(cur, left, right);
        if start > right || end < left {
            return std::i64::MAX;
        }
        if start <= left && right <= end {
            return self.tree[cur];
        }

        let mid = (left + right) / 2;
        return std::cmp::min(
            self.query(cur * 2, left, mid, start, end),
            self.query(cur * 2 + 1, mid + 1, right, start, end),
        )
    }
}

pub struct HLD {
    size: Vec<usize>,
    top: Vec<usize>,
    parent: Vec<usize>,
    depth: Vec<usize>,
    inn: Vec<usize>,
    out: Vec<usize>,
    graph: Vec<Vec<usize>>,
    adj_list: Vec<Vec<usize>>,
    dfs_check: Vec<bool>,
    segment_tree: SegmentTree,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            size: vec![0; n],
            top: vec![0; n],
            parent: vec![0; n],
            depth: vec![0; n],
            inn: vec![0; n],
            out: vec![0; n],
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
            self.depth[next] = self.depth[cur] + 1;
            self.parent[next] = cur;
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

    fn update(&mut self, mut a: usize, mut b: usize, diff: i64) {
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            self.segment_tree.update(1, 1, self.size[1], self.inn[self.top[a]], self.inn[a], diff);
            a = self.parent[self.top[a]];
        }

        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.segment_tree.update(1, 1, self.size[1], self.inn[a], self.inn[b], diff);
    }

    fn query(&mut self, mut a: usize, mut b: usize) -> i64 {
        let mut result: i64 = std::i64::MAX;
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

    let n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();
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

    let mut array: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        let lw: i64 = next().parse().unwrap();
        array[hld.inn[i]] = lw;
    }
    hld.segment_tree.setup(1, 1, hld.size[1], &array);

    let mut i: usize = 0;
    for _ in 0..m {
        let x: usize = next().parse().unwrap();
        let y: usize = next().parse().unwrap();
        let w: i64 = next().parse().unwrap();
        let min_weight = hld.query(x, y);
        if min_weight < w {
            break;
        } else {
            i += 1;
        }
        hld.update(x, y, -1 * w);
    }
    print!("{i}");
}