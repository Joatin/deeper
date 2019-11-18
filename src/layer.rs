use futures::Future;
use std::sync::Arc;

pub trait Layer {
    fn tick(&self, input_data: Vec<u8>) -> Box<dyn Future<Output=Vec<u8>> + Unpin>;
    fn total_neurons(&self) -> u32;
    fn total_connections(&self) -> u64;
    fn improve(&self, input_data: Vec<f32>) -> Box<dyn Future<Output=Vec<f32>> + Unpin>;
}