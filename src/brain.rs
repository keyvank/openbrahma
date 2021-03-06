use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Weight = i32;
pub type NeuronId = usize;
pub type Axon = (Weight, NeuronId);

pub const LEAK: Weight = 1i32;
pub const THRESHOLD: Weight = 50i32;
pub const REST: Weight = -10i32;

pub const WEIGHT_MU: Weight = 5i32;
pub const WEIGHT_SIGMA: f64 = 3.0;

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
    fn random_weight<R: Rng>(rng: &mut R) -> Weight {
        let normal = Normal::new(WEIGHT_MU as f64, WEIGHT_SIGMA).unwrap();
        (normal.sample(rng) as Weight).max(0i32).min(THRESHOLD)
    }
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
                edges.push((Brain::random_weight(&mut rng), to));
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

    pub fn random_axons(&self, count: usize) -> Vec<Axon> {
        let mut rng = thread_rng();
        self.random_neurons(count)
            .into_iter()
            .map(|id| (Brain::random_weight(&mut rng), id))
            .collect()
    }

    pub fn get_deltas(&self, neurons: &Vec<NeuronId>) -> Vec<Weight> {
        neurons
            .iter()
            .map(|&id| self.neurons[&id].0.delta)
            .collect()
    }

    pub fn get_or_create(&mut self, id: &NeuronId) -> &mut (Neuron, Vec<Axon>) {
        self.neurons.entry(*id).or_insert((
            Neuron {
                energy: 0,
                delta: 0,
            },
            Vec::new(),
        ))
    }

    pub fn crossover_axons(&mut self, a: &mut Vec<Axon>, b: &Vec<Axon>) {
        let mut rng = thread_rng();
        for (a, b) in a.iter_mut().zip(b.iter()) {
            if rng.gen::<bool>() {
                self.get_or_create(&b.1);
                *a = *b;
            }
        }
    }

    pub fn mutate_axons(&mut self, a: &mut Vec<Axon>, rate: f32) {
        let mut rng = thread_rng();
        let length = a.len();
        for (a, b) in a.iter_mut().zip(self.random_axons(length)) {
            if rng.gen::<f32>() < rate {
                *a = b;
            }
        }
    }

    pub fn crossover_neurons(&mut self, a: &mut Vec<NeuronId>, b: &Vec<NeuronId>) {
        let mut rng = thread_rng();
        for (a, b) in a.iter_mut().zip(b.iter()) {
            if rng.gen::<bool>() {
                self.get_or_create(b);
                *a = *b;
            }
        }
    }

    pub fn mutate_neurons(&mut self, a: &mut Vec<NeuronId>, rate: f32) {
        let mut rng = thread_rng();
        let length = a.len();
        for (a, b) in a.iter_mut().zip(self.random_neurons(length)) {
            if rng.gen::<f32>() < rate {
                *a = b;
            }
        }
    }

    pub fn crossover(&mut self, b: &Brain) {
        let mut rng = thread_rng();
        let ids = self.neuron_ids();
        for (id, (_, axons)) in b.neurons.iter() {
            if rng.gen::<bool>() {
                let weights = &mut self.get_or_create(id).1;
                weights.clear();
                for (new_weight, new_id) in axons {
                    if ids.contains(new_id) {
                        weights.push((*new_weight, *new_id));
                    }
                }
            }
        }
    }

    pub fn mutate(&mut self, rate: f32) {
        let mut rng = thread_rng();
        let ids = self.neuron_ids();
        for (_, (_, weights)) in self.neurons.iter_mut() {
            if rng.gen::<f32>() < rate {
                weights.clear();
                for &to in ids.choose_multiple(&mut rng, self.connectivity) {
                    weights.push((Brain::random_weight(&mut rng), to));
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
