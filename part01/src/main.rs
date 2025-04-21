use std::{mem, time::Instant};

use ndarray::Array2;
use rustc_hash::FxHashSet;

fn main() {
    compare_structures(100, 1000);
    compare_structures(1000, 10000);
    compare_structures(10000, 100000);
    compare_structures(100000, 1000000);
    println!("Finished comparing structures.");
}

fn compare_structures(v: usize, e: usize) {
    println!("Vertices: {}, Edges: {}", v, e);

    let mut adjacency_list = AdjacencyList::new(v, e);
    let mut fast_adjacency_list = FastAdjacencyList::new(v, e);
    let mut adjacency_matrix = AdjacencyMatrix::new(v, e);

    for _ in 0..e {
        let u = fastrand::usize(0..v);
        let v = fastrand::usize(0..v);
        if u == v {
            continue;
        }
        adjacency_list.connect(u, v);
        fast_adjacency_list.connect(u, v);
        adjacency_matrix.connect(u, v);
    }

    println!(
        "AdjacencyList: {:?} KB",
        adjacency_list.memory_usage() / 1024
    );
    println!(
        "FastAdjacencyList: {:?} KB",
        fast_adjacency_list.memory_usage() / 1024
    );
    println!(
        "AdjacencyMatrix: {:?} KB",
        adjacency_matrix.memory_usage() / 1024
    );

    let tests: Vec<_> = (0..100000)
        .map(|_| {
            let u = fastrand::usize(0..v);
            let v = fastrand::usize(0..v);
            (u, v)
        })
        .collect();

    let start = Instant::now();
    for (u, v) in &tests {
        adjacency_list.is_connected(*u, *v);
    }
    let duration = start.elapsed();
    println!("AdjacencyList: {:?}μs", duration.as_micros());

    let start = Instant::now();
    for (u, v) in &tests {
        fast_adjacency_list.is_connected(*u, *v);
    }
    let duration = start.elapsed();
    println!("FastAdjacencyList: {:?}μs", duration.as_micros());

    let start = Instant::now();
    for (u, v) in &tests {
        adjacency_matrix.is_connected(*u, *v);
    }
    let duration = start.elapsed();
    println!("AdjacencyMatrix: {:?}μs", duration.as_micros());
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

    fn memory_usage(&self) -> usize {
        let mut size = 0;
        for neighbors in &self.data {
            size += mem::size_of::<usize>() * neighbors.len();
        }
        size + mem::size_of::<Vec<usize>>() * self.data.len() + mem::size_of::<Vec<Vec<usize>>>()
    }
}

pub struct FastAdjacencyList {
    data: Vec<FxHashSet<usize>>,
}

impl UndirectedGraph for FastAdjacencyList {
    fn new(v: usize, _e: usize) -> Self {
        FastAdjacencyList {
            data: vec![FxHashSet::default(); v],
        }
    }

    fn connect(&mut self, u: usize, v: usize) {
        self.data[u].insert(v);
        self.data[v].insert(u);
    }

    fn disconnect(&mut self, u: usize, v: usize) {
        self.data[u].remove(&v);
        self.data[v].remove(&u);
    }

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.data[u].contains(&v)
    }

    fn list_neighbors(&self, u: usize) -> Vec<usize> {
        self.data[u].iter().copied().collect()
    }

    fn memory_usage(&self) -> usize {
        let mut size = 0;
        for neighbors in &self.data {
            size += mem::size_of::<usize>() * neighbors.len();
        }
        size + mem::size_of::<FxHashSet<usize>>() * self.data.len()
            + mem::size_of::<Vec<FxHashSet<usize>>>()
    }
}

pub struct AdjacencyMatrix {
    data: Array2<u8>,
}

impl UndirectedGraph for AdjacencyMatrix {
    fn new(v: usize, _e: usize) -> Self {
        AdjacencyMatrix {
            data: Array2::zeros((v, v)),
        }
    }

    fn connect(&mut self, u: usize, v: usize) {
        self.data[[u, v]] = 1;
        self.data[[v, u]] = 1;
    }

    fn disconnect(&mut self, u: usize, v: usize) {
        self.data[[u, v]] = 0;
        self.data[[v, u]] = 0;
    }

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.data[[u, v]] == 1
    }

    fn list_neighbors(&self, u: usize) -> Vec<usize> {
        (0..self.data.shape()[1])
            .filter(|&v| self.is_connected(u, v))
            .collect()
    }

    fn memory_usage(&self) -> usize {
        mem::size_of::<u8>() * self.data.len()
    }
}

pub trait UndirectedGraph {
    fn new(v: usize, e: usize) -> Self;

    fn connect(&mut self, u: usize, v: usize);

    fn disconnect(&mut self, u: usize, v: usize);

    fn is_connected(&self, u: usize, v: usize) -> bool;

    fn list_neighbors(&self, u: usize) -> Vec<usize>;

    fn list_all_edges(&self) -> Vec<(usize, usize)> {
        unimplemented!()
    }

    fn memory_usage(&self) -> usize {
        unimplemented!()
    }
}
