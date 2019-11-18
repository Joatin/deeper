use deeper::Network;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::runtime::Builder;
use image::{load_from_memory_with_format, ImageFormat};

const CIRCLE: &'static [u8] = include_bytes!("circle.png");

fn main() {
    let rt = Builder::new()
        .blocking_threads(16)
        .core_threads(16)
        .build()
        .unwrap();

    let img = load_from_memory_with_format(&CIRCLE, ImageFormat::PNG).unwrap();

    let bytes: Vec<i8> = img.to_luma()
        .to_vec()
        .into_iter()
        .map(|b| b as i8 + 5)
        .collect();

    let now = Instant::now();
    let mut network = Network::builder(bytes.len() as u32)
        .add_layer(10_000)
        .add_layer(1)
        .build();

    let mut result = vec![];

    println!("Starting to process data! Image size is {}", bytes.len());
    println!("{}", bytes[100]);

    for _ in 0..1 {
        let fut = network.tick(bytes.clone());
        let block = rt.block_on(fut);
        result = block;
    }

    let dur = Instant::now().duration_since(now);

    println!("Tick Result: {:?}", result[0]);
    println!("\n{}\n", network);
    println!("Total time: {:#?}", dur);
    println!("Hello");
}