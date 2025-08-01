use std::collections::VecDeque;

#[derive(Clone)]
pub struct Edge {
    end: usize,
    capacity: i64,
    flow: i64,
    cost: i64,
    inverse_edge_index: usize,
}

impl Edge {
    fn init(end: usize, capacity: i64, cost: i64, inverse_edge_index: usize) -> Edge {
        Edge {
            end: end,
            capacity: capacity,
            flow: 0,
            cost: cost,
            inverse_edge_index: inverse_edge_index,
        }
    }
}

pub struct MCMF {
    start: usize,
    end: usize,
    n: usize,
    adj_list: Vec<Vec<Edge>>,
}

impl MCMF {
    fn init(n: usize, start: usize, end: usize) -> MCMF {
        MCMF {
            n: n,
            start: start,
            end: end,
            adj_list: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, start: usize, end: usize, capacity: i64, cost: i64) {
        let edge_index = self.adj_list[start].len();
        let inverse_edge_index = self.adj_list[end].len();
        self.adj_list[start].push(Edge::init(end, capacity, cost, inverse_edge_index));
        self.adj_list[end].push(Edge::init(start, 0, -1 * cost, edge_index));
    }

    fn solve(&mut self) {
        let mut min_cost: i64 = 0;
        let mut max_flow: i64 = 0;

        let mut prev: Vec<usize> = vec![std::usize::MAX; self.n];
        let mut cost: Vec<i64> = vec![std::i64::MAX; self.n];
        let mut using: Vec<bool> = vec![false; self.n];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut edge_index: Vec<usize> = vec![std::usize::MAX; self.n];
        loop {
            prev.fill(std::usize::MAX);
            cost.fill(std::i64::MAX);
            using.fill(false);
            queue.clear();
            edge_index.fill(std::usize::MAX);

            // SPFA
            cost[self.start] = 0;
            using[self.start] = true;
            queue.push_back(self.start);
            while !queue.is_empty() {
                let cur = queue.pop_front().unwrap();
                using[cur] = false;
                for index in 0..self.adj_list[cur].len() {
                    if self.adj_list[cur][index].capacity > self.adj_list[cur][index].flow &&
                        cost[cur] + self.adj_list[cur][index].cost < cost[self.adj_list[cur][index].end] {
                        prev[self.adj_list[cur][index].end] = cur;
                        cost[self.adj_list[cur][index].end] = cost[cur] + self.adj_list[cur][index].cost;
                        edge_index[self.adj_list[cur][index].end] = index;
                        if !using[self.adj_list[cur][index].end] {
                            using[self.adj_list[cur][index].end] = true;
                            queue.push_back(self.adj_list[cur][index].end);
                        }
                    }
                }
            }

            if prev[self.end] == std::usize::MAX {
                break;
            }

            // update flow
            let mut cur = self.end;
            while cur != self.start {
                let iei = self.adj_list[prev[cur]][edge_index[cur]].inverse_edge_index;
                self.adj_list[prev[cur]][edge_index[cur]].flow += 1;
                self.adj_list[cur][iei].flow -=1;
                cur = prev[cur];
            }

            min_cost += cost[self.end];
            max_flow += 1;
        }

        print!("{}\n{}", max_flow, min_cost *  -1);
    }
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    
    let n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();

    // Start Node 1
    // Employee 2 ~ n + 1
    // Work n + 2 ~ n + m + 1
    // End n + m + 2
    let mut mcmf: MCMF = MCMF::init(n + m + 3, 1, n + m + 2);
    for i in 2..=(n + 1) {
        mcmf.add_edge(mcmf.start, i, 1, 0);
    }
    for i in (n + 2)..=(n + m + 1) {
        mcmf.add_edge(i, mcmf.end, 1, 0);
    }

    for i in 2..=(n + 1) {
        let p: usize = next().parse().unwrap();
        for _ in 0..p {
            let w: usize = next().parse().unwrap();
            let c: i64 = next().parse().unwrap();
            mcmf.add_edge(i, w + n + 1, 1, -1 * c);
        }
    }
    mcmf.solve();
}