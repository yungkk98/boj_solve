use std::collections::VecDeque;

#[derive(Clone)]
pub struct Edge {
    end: usize,
    flow: i64,
    capacity: i64,
    cost: i64,
    inverse_edge_index: usize,
}

impl Edge {
    fn init(end: usize, flow: i64, capacity: i64, cost: i64, inverse_edge_index: usize) -> Edge {
        Edge {
            end: end,
            flow: flow,
            capacity: capacity,
            cost: cost,
            inverse_edge_index: inverse_edge_index,
        }
    }
}

pub struct MCMF {
    n: usize,
    start: usize,
    end: usize,
    edge_list: Vec<Vec<Edge>>,
}

impl MCMF {
    // Node 1 : start
    // Node 2 ~ n + 1 : student
    // Node n + 2 ~ 2n + 1 : problem
    // Node 2n + 2 : end

    fn init(n: usize) -> MCMF {
        MCMF {
            n: n,
            start: 1,
            end: 2 * n + 2,
            edge_list: vec![vec![]; 2 * n + 3],
        }
    }

    fn add_edge(&mut self, start: usize, end: usize, capacity: i64, cost: i64) {
        let edge_index: usize = self.edge_list[start].len();
        let inverse_edge_index: usize = self.edge_list[end].len();
        self.edge_list[start].push(Edge::init(end, 0, capacity, cost, inverse_edge_index));
        self.edge_list[end].push(Edge::init(start, 0, 0, -1 * cost, edge_index));
    }

    fn build_base(&mut self) {
        for student in 2..=(self.n + 1) {
            self.add_edge(self.start, student, 1, 0);
        }
        for problem in (self.n + 2)..=(2 * self.n + 1) {
            self.add_edge(problem, self.end, 1, 0);
        }
    }

    fn solve(&mut self) -> i64 {
        let mut result: i64 = 0;

        let mut prev: Vec<usize> = vec![std::usize::MAX; 2 * self.n + 3];
        let mut prev_edge_index: Vec<usize> = vec![std::usize::MAX; 2 * self.n + 3];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut using: Vec<bool> = vec![false; 2 * self.n + 3];
        let mut cost: Vec<i64> = vec![std::i64::MAX; 2 * self.n + 3];

        loop {
            // SPFA
            prev.fill(std::usize::MAX);
            prev_edge_index.fill(std::usize::MAX);
            queue.clear();
            using.fill(false);
            cost.fill(std::i64::MAX);

            queue.push_back(self.start);
            using[self.start] = true;
            cost[self.start] = 0;
            while !queue.is_empty() {
                let cur = queue.pop_front().unwrap();
                using[cur] = false;
                for index in 0..self.edge_list[cur].len() {
                    if self.edge_list[cur][index].capacity - self.edge_list[cur][index].flow > 0 &&
                        self.edge_list[cur][index].cost + cost[cur] < cost[self.edge_list[cur][index].end] {
                            cost[self.edge_list[cur][index].end] = self.edge_list[cur][index].cost + cost[cur];
                            prev[self.edge_list[cur][index].end] = cur;
                            prev_edge_index[self.edge_list[cur][index].end] = index;
                            if !using[self.edge_list[cur][index].end] {
                                using[self.edge_list[cur][index].end] = true;
                                queue.push_back(self.edge_list[cur][index].end);
                            }
                        }
                }
            }

            if cost[self.end] == std::i64::MAX {
                break;
            }

            result += cost[self.end];
            let mut cur: usize = self.end;
            while cur != self.start {
                let p = prev[cur];
                let iei = self.edge_list[p][prev_edge_index[cur]].inverse_edge_index;
                self.edge_list[p][prev_edge_index[cur]].flow += 1;
                self.edge_list[cur][iei].flow -= 1;
                cur = p;
            }
        }

        return result;
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let mut mcmf: MCMF =MCMF::init(n);
    mcmf.build_base();
    for student in 1..=n {
        for problem in 1..=n {
            let cost: i64 = next().parse().unwrap();
            mcmf.add_edge(student + 1, problem + n + 1, 1, cost);
        }
    }

    print!("{}", mcmf.solve());
}