use rand::seq::SliceRandom;
use rand::thread_rng;

use std::cell::RefCell;
use std::rc::Rc;

const LEAK: i32 = 1i32;
const THRESHOLD: i32 = 50i32;
const REST: i32 = -10i32;
const POWER: i32 = 3i32;

#[derive(Debug)]
struct Neuron {
    energy: i32,
    outputs: Vec<Rc<RefCell<Neuron>>>,
}

impl Neuron {
    pub fn stimulate(&mut self, power: i32) {
        // If not resting
        if self.energy > 0 {
            self.energy += power;
            if self.energy > THRESHOLD {
                for n in self.outputs.iter() {
                    n.borrow_mut().stimulate(POWER);
                }
                self.energy = REST;
            }
        }
    }

    pub fn tick(&mut self) {
        if self.energy < 0 {
            self.energy += LEAK;
        } else {
            self.energy -= LEAK;
        }
    }
}

pub struct Brain {
    neurons: Vec<Rc<RefCell<Neuron>>>,
}

impl Brain {
    pub fn new(neuron_count: usize, connection_count: usize) -> Brain {
        let mut rng = thread_rng();

        let b = Brain {
            neurons: (0..neuron_count)
                .map(|_| {
                    Rc::new(RefCell::new(Neuron {
                        energy: 0i32,
                        outputs: Vec::new(),
                    }))
                })
                .collect(),
        };

        for src in b.neurons.iter() {
            for dst in b.neurons.choose_multiple(&mut rng, connection_count) {
                src.borrow_mut().outputs.push(Rc::clone(dst));
            }
        }

        b
    }

    pub fn tick(&mut self) {
        for n in self.neurons.iter() {
            n.borrow_mut().tick();
        }
    }
}
