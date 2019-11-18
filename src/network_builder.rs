use crate::Network;
use crate::layer::Layer;
use crate::basic_layer::BasicLayer;

pub struct NetworkBuilder {
    layers: Vec<Box<dyn Layer>>,
    input_size: u32,
}

impl NetworkBuilder {
    pub fn new(input_size: u32) -> Self {
        Self {
            layers: vec![],
            input_size
        }
    }

    pub fn add_layer(mut self, size: u32) -> Self {
        let prev_size = if self.layers.is_empty() {
            self.input_size
        } else {
            self.layers.last().unwrap().total_neurons()
        };

        let layer = BasicLayer::new(size, prev_size);
        self.layers.push(Box::new(layer));
        self
    }

    pub fn build(self) -> Network {
        if self.layers.is_empty() {
            panic!("You must have at least one layer!")
        }

        Network::new(
            self.layers
        )
    }
}