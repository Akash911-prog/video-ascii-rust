use std::ops::Deref;

use crate::{config::Config, p2c_mapper::{logarithmic_mapper::LogarithmicMapper, sigmoid_mapper::SigmoidMapper}};

pub mod logarithmic_mapper;
pub mod sigmoid_mapper;

pub trait Map {
    fn map(&self, pixel: &u8, flip: bool) -> u8;
}

pub struct P2CMapper<'a> {
    mapper: Box<dyn Map + 'a>,
}

impl<'a> P2CMapper<'a> {
    pub fn new(mapper_type: P2CMapperType, config: &'a Config) -> Self {
        let mapper: Box<dyn Map + 'a> = match mapper_type {
            P2CMapperType::Sigmoid => {
                Box::new(SigmoidMapper::new(
                    *b".,''-:;=!$@#",
                    config,
                ))
            }

            P2CMapperType::Logarithmic => {
                Box::new(LogarithmicMapper::new(
                    [44, 46, 32],
                    [43, 64, 35],
                ))
            }
        };

        Self { mapper }
    }
}


pub enum P2CMapperType {
    Sigmoid,
    Logarithmic,
}

impl<'a> Deref for P2CMapper<'a> {
    type Target = Box<dyn Map + 'a>;

    fn deref(&self) -> &Self::Target {
        &self.mapper
    }
}
