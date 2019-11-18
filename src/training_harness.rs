use crate::training_set::TrainingSet;
use crate::Network;
use std::process::Output;
use futures::Future;

pub struct TrainingHarness {
    sets: Vec<TrainingSet>
}

impl TrainingHarness {
    pub fn new(sets: Vec<TrainingSet>) -> Self {
        Self {
            sets
        }
    }

    pub async fn train(&self, network: &mut Network) {
        let mut success_count = 0;
        let mut fail_count = 0;
        loop {
            for set in &self.sets {
                loop {
                    let result = network.tick(set.data.clone()).await;
                    if result == set.result {
                        println!("SUCCESS, FAILED ITERATIONS: {}", fail_count);
                        fail_count = 0;
                        success_count += 1;
                        break;
                    } else {
                        // println!("FAIL expected: {:?}, actual: {:?}", set.result, result);
                        success_count = 0;
                        fail_count += 1;
                        let diff_amount = get_diff(&set.result, &result);

                        network.improve(diff_amount).await;
                    }
                }
            }
            println!("SUCCESS COUNT: {}", success_count);
            if success_count > 100 {
                break;
            }
        }
    }
}

fn get_diff(exp: &Vec<u8>, res: &Vec<u8>) -> Vec<f32> {
    exp.iter()
        .zip(res.iter())
        .map(|(expected, actual)| {
            if expected > actual {
                0.05
            } else if expected < actual {
                -0.05
            } else {
                0.0
            }
        })
        .collect()
}