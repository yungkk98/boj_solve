pub struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn init(n: usize) -> DisjointSet {
        DisjointSet {
            parent: (0..n).collect(),
        }
    }

    fn get_parent(&mut self, p: usize) -> usize {
        if p == self.parent[p] {
            return p;
        }

        self.parent[p] = self.get_parent(self.parent[p]);
        return self.parent[p];
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        if pa == pb {
            return false;
        }
        
        self.parent[pb] = pa;
        return true;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();
    let mut disjoint_set: DisjointSet = DisjointSet::init(n);
    for i in 1..=m {
        let a: usize = next().parse().unwrap();
        let b: usize = next().parse().unwrap();
        if !disjoint_set.merge(a, b) {
            println!("{i}");
            return;
        }
    }
    println!("0");
}