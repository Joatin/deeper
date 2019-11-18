use crate::network_builder::NetworkBuilder;
use crate::basic_layer::BasicLayer;
use crate::layer::Layer;
use std::fmt::{Display, Formatter, Error};
use futures::Future;
use futures::future::join_all;
use std::sync::Arc;


pub struct Network {
    layers: Vec<Box<dyn Layer>>,
    iterations: u32
}

impl Network {

    pub(crate) fn new(layers: Vec<Box<dyn Layer>>) -> Self {
        Self {
            layers,
            iterations: 0
        }
    }

    pub fn builder(input_size: u32) -> NetworkBuilder {
        NetworkBuilder::new(input_size)
    }

    pub async fn tick(&mut self, input_data: Vec<u8>) -> Vec<u8> {
        let mut data = input_data;
        for l in &self.layers {
            data = l.tick(data).await
        }
        assert_eq!(data.len() as u64, self.output_size());
        self.iterations += 1;
        data
    }

    pub async fn improve(&self, input_data: Vec<f32>) {
        let mut data = input_data;
        for l in self.layers.iter().rev() {
            data = l.improve(data).await;
        }
    }

    pub fn total_neurons(&self) -> u32 {
        self.layers.iter()
            .map(|l| {
                l.total_neurons()
            })
            .sum()
    }

    pub fn total_connections(&self) -> u64 {
        self.layers.iter()
            .fold(0 as u64, |mut acc, l| {
                acc += l.total_connections() as u64;
                acc as u64
            })
    }

    pub fn input_size(&self) -> u64 {
        self.layers.first().map(|l| l.total_neurons()).unwrap_or_default() as u64
    }

    pub fn output_size(&self) -> u64 {
        self.layers.last().map(|l| l.total_neurons()).unwrap_or_default() as u64
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "~~~ Network ~~~\n")?;
        write!(f, "Inputs: {}\n", self.input_size())?;
        write!(f, "Total Neurons: {}\n", self.total_neurons())?;
        write!(f, "Total Connections: {}\n", self.total_connections())?;
        write!(f, "Outputs: {}\n", self.output_size())?;
        write!(f, "Iterations: {}\n", self.iterations)?;
        write!(f, "~~~~~~~~~~~~~~~")?;
        Ok(())
    }
}