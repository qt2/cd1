fn main() {
    let mut graph = Graph::new(5);
    graph.connect(0, 1, 8);
    graph.connect(0, 2, 2);
    graph.connect(0, 3, 12);
    graph.connect(1, 3, 9);
    graph.connect(1, 4, 24);
    graph.connect(2, 3, 4);
    graph.connect(3, 4, 18);

    let mst = boruvka(&graph);
    println!("Minimum Spanning Tree: {:?}", mst);
    println!(
        "Weight of MST: {}",
        mst.all_edges().iter().map(|(_, _, w)| w).sum::<i32>()
    );

    let mut diff = (usize::MAX, usize::MAX, usize::MAX, usize::MAX, i32::MAX);
    let mut candidate = mst.clone();

    for (u, v, original_weight) in mst.all_edges() {
        candidate.disconnect(u, v);

        let (_count, labels) = count_and_label(&candidate);

        let mut smallest = (usize::MAX, usize::MAX, i32::MAX);
        for (u2, v2, weight) in graph.all_edges() {
            if labels[u2] == labels[v2] || (u2 == u && v2 == v) {
                continue; // Skip edges within the same component
            }

            if weight < smallest.2 {
                smallest = (u2, v2, weight);
            }
        }

        if smallest.2 - original_weight < diff.4 {
            diff = (u, v, smallest.0, smallest.1, smallest.2 - original_weight);
        }

        candidate.connect(u, v, original_weight);
    }

    candidate.disconnect(diff.0, diff.1);
    candidate.connect(diff.2, diff.3, graph.get_weight(diff.2, diff.3).unwrap());
    println!("Second MST: {:?}", candidate);
    println!(
        "Weight of Second MST: {}",
        candidate.all_edges().iter().map(|(_, _, w)| w).sum::<i32>()
    );
}

#[derive(Debug, Clone)]
struct Graph {
    links: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            links: vec![Vec::new(); size],
        }
    }

    fn connect(&mut self, u: usize, v: usize, weight: i32) {
        if self.links[u].iter().any(|&(x, _)| x == v) {
            // Edge already exists, skip adding
            return;
        }

        self.links[u].push((v, weight));
        self.links[v].push((u, weight));
    }

    fn disconnect(&mut self, u: usize, v: usize) {
        self.links[u].retain(|&(x, _)| x != v);
        self.links[v].retain(|&(x, _)| x != u);
    }

    fn neighbors(&self, u: usize) -> &[(usize, i32)] {
        &self.links[u]
    }

    fn get_weight(&self, u: usize, v: usize) -> Option<i32> {
        self.links[u]
            .iter()
            .find(|&&(x, _)| x == v)
            .map(|&(_, weight)| weight)
    }

    fn all_edges(&self) -> Vec<(usize, usize, i32)> {
        let mut edges = Vec::new();
        for u in 0..self.links.len() {
            for &(v, weight) in &self.links[u] {
                if u < v {
                    // Avoid duplicates
                    edges.push((u, v, weight));
                }
            }
        }
        edges
    }
}

fn boruvka(graph: &Graph) -> Graph {
    let mut mst = Graph::new(graph.links.len());

    loop {
        let (count, labels) = count_and_label(&mst);
        dbg!(count, &labels);
        if count <= 1 {
            break; // All vertices are connected
        }

        let mut safe: Vec<Option<(usize, i32)>> = vec![None; count];
        for (u, v, weight) in graph.all_edges() {
            let label_u = labels[u];
            let label_v = labels[v];

            if label_u != label_v {
                if safe[label_u].is_none_or(|(_, w)| w > weight) {
                    safe[label_u] = Some((v, weight));
                }
                if safe[label_v].is_none_or(|(_, w)| w > weight) {
                    safe[label_v] = Some((u, weight));
                }
            }
        }
        for (i, edge) in safe.iter().enumerate() {
            if let Some((v, weight)) = edge {
                mst.connect(i, *v, *weight);
            }
        }
    }

    mst
}

fn count_and_label(graph: &Graph) -> (usize, Vec<usize>) {
    let mut visited = vec![false; graph.links.len()];
    let mut label = vec![0; graph.links.len()];
    let mut count = 0;

    for u in 0..graph.links.len() {
        if !visited[u] {
            dfs(graph, u, count, &mut visited, &mut label);
            count += 1;
        }
    }

    (count, label)
}

fn dfs(graph: &Graph, u: usize, count: usize, visited: &mut Vec<bool>, label: &mut Vec<usize>) {
    visited[u] = true;
    label[u] = count;

    for &(v, _) in graph.neighbors(u) {
        if !visited[v] {
            dfs(graph, v, count, visited, label);
        }
    }
}
