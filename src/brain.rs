use petgraph::visit::EdgeRef;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

const LEAK: i32 = 1i32;
const THRESHOLD: i32 = 50i32;
const REST: i32 = -10i32;

#[derive(Debug, Clone)]
struct Neuron {
    energy: i32,
}

impl Neuron {
    pub fn stimulate(&mut self, power: i32) -> bool {
        // If not resting
        if self.energy >= 0 {
            self.energy += power;
            if self.energy > THRESHOLD {
                self.energy = REST;
                return true; // TODO: Means neuron should fire!
            }
        }
        return false;
    }

    pub fn tick(&mut self) {
        if self.energy >= LEAK {
            self.energy -= LEAK;
        } else {
            self.energy += LEAK;
        }
    }
}

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

pub struct Brain {
    graph: StableGraph<Neuron, i32>,
    neurons: Vec<NodeIndex>,
}

impl Brain {
    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            graph: StableGraph::new(),
            neurons: Vec::new(),
        };

        for _ in 0..neuron_count {
            let i = b.graph.add_node(Neuron { energy: 0i32 });
        }

        b.update_neurons();

        for &src in b.neurons.iter() {
            for &dst in b.neurons.choose_multiple(&mut rng, connection_count) {
                b.graph.add_edge(src, dst, 1i32);
            }
        }

        b
    }

    pub fn update_neurons(&mut self) {
        self.neurons = self.graph.node_indices().collect();
    }

    pub fn stimulate(&mut self, index: NodeIndex, power: i32) {
        let mut nodes = vec![(index, power)];
        while !nodes.is_empty() {
            let (ix, pow) = nodes.remove(0);
            if self.graph.node_weight_mut(ix).unwrap().stimulate(pow) {
                for neigh in self.graph.edges(ix) {
                    nodes.push((neigh.target(), *neigh.weight()));
                }
            }
        }
    }

    pub fn random_node(&self) -> NodeIndex {
        let mut rng = thread_rng();
        *self.neurons.choose(&mut rng).unwrap()
    }

    pub fn pop_region(&mut self, len: usize) -> HashMap<NodeIndex, Vec<(NodeIndex, i32)>> {
        let start = self.random_node();

        let mut ret = HashMap::new();
        ret.insert(start, Vec::new());

        let mut edges = Vec::new();

        let mut queue = vec![start];

        for _ in 0..len {
            if queue.is_empty() {
                break;
            }
            let src = queue.remove(0);
            for neigh in self.graph.edges(src) {
                let dst = neigh.target();
                if ret.contains_key(&dst) || ret.len() < len {
                    queue.push(dst);
                    ret.entry(dst).or_insert(Vec::new());
                    if !edges.contains(&neigh.id()) {
                        ret.get_mut(&src).unwrap().push((dst, *neigh.weight()));
                        edges.push(neigh.id());
                    }
                }
            }
        }

        for ix in edges {
            self.graph.remove_edge(ix);
        }

        ret
    }

    pub fn crossover(&mut self, b: &mut Brain) {
        let reg_a = self.pop_region(1000);
        let reg_b = b.pop_region(1000);
        let a_to_b = reg_a
            .keys()
            .copied()
            .zip(reg_b.keys().copied())
            .collect::<HashMap<_, _>>();
        let b_to_a = reg_b
            .keys()
            .copied()
            .zip(reg_a.keys().copied())
            .collect::<HashMap<_, _>>();
        for (k, v) in reg_a {
            let src = a_to_b.get(&k).unwrap();
            for (dst, w) in v {
                let dst = a_to_b.get(&dst).unwrap();
                b.graph.add_edge(*src, *dst, w);
            }
        }
        for (k, v) in reg_b {
            let src = b_to_a.get(&k).unwrap();
            for (dst, w) in v {
                let dst = b_to_a.get(&dst).unwrap();
                self.graph.add_edge(*src, *dst, w);
            }
        }
    }

    pub fn tick(&mut self) {
        let indices = self.graph.node_indices().collect::<Vec<NodeIndex<_>>>();
        for i in indices {
            self.graph.node_weight_mut(i).unwrap().tick();
        }
    }
}
