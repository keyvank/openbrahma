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
    pub fn new(neuron_count: usize) -> Brain {
        Brain {
            neurons: vec![
                Rc::new(RefCell::new(Neuron {
                    energy: 0i32,
                    outputs: Vec::new(),
                }));
                neuron_count
            ],
        }
    }

    pub fn tick(&mut self) {
        for n in self.neurons.iter() {
            n.borrow_mut().tick();
        }
    }
}
