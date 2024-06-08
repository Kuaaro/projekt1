use rand_distr::Normal;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::constants;


pub struct NatDisNumGen {
    normal: Normal<f32>,
    generator: ThreadRng
}

impl NatDisNumGen {

    pub fn new_const() -> Self {
        let mean = constants::CONSTANTS.lock().unwrap().mean;
        let variance = constants::CONSTANTS.lock().unwrap().variance;

        let normal = Normal::new(mean, variance).unwrap();
        let generator = rand::thread_rng();

        return NatDisNumGen{normal, generator};
    }

    pub fn range_gen(&mut self, min: i32, max: i32) -> i32 {
        let diff = max - min;
        let out_f32 = diff as f32 * self.normal_gen();
        return min + out_f32.round() as i32;
    
    }

    pub fn normal_gen(&mut self) -> f32 {
        let mut possible_out = self.generator.sample(self.normal);
        while possible_out < 0. || possible_out > 1. {
            possible_out = self.generator.sample(self.normal);
        }

        return possible_out;
    }
}