use std::array;

use crate::{config::{Config}, p2c_mapper::Map};

pub struct SigmoidMapper {
    brightness: [u8; 12],
    thresholds: [u8; 11],
}

impl SigmoidMapper {
    pub fn new(brightness: [u8; 12], config: &Config) -> Self {
        let thresholds = Self::generate_thresholds(20.0, 50.0, config);
        Self {
            brightness,
            thresholds,
        }
    }

    fn generate_thresholds(lower_bound: f32, upper_bound: f32, config: &Config) -> [u8; 11] {
        let bias: f32 = config.bias.unwrap_or(1.5);

        array::from_fn(|i| {
            let p = (i as f32 + 0.5) / 11.0;

            // Compress lower values, spread upper values
            let p = p.powf(bias);

            let threshold = lower_bound + p * (upper_bound - lower_bound);

            threshold.round().clamp(0.0, 255.0) as u8
        })
    
    }

    fn sigmoid(&self, pixel: &u8, flip: bool) -> u8 {
        let mut index = self
            .thresholds
            .iter()
            .position(|&threshold| *pixel < threshold)
            .unwrap_or(self.brightness.len() - 1);

        if flip {
            index = self.brightness.len() - 1 - index;
        }

        self.brightness[index]
    }
}

impl Map for SigmoidMapper {
    fn map(&self, pixel: &u8, flip: bool) -> u8 {
        self.sigmoid(pixel, flip)
    }
}
