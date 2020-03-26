use rand::seq::SliceRandom;
use rand::thread_rng;

const LEAK: i32 = 1i32;
const THRESHOLD: i32 = 50i32;
const REST: i32 = -10i32;
const POWER: i32 = 3i32;

#[derive(Debug)]
struct Neuron {
    energy: i32,
}

impl Neuron {
    pub fn stimulate(&mut self, power: i32) -> bool {
        // If not resting
        if self.energy > 0 {
            self.energy += power;
            if self.energy > THRESHOLD {
                return true; // TODO: Means neuron should fire!
                self.energy = REST;
            }
        }
        return false;
    }

    pub fn tick(&mut self) {
        if self.energy < 0 {
            self.energy += LEAK;
        } else {
            self.energy -= LEAK;
        }
    }
}

use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub struct Brain {
    neurons: Graph<Neuron, u32>,
}

impl Brain {
    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            neurons: Graph::new(),
        };

        for _ in 0..neuron_count {
            let i = b.neurons.add_node(Neuron { energy: 0i32 });
        }

        let indices: Vec<NodeIndex<_>> = b.neurons.node_indices().collect();

        for &src in indices.iter() {
            for &dst in indices.choose_multiple(&mut rng, connection_count) {
                b.neurons.add_edge(src, dst, 0u32);
            }
        }

        b
    }

    pub fn tick(&mut self) {
        for n in self.neurons.node_weights_mut() {
            n.tick();
        }
    }
}
