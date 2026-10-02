use crate::p2c_mapper::Map;

pub struct LogarithmicMapper {
    light: [u8; 3],
    dark: [u8; 3],
}

impl LogarithmicMapper {
    pub fn new(light: [u8; 3], dark: [u8; 3]) -> Self {
        Self { light, dark }
    }
}

impl Map for LogarithmicMapper {
    fn map(&self, pixel: &u8, flip: bool) -> u8 {
        if *pixel == 0 {
            let _flip = flip;
            self.light[0]
        } else {
            self.dark[0]
        }
    }
}
