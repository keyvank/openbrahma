use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub type Weight = i32;
pub type NeuronId = usize;

const LEAK: Weight = 1i32;
const THRESHOLD: Weight = 50i32;
const REST: Weight = -10i32;
const WEIGHT: Weight = 3i32;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub energy: i32,
    pub delta: Weight,
}

impl Neuron {
    pub fn stimulate(&mut self, power: Weight) -> bool {
        self.delta += power;

        // If not resting
        if self.energy >= 0 {
            self.energy += power;
            if self.energy > THRESHOLD {
                self.energy = REST;
                return true;
            }
        }
        return false;
    }

    pub fn update(&mut self) {
        self.delta = 0;
        if self.energy >= LEAK {
            self.energy -= LEAK;
        } else {
            self.energy += LEAK;
        }
    }
}

pub struct Brain {
    neurons: HashMap<NeuronId, (Neuron, Vec<(Weight, NeuronId)>)>,
    neuron_id: NeuronId,
}

impl Brain {
    pub fn add_neuron(&mut self, n: Neuron) {
        self.neurons.insert(self.neuron_id, (n, Vec::new()));
        self.neuron_id += 1;
    }

    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            neurons: HashMap::new(),
            neuron_id: 0,
        };

        for _ in 0..neuron_count {
            b.add_neuron(Neuron {
                delta: 0i32,
                energy: 0i32,
            });
        }

        let ids = b.neurons.keys().copied().collect::<Vec<NeuronId>>();
        for (id, (neuron, edges)) in b.neurons.iter_mut() {
            for &to in ids.choose_multiple(&mut rng, connection_count) {
                edges.push((WEIGHT, to));
            }
        }

        b
    }

    pub fn stimulate(&mut self, index: NeuronId, power: Weight) {
        let mut nodes = vec![(power, index)];
        while !nodes.is_empty() {
            let (pow, ix) = nodes.remove(0);
            let (neuron, edges) = self.neurons.get_mut(&ix).unwrap();
            if neuron.stimulate(pow) {
                for edge in edges {
                    nodes.push(*edge);
                }
            }
        }
    }

    pub fn random_neurons(&self, count: usize) -> Vec<NeuronId> {
        let mut rng = thread_rng();
        self.neurons
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, count)
            .copied()
            .collect()
    }

    pub fn get_deltas(&self, neurons: &Vec<NeuronId>) -> Vec<Weight> {
        neurons
            .iter()
            .map(|&id| self.neurons[&id].0.delta)
            .collect()
    }

    /*fn pop_region(&mut self, len: usize) -> HashMap<NeuronId, Vec<(NeuronId, i32)>> {
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
    }*/

    pub fn update(&mut self) {
        for (n, _) in self.neurons.values_mut() {
            n.update();
        }
    }
}
