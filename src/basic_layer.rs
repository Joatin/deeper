use crate::layer::Layer;
use crate::basic_neuron::BasicNeuron;
use std::sync::Arc;
use futures::{Future, FutureExt};
use futures::future::join_all;
use std::fmt::{Display, Formatter, Error};
use rand::Rng;

pub struct BasicLayer {
    neurons: Vec<BasicNeuron>,
    input_data_size: u32
}

impl BasicLayer {

    pub fn new(size: u32, input_data_size: u32) -> Self {
        let neurons = vec![BasicNeuron::new(0.5, input_data_size as u32); size as usize];
        Self {
            neurons,
            input_data_size
        }
    }
}

impl Layer for BasicLayer {
    fn tick(&self, input_data: Vec<u8>) -> Box<dyn Future<Output=Vec<u8>> + Unpin> {
        assert_eq!(input_data.len() as u32, self.input_data_size);
        let data = Arc::new(input_data);
        let all_fut: Vec<_> = self.neurons.iter()
            .map(|n| {
                n.tick(Arc::clone(&data))
            })
            .collect();
        Box::new(join_all(all_fut))
    }

    fn total_neurons(&self) -> u32 {
        self.neurons.len() as u32
    }

    fn total_connections(&self) -> u64 {
        self.input_data_size as u64 * self.total_neurons() as u64
    }

    fn improve(&self, input_data: Vec<f32>) -> Box<dyn Future<Output=Vec<f32>> + Unpin> {
        let all_fut: Vec<_> = self.neurons.iter()
            .zip(input_data.into_iter())
            .map(|(n, p)| {
                n.improve(p)
            })
            .collect();
        Box::new(join_all(all_fut).map(|amounts| {
            let mut result = vec![0.0; amounts[0].len()];

            for v in &amounts {
                for (i, item) in v.iter().enumerate() {
                    result[i] += *item;
                }
            }
            for i in &mut result {
                *i = *i / amounts.len() as f32
            }
            result
        }))
    }
}

impl Display for BasicLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}