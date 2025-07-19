use std::fmt::Write;

pub struct Tree {
    size: Vec<usize>,
    dfs_check: Vec<bool>,
    adj_list: Vec<Vec<usize>>,
}

impl Tree {
    fn init(n: usize) -> Tree {
        Tree {
            size: vec![0; n],
            dfs_check: vec![false; n],
            adj_list: vec![vec![]; n],
        }
    }

    fn dfs(&mut self, cur: usize) {
        self.size[cur] = 1;
        self.dfs_check[cur] = true;
        for index in 0..self.adj_list[cur].len() {
            let next = self.adj_list[cur][index];
            if !self.dfs_check[next] {
                self.dfs_check[next] = true;
                self.dfs(next);
                self.size[cur] += self.size[next];
            }
        }
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let r: usize = next().parse().unwrap();
    let q: usize = next().parse().unwrap();
    let mut tree: Tree = Tree::init(n + 1);
    for _ in 1..n {
        let u: usize = next().parse().unwrap();
        let v: usize = next().parse().unwrap();
        tree.adj_list[u].push(v);
        tree.adj_list[v].push(u);
    }

    tree.dfs(r);
    for _ in 0..q {
        let u: usize = next().parse().unwrap();
        writeln!(stdout, "{}", tree.size[u]).unwrap();
    }
    print!("{stdout}");
}