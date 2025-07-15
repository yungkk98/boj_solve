use std::fmt::Write;
use std::collections::VecDeque;

fn get_level(n: usize, level: &mut Vec<usize>, used: &Vec<bool>, adj_list: &Vec<Vec<usize>>, col_connect: &Vec<usize>) {
    let mut queue: VecDeque<usize> = VecDeque::new();
    level.fill(std::usize::MAX);
    for i in 1..=n {
        if !used[i] {
            level[i] = 0;
            queue.push_back(i);
        }
    }
    
    while !queue.is_empty() {
        let now = queue.pop_front().unwrap();
        for next in &adj_list[now] {
            if col_connect[*next] != std::usize::MAX && level[col_connect[*next]] == std::usize::MAX {
                level[col_connect[*next]] = level[now] + 1;
                queue.push_back(col_connect[*next]);
            }
        }
    }
}

fn dfs(row: usize, n: usize, level: &Vec<usize>, used: &mut Vec<bool>, adj_list: &Vec<Vec<usize>>, row_connect: &mut Vec<usize>, col_connect: &mut Vec<usize>) -> bool {
    for col in &adj_list[row] {
        if col_connect[*col] == std::usize::MAX || level[col_connect[*col]] == level[row] + 1 && dfs(col_connect[*col], n, level, used, adj_list, row_connect, col_connect) {
            used[row] = true;
            row_connect[row] = *col;
            col_connect[*col] = row;
            return true;
        }
    }
    return false;
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let r: usize = next().parse().unwrap();
    let c: usize = next().parse().unwrap();
    let n: usize = next().parse().unwrap();

    let mut chess_board: Vec<Vec<bool>> = vec![vec![true; c + 1]; r + 1];
    for _ in 0..n {
        let (y, x): (usize, usize) = (next().parse().unwrap(), next().parse().unwrap());
        chess_board[y][x] = false;
    }

    // create node
    let mut level: Vec<usize> = vec![std::usize::MAX; r + 1];
    let mut match_row: Vec<usize> = vec![std::usize::MAX; r + 1];
    let mut match_col: Vec<usize> = vec![std::usize::MAX; c + 1];
    let mut used: Vec<bool> = vec![false; r + 1];

    // create edge
    let mut edges: Vec<Vec<usize>> = vec![vec![]; r + 1];
    for i in 1..=r {
        for j in 1..=c {
            if chess_board[i][j] {
                edges[i].push(j);
            }
        }
    }

    let mut max_match: i64 = 0;
    loop {
        get_level(r, &mut level, &used, &edges, &match_col);
        let mut flow: i64 = 0;
        for i in 1..=r {
            if !used[i] && dfs(i, r, &level, &mut used, &edges, &mut match_row, &mut match_col) {
                flow += 1;
            }
        }
        if flow == 0 {
            break;
        }
        max_match += flow;
    }

    print!("{max_match}");
}