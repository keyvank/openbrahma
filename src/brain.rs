use petgraph::visit::EdgeRef;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

const LEAK: i32 = 1i32;
const THRESHOLD: i32 = 50i32;
const REST: i32 = -10i32;
const WEIGHT: i32 = 3i32;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub energy: i32,
    pub delta: i32,
}

impl Neuron {
    pub fn stimulate(&mut self, power: i32) -> bool {
        self.delta += power;

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
        self.delta = 0;
        if self.energy >= LEAK {
            self.energy -= LEAK;
        } else {
            self.energy += LEAK;
        }
    }
}

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

pub type NeuronId = NodeIndex;

pub struct Brain {
    graph: StableGraph<Neuron, i32>,
    neurons: Vec<NeuronId>,
}

impl Brain {
    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            graph: StableGraph::new(),
            neurons: Vec::new(),
        };

        for _ in 0..neuron_count {
            let i = b.graph.add_node(Neuron {
                delta: 0i32,
                energy: 0i32,
            });
        }

        b.update_neurons();

        for &src in b.neurons.iter() {
            for &dst in b.neurons.choose_multiple(&mut rng, connection_count) {
                b.graph.add_edge(src, dst, WEIGHT);
            }
        }

        b
    }

    fn update_neurons(&mut self) {
        self.neurons = self.graph.node_indices().collect();
    }

    pub fn stimulate(&mut self, index: NeuronId, power: i32) {
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

    pub fn random_neurons(&self, count: usize) -> Vec<NeuronId> {
        let mut rng = thread_rng();
        self.neurons
            .choose_multiple(&mut rng, count)
            .copied()
            .collect()
    }

    pub fn get_deltas(&self, neurons: &Vec<NeuronId>) -> Vec<i32> {
        neurons
            .iter()
            .map(|&id| self.graph.node_weight(id).unwrap().delta)
            .collect()
    }

    fn pop_region(&mut self, len: usize) -> HashMap<NeuronId, Vec<(NeuronId, i32)>> {
        let start = self.random_neurons(1)[0];

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

    pub fn crossover(&mut self, b: &mut Brain, len: usize) {
        let reg_a = self.pop_region(len);
        let reg_b = b.pop_region(len);
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
        for &i in self.neurons.iter() {
            self.graph.node_weight_mut(i).unwrap().tick();
        }
    }
}
