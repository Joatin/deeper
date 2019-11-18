use deeper::{Network, TrainingHarness, TrainingSet};
use std::time::Instant;
use tokio::runtime::Runtime;


fn main() {
    let rt = Runtime::new().unwrap();

    let now = Instant::now();
    let mut network = Network::builder(2)
        .add_layer(3_000)
        .add_layer(1)
        .build();

    let harness = TrainingHarness::new(vec![
        TrainingSet::new(vec![1, 0], vec![0]),
        TrainingSet::new(vec![0, 1], vec![0]),
        TrainingSet::new(vec![1, 1], vec![1])
    ]);

    rt.block_on(harness.train(&mut network));

    let dur = Instant::now().duration_since(now);

    println!("\n{}\n", network);
    println!("Total time: {:#?}", dur);
    println!("Hello");
}