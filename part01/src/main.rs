use std::collections::HashMap;

fn main() {}

fn compare_structures(v: usize, e: usize) {
    let adjacency_list = ();
    let fast_adjacency_list = ();
}

pub struct AdjacencyList {
    data: Vec<Vec<usize>>,
}

impl UndirectedGraph for AdjacencyList {
    fn new(v: usize, _e: usize) -> Self {
        AdjacencyList {
            data: vec![vec![]; v],
        }
    }

    fn connect(&mut self, u: usize, v: usize) {
        self.data[u].push(v);
        self.data[v].push(u);
    }

    fn disconnect(&mut self, u: usize, v: usize) {
        self.data[u].retain(|&x| x != v);
        self.data[v].retain(|&x| x != u);
    }

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.data[u].contains(&v)
    }

    fn list_neighbors(&self, u: usize) -> Vec<usize> {
        self.data[u].clone()
    }

    fn list_all_edges(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(u, neighbors)| neighbors.iter().map(move |&v| (u, v)))
            .collect()
    }
}

pub struct FastAdjacencyList {}

pub struct AdjacencyMatrix {}

pub trait UndirectedGraph {
    fn new(v: usize, e: usize) -> Self;

    fn connect(&mut self, u: usize, v: usize);

    fn disconnect(&mut self, u: usize, v: usize);

    fn is_connected(&self, u: usize, v: usize) -> bool;

    fn list_neighbors(&self, u: usize) -> Vec<usize>;

    fn list_all_edges(&self) -> Vec<(usize, usize)>;
}
