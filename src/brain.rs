use petgraph::visit::EdgeRef;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
    neurons: StableGraph<Neuron, i32>,
}

impl Brain {
    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            neurons: StableGraph::new(),
        };

        for _ in 0..neuron_count {
            let i = b.neurons.add_node(Neuron { energy: 0i32 });
        }

        let indices: Vec<NodeIndex> = b.neurons.node_indices().collect();

        for &src in indices.iter() {
            for &dst in indices.choose_multiple(&mut rng, connection_count) {
                b.neurons.add_edge(src, dst, 1i32);
            }
        }

        b
    }

    pub fn stimulate(&mut self, index: NodeIndex, power: i32) {
        let mut nodes = vec![(index, power)];
        while !nodes.is_empty() {
            let (ix, pow) = nodes.remove(0);
            if self.neurons.node_weight_mut(ix).unwrap().stimulate(pow) {
                for neigh in self.neurons.edges(ix) {
                    nodes.push((neigh.target(), *neigh.weight()));
                }
            }
        }
    }

    pub fn random_node(&self) -> NodeIndex {
        let mut rng = thread_rng();

        *self
            .neurons
            .node_indices()
            .collect::<Vec<_>>()
            .choose(&mut rng)
            .unwrap()
    }

    pub fn random_region(&mut self, len: usize) -> Vec<NodeIndex> {
        let mut nodes = vec![self.random_node()];
        let mut region = Vec::new();

        while region.len() < len {
            let ix = nodes.remove(0);
            region.push(ix);
            for neigh in self.neurons.edges(ix) {
                nodes.push(neigh.target());
            }
        }

        region
    }

    pub fn tick(&mut self) {
        let indices = self.neurons.node_indices().collect::<Vec<NodeIndex<_>>>();
        for i in indices {
            self.neurons.node_weight_mut(i).unwrap().tick();
        }
    }
}
