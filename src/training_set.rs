

pub struct TrainingSet {
    pub data: Vec<u8>,
    pub result: Vec<u8>
}

impl TrainingSet {
    pub fn new(data: Vec<u8>, result: Vec<u8>) -> Self {
        Self {
            data,
            result
        }
    }
}