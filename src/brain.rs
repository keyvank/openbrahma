use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Weight = i32;
pub type NeuronId = usize;
pub type Axon = (Weight, NeuronId);

const LEAK: Weight = 1i32;
const THRESHOLD: Weight = 50i32;
const REST: Weight = -10i32;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Neuron {
    pub energy: Weight,
    pub delta: Weight,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            energy: 0,
            delta: 0,
        }
    }

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    neurons: HashMap<NeuronId, (Neuron, Vec<Axon>)>,
    neuron_id: NeuronId,
    connectivity: usize,
}

impl Brain {
    pub fn add_neuron(&mut self, n: Neuron) {
        self.neurons.insert(self.neuron_id, (n, Vec::new()));
        self.neuron_id += 1;
    }

    pub fn new(neuron_count: usize, connectivity: usize) -> Brain {
        let mut rng = thread_rng();

        let mut b = Brain {
            neurons: HashMap::new(),
            neuron_id: 0,
            connectivity: connectivity,
        };

        for _ in 0..neuron_count {
            b.add_neuron(Neuron {
                delta: 0i32,
                energy: 0i32,
            });
        }

        let ids = b.neuron_ids();
        for (_, (_, edges)) in b.neurons.iter_mut() {
            for &to in ids.choose_multiple(&mut rng, connectivity) {
                edges.push((rng.gen_range(0, THRESHOLD), to));
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

    pub fn neuron_ids(&self) -> Vec<NeuronId> {
        self.neurons.keys().copied().collect::<Vec<_>>()
    }

    pub fn random_neurons(&self, count: usize) -> Vec<NeuronId> {
        let mut rng = thread_rng();
        self.neuron_ids()
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

    pub fn crossover(&mut self, b: &Brain) {
        let mut rng = thread_rng();
        let ids = self.neuron_ids();
        for (id, (neuron, axons)) in b.neurons.iter() {
            if rng.gen::<bool>() {
                let nn = self.neurons.entry(*id).or_insert((*neuron, Vec::new()));
                nn.1.clear();
                for (new_weight, new_id) in axons {
                    if ids.contains(new_id) {
                        nn.1.push((*new_weight, *new_id));
                    }
                }
            }
        }
    }

    pub fn mutate(&mut self, rate: f32) {
        let mut rng = thread_rng();
        let ids = self.neuron_ids();
        for (_, (_, edges)) in self.neurons.iter_mut() {
            if rng.gen::<f32>() < rate {
                edges.clear();
                for &to in ids.choose_multiple(&mut rng, self.connectivity) {
                    edges.push((rng.gen_range(0, THRESHOLD), to));
                }
            }
        }
    }

    pub fn update(&mut self) {
        for (n, _) in self.neurons.values_mut() {
            n.update();
        }
    }
}
