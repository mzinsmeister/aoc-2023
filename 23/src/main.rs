use std::{fs::read_to_string, collections::{VecDeque, HashMap}};

use petgraph::{graph::DiGraph, graph::NodeIndex};

// We take out all of the single way paths and connect junctions in a graph
fn construct_graph_bfs(grid: &Vec<Vec<char>>) -> (NodeIndex, NodeIndex, DiGraph<(usize, usize), usize>) {
    let mut graph: DiGraph<(usize,usize), usize> = DiGraph::new();
    let mut node_map = HashMap::new();
    let start_pos = (0, 1);
    let end_pos = (grid.len() - 1, grid[0].len() - 2);
    let start_node = graph.add_node(start_pos);
    node_map.insert(start_pos, start_node);
    let end_node = graph.add_node(end_pos);
    node_map.insert( end_pos, end_node);
    let mut queue = VecDeque::new();
    queue.push_back((start_node, (start_pos.0 + 1, start_pos.1)));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start_pos.0][start_pos.1] = true;
    while let Some((init_node, (row, col))) = queue.pop_front() {
        let mut path_length = 1;
        let mut current_row = row;
        let mut current_col = col;
        let mut forward_possible = true;
        let mut backward_possible = true;
        loop {
            visited[current_row][current_col] = true;
            let mut neighbors = vec![];
            if current_row > 0 && grid[current_row - 1][current_col] != '#' {
                let mut forward_possible = forward_possible;
                let mut backward_possible = backward_possible;
                if grid[current_row][current_col] == '^'  {
                    backward_possible = false;
                } else if grid[current_row][current_col] == 'v' {
                    forward_possible = false;
                }
                neighbors.push(((forward_possible, backward_possible), (current_row - 1, current_col)));
            }
            if current_row < grid.len() - 1 && grid[current_row + 1][current_col] != '#' {
                let mut forward_possible = forward_possible;
                let mut backward_possible = backward_possible;
                if grid[current_row][current_col] == '^' {
                    forward_possible = false;
                } else if grid[current_row][current_col] == 'v' {
                    backward_possible = false;
                }
                neighbors.push(((forward_possible, backward_possible), (current_row + 1, current_col)));
            }
            if current_col > 0 && grid[current_row][current_col - 1] != '#' {
                let mut forward_possible = forward_possible;
                let mut backward_possible = backward_possible;
                if grid[current_row][current_col] == '<' {
                    backward_possible = false;
                } else if grid[current_row][current_col] == '>' {
                    forward_possible = false;
                }
                neighbors.push(((forward_possible, backward_possible), (current_row, current_col - 1)));
            }
            if current_col < grid[0].len() - 1 && grid[current_row][current_col + 1] != '#' {
                let mut forward_possible = forward_possible;
                let mut backward_possible = backward_possible;
                if grid[current_row][current_col] == '<' {
                    forward_possible = false;
                } else if grid[current_row][current_col] == '>' {
                    backward_possible = false;
                }
                neighbors.push(((forward_possible, backward_possible), (current_row, current_col + 1)));
            }
            let neighbor_node = neighbors.iter().filter_map(|(_, (nrow, ncol))| node_map.get(&(*nrow, *ncol))).find(|n| **n != init_node);
            if let Some(neighbor_node) = neighbor_node {
                if forward_possible {
                    graph.add_edge(init_node, *neighbor_node, path_length + 1);
                }
                if backward_possible {
                    graph.add_edge(*neighbor_node, init_node, path_length + 1);
                }
                break;
            }
            let unvisited_neighbors = neighbors.iter().filter(|(_, (nrow, ncol))| !visited[*nrow][*ncol]).collect::<Vec<_>>();
            if unvisited_neighbors.len() == 1 {
                current_row = unvisited_neighbors[0].1.0;
                current_col = unvisited_neighbors[0].1.1;
                forward_possible = unvisited_neighbors[0].0.0;
                backward_possible = unvisited_neighbors[0].0.1;
                path_length += 1;
                visited[current_row][current_col] = true;
            } else {
                let node = graph.add_node((current_row, current_col));
                node_map.insert((current_row, current_col), node);
                if forward_possible {
                    graph.add_edge(init_node, node, path_length);
                }
                if backward_possible {
                    graph.add_edge(node, init_node, path_length);
                }
                for neighbor in unvisited_neighbors {
                    queue.push_back((node, neighbor.1));
                }
                break;
            }
        }
    }
    (start_node, end_node, graph)
}

fn construct_graph_bfs2(grid: &Vec<Vec<char>>) -> (NodeIndex, NodeIndex, DiGraph<(usize, usize), usize>) {
    let mut graph: DiGraph<(usize,usize), usize> = DiGraph::new();
    let mut node_map = HashMap::new();
    let start_pos = (0, 1);
    let end_pos = (grid.len() - 1, grid[0].len() - 2);
    let start_node = graph.add_node(start_pos);
    node_map.insert(start_pos, start_node);
    let end_node = graph.add_node(end_pos);
    node_map.insert( end_pos, end_node);
    let mut queue = VecDeque::new();
    queue.push_back((start_node, (start_pos.0 + 1, start_pos.1)));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start_pos.0][start_pos.1] = true;
    while let Some((init_node, (row, col))) = queue.pop_front() {
        let mut path_length = 1;
        let mut current_row = row;
        let mut current_col = col;
        loop {
            visited[current_row][current_col] = true;
            let mut neighbors = vec![];
            if current_row > 0 && grid[current_row - 1][current_col] != '#' {
                neighbors.push((current_row - 1, current_col));
            }
            if current_row < grid.len() - 1 && grid[current_row + 1][current_col] != '#' {
                neighbors.push((current_row + 1, current_col));
            }
            if current_col > 0 && grid[current_row][current_col - 1] != '#' {
                neighbors.push((current_row, current_col - 1));
            }
            if current_col < grid[0].len() - 1 && grid[current_row][current_col + 1] != '#' {
                neighbors.push((current_row, current_col + 1));
            }
            let neighbor_node = neighbors.iter().filter_map(|(nrow, ncol)| node_map.get(&(*nrow, *ncol))).find(|n| **n != init_node);
            if let Some(neighbor_node) = neighbor_node {
                graph.add_edge(init_node, *neighbor_node, path_length + 1);
                graph.add_edge(*neighbor_node, init_node, path_length + 1);
                break;
            }
            let unvisited_neighbors = neighbors.iter().filter(|(nrow, ncol)| !visited[*nrow][*ncol]).collect::<Vec<_>>();
            if unvisited_neighbors.len() == 1 {
                current_row = unvisited_neighbors[0].0;
                current_col = unvisited_neighbors[0].1;
                path_length += 1;
                visited[current_row][current_col] = true;
            } else {
                let node = graph.add_node((current_row, current_col));
                node_map.insert((current_row, current_col), node);
                graph.add_edge(init_node, node, path_length);
                graph.add_edge(node, init_node, path_length);
                for neighbor in unvisited_neighbors {
                    queue.push_back((node, *neighbor));
                }
                break;
            }
        }
    }
    (start_node, end_node, graph)
}

fn get_len(graph: &DiGraph<(usize, usize), usize>, path: Vec<NodeIndex>) -> usize {
    let mut len = 0;
    for i in 0..path.len() - 1 {
        let edge = graph.find_edge(path[i], path[i + 1]).unwrap();
        len += graph[edge];
    }
    len
}

fn main() {
    let input = read_to_string("input.txt").unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

    let (start_node, end_node, graph) = construct_graph_bfs(&input);
    
    // Pretty sure we might be able to do something like a modified dijkstra's algorithm here
    // But in release mode this brute force thing it's reasonably fast (5s on my laptop for both parts)

    let max = petgraph::algo::all_simple_paths(&graph, start_node, end_node, 0, None).map(|p: Vec<NodeIndex>| get_len(&graph, p)).max().unwrap();

    println!("result1: {}", max);

    let (start_node, end_node, graph) = construct_graph_bfs2(&input);
    
    let max = petgraph::algo::all_simple_paths(&graph, start_node, end_node, 0, None).map(|p: Vec<NodeIndex>| get_len(&graph, p)).max().unwrap();

    println!("result2: {}", max);
}
