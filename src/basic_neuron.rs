use futures::Future;
use futures::future::lazy;
use std::sync::{RwLock, Arc};
use std::pin::Pin;
use rand::Rng;


pub struct BasicNeuron {
    weights: Arc<RwLock<Vec<f32>>>,
    base: Arc<RwLock<f32>>
}

impl BasicNeuron {
    pub fn new(base_value: f32, input_data_size: u32) -> Self {
        if base_value > 1.0 {
            panic!("Base value can not be higher than one");
        }
        if base_value < 0.0 {
            panic!("Base value can not be lower than zero");
        }
        let weights = Arc::new(RwLock::new(vec![base_value; input_data_size as usize]));
        let base = Arc::new(RwLock::new(128.0));
        Self {
            weights,
            base
        }
    }

    pub fn tick(&self, data: Arc<Vec<u8>>) -> impl Future<Output=u8> {
        let data = Arc::clone(&data);
        let weights_lock = Arc::clone(&self.weights);
        let base_lock = Arc::clone(&self.base);
        lazy(move |_| {
            let weights = weights_lock.read().unwrap();
            let base = base_lock.read().unwrap();
            let sum = weights.iter()
                .zip(data.iter())
                .map(|(w, d)| {
                    let mut sum = (*d as f32 + *base) * *w;
                    if sum > 255.0 {
                        sum = 255.0;
                    }
                    let res: u8 = sum.floor() as u8;
//                    println!("Input: {}", *d);
//                    println!("Weigth: {}", *w);
//                    println!("Base: {}", *base);
//                    println!("Res: {}", res);
                    res
                })
                .fold(0 as u64, |mut acc, p| {
                    acc += p as u64;
                    acc
                });
            let res = (sum / weights.len() as u64) as u8;
            res
        })
    }

    pub fn improve(&self, amount: f32) -> impl Future<Output=Vec<f32>> {
        let weights_lock = Arc::clone(&self.weights);
        let base_lock = Arc::clone(&self.base);
        lazy(move |_| {
            let mut weights = weights_lock.write().unwrap();
            let mut base = base_lock.write().unwrap();
            let mut rng = rand::thread_rng();

            if amount != 0.0 {
                *base += (amount / 100.0) * rng.gen_range(0.0, 1.0);;

                if *base < 0.0 {
                    *base = 0.0;
                }

                if *base > 255.0 {
                    *base = 255.0;
                }

                let mut result = Vec::with_capacity(weights.len());

                for w in weights.iter_mut() {
                    let inc = (amount / 100.0) * rng.gen_range(0.0, 1.0);
                    *w += inc;
                    if *w < 0.0 {
                        *w = 0.0;
                    }
                    if *w > 1.0 {
                        *w = 1.0;
                    }

                    if *w < 0.001 {
                        result.push(0.0);
                    } else {
                        result.push(inc / 2.0);
                    }
                }
                result
            } else {
                vec![0.0; weights.len()]
            }


        })
    }
}

impl Clone for BasicNeuron {
    fn clone(&self) -> Self {
        let weights = {
            let old = self.weights.read().unwrap();
            Arc::new(RwLock::new(old.clone()))
        };
        let base = {
            let old = self.base.read().unwrap();
            Arc::new(RwLock::new(old.clone()))
        };

        Self {
            weights,
            base
        }
    }
}