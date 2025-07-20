use std::fmt::Write;

pub struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn init(n: usize) -> DisjointSet {
        DisjointSet {
            parent: (0..=n).collect(),
        }
    }

    fn get_parent(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            return a;
        }

        self.parent[a] = self.get_parent(self.parent[a]);
        return self.parent[a];
    }

    fn check(&mut self, a: usize, b: usize) -> (bool, usize, usize) {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        if pa == pb {
            return (false, 0, 0);
        }

        return (true, pa, pb);
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let (result, pa, pb) = self.check(a, b);
        if result {
            self.parent[pb] = pa;
        }
        return result;
    }
}

pub struct SegmentTree {
    tree: Vec<i64>,
}

impl SegmentTree {
    fn init(n: usize) -> SegmentTree {
        let tree_depth: f64 = (n as f64).log2().ceil() + 1.0_f64;
        let tree_size: usize = 2.0_f64.powf(tree_depth) as usize;
        SegmentTree {
            tree: vec![0; tree_size],
        }
    }

    fn setup(&mut self, cur: usize, left: usize, right: usize, array: &Vec<i64>, index: &Vec<usize>) {
        if left == right {
            self.tree[cur] = array[index[left]];
            return;
        }

        let mid = (left + right) / 2;
        self.setup(cur * 2, left, mid, array, index);
        self.setup(cur * 2 + 1, mid + 1, right, array, index);
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
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
        self.tree[cur] = self.tree[cur * 2] + self.tree[cur * 2 + 1];
    }

    fn query(&mut self, cur: usize, left: usize, right: usize, start: usize, end: usize) -> i64 {
        if end < left || start > right {
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
    top: Vec<usize>,
    parent: Vec<usize>,
    depth: Vec<usize>,
    inn: Vec<usize>,
    rev_inn: Vec<usize>,
    out: Vec<usize>,
    dfs_check: Vec<bool>,
    graph: Vec<Vec<usize>>,
    adj_list: Vec<Vec<usize>>,
    segment_tree: SegmentTree,
    numbers: Vec<i64>,
}

impl HLD {
    fn init(n: usize) -> HLD {
        HLD {
            size: vec![0; n],
            top: vec![0; n],
            parent: vec![0; n],
            depth: vec![0; n],
            inn: vec![0; n],
            rev_inn: vec![0; n],
            out: vec![0; n],
            dfs_check: vec![false; n],
            graph: vec![vec![]; n],
            adj_list: vec![vec![]; n],
            segment_tree: SegmentTree::init(n),
            numbers: vec![0; n],
        }
    }

    fn dfs(&mut self, cur: usize) {
        self.dfs_check[cur] = true;
        for index in 0..self.adj_list[cur].len() {
            let next = self.adj_list[cur][index];
            if self.dfs_check[next] {
                continue;
            }
            self.dfs_check[next] = true;
            self.graph[cur].push(next);
            self.parent[next] = cur;
            self.dfs(next);
        }
    }

    fn dfs1(&mut self, cur: usize) {
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

    fn update(&mut self, index: usize, value: i64) {
        let diff = value - self.numbers[index];
        self.segment_tree.update(1, 1, self.size[1], self.inn[index], diff);
        self.numbers[index] = value;
    }

    fn query(&mut self, mut a: usize, mut b: usize) -> i64 {
        let mut result: i64 = 0;
        while self.top[a] != self.top[b] {
            if self.depth[self.top[a]] < self.depth[self.top[b]] {
                std::mem::swap(&mut a, &mut b);
            }
            result += self.segment_tree.query(1, 1, self.size[1], self.inn[self.top[a]], self.inn[a]);
            a = self.parent[self.top[a]];
        }

        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        result += self.segment_tree.query(1, 1, self.size[1], self.inn[a], self.inn[b]);
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
    for i in 1..=n {
        hld.numbers[i] = next().parse().unwrap();
    }

    let mut disjoint_set: DisjointSet = DisjointSet::init(n + 1);
    let mut command_list: Vec<(u8, usize, usize)> = vec![];
    let q: usize = next().parse().unwrap();
    for _ in 0..q {
        let command_type = next().chars().collect::<Vec<_>>()[0];
        let a: usize = next().parse().unwrap();
        let b: usize = next().parse().unwrap();
        if command_type == 'e' {
            if disjoint_set.check(a, b).0 {
                command_list.push((0, a, b));
            } else {
                command_list.push((1, a, b));
            }
        } else if command_type == 'b' {
            if disjoint_set.merge(a, b) {
                hld.adj_list[a].push(b);
                hld.adj_list[b].push(a);
                command_list.push((2, a, b));
            } else {
                command_list.push((3, a, b));
            }
        } else {
            command_list.push((4, a, b))
        }
    }
    for i in 2..=n {
        if disjoint_set.get_parent(i) != 1 {
            hld.adj_list[1].push(i);
            hld.adj_list[i].push(1);
        }
    }

    hld.dfs(1);
    hld.dfs1(1);
    hld.dfs2(1, &mut 0);
    hld.segment_tree.setup(1, 1, hld.size[1], &hld.numbers, &hld.rev_inn);

    for (t, a, b) in command_list.into_iter() {
        match t {
            0 => writeln!(stdout, "impossible").unwrap(),
            1 => writeln!(stdout, "{}", hld.query(a, b)).unwrap(),
            2 => writeln!(stdout, "yes").unwrap(),
            3 => writeln!(stdout, "no").unwrap(),
            4 => hld.update(a, b as i64),
            _ => todo!(),
        }
    }
    print!("{stdout}");
}