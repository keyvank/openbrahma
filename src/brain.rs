use std::rc::Rc;

#[derive(Debug)]
struct Neuron {
    energy: f64,
    outputs: Vec<Rc<Neuron>>,
}

pub struct Brain {
    neurons: Vec<Rc<Neuron>>,
}

impl Brain {
    pub fn new(neuron_count: usize) -> Brain {
        Brain {
            neurons: vec![
                Rc::new(Neuron {
                    energy: 0.0,
                    outputs: Vec::new(),
                });
                neuron_count
            ],
        }
    }
}
