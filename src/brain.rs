use std::rc::Rc;

#[derive(Debug)]
struct Neuron {
    energy: i32,
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
                    energy: 0i32,
                    outputs: Vec::new(),
                });
                neuron_count
            ],
        }
    }
}
