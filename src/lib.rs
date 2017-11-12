extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// Structure to represent a Graph Coloring Problem.
#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Vec<usize>>
}

impl Graph {
    /// Creates an empty graph with `n` nodes and `0` edges.
    pub fn new(n: usize) -> Self {
        Graph {
            nodes: vec![Vec::new(); n]
        }
    }
    /// Adds an edge between the nodes `a` and `b`.
    pub fn add_edge(&mut self, a: usize, b: usize) {
        assert!(a < self.nodes.len());
        assert!(b < self.nodes.len());
        match (self.nodes[a].binary_search(&b),
               self.nodes[b].binary_search(&a)) {
            (Err(ia), Err(ib)) => {
                self.nodes[a].insert(ia, b);
                self.nodes[b].insert(ib, a);
            }
            _ => unreachable!()
        }
    }
    /// Attempts to solve the graph coloring problem by simulated
    /// annealing, using `k` colors, an initial temperature `t0`,
    /// and a maximal number of steps `nsteps`.
    /// 
    /// Upon success, returns the vector of colors.
    pub fn solve(&self, k: usize, t0: f64, nsteps: usize) -> Option<Vec<usize>> {
        let mut rng = rand::weak_rng();
        let rnd_color = Range::new(0, k);
        let mut colors = Vec::with_capacity(self.nodes.len());
        for _ in 0..self.nodes.len() {
            colors.push(rnd_color.ind_sample(&mut rng));
        }
        assert_eq!(colors.len(), self.nodes.len());
        let mut freqs: Vec<Vec<isize>> = vec![vec![0; k]; self.nodes.len()];
        for (i, neighbors) in self.nodes.iter().enumerate() {
            for &j in neighbors {
                freqs[i][colors[j]] += 1;
            }
        }
        for istep in 0..nsteps {
            let t = t0*(1. - (istep as f64)/(nsteps as f64));
            let conflicts: Vec<usize> = colors.iter().enumerate()
                .filter_map(|(i, &color)| if freqs[i][color] > 0 { Some(i) } else { None })
                .collect();
            if conflicts.is_empty() {
                return Some(colors);
            }
            // println!("T: {:.8}, nb_conflicts: {}", t, conflicts.len());
            for _ in 0..4*k*conflicts.len() {
                let iv = rng.gen_range(0, conflicts.len());
                let v = conflicts[iv];
                let old_color = colors[v];
                let new_color = rnd_color.ind_sample(&mut rng);
                let delta = freqs[v][new_color] - freqs[v][old_color];
                if rng.gen::<f64>() < (-(delta as f64) / t).exp() {
                    colors[v] = new_color;
                    for &j in &self.nodes[v] {
                        freqs[j][new_color] += 1;
                        freqs[j][old_color] -= 1;
                    }
                }
            }
        }
        None
    }
    /// Checks if a given coloring is valid.
    pub fn check(&self, colors: &[usize]) -> bool {
        for (i, neighbors) in self.nodes.iter().enumerate() {
            if neighbors.iter().any(|&j| colors[i] == colors[j]) {
                return false
            }
        }
        true
    }
}

