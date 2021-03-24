use crate::statistics::Data;
use std::convert::Infallible;

pub struct Population {
    pub val: Vec<f32>,
}

pub trait Parameter: Data {
    fn get_parameter(&self, val: f32) -> Result<f32, Infallible>;
}

impl Data for Population {
    fn get_mean(&self) -> Result<f32, Infallible> {
        let result = self.val.iter().sum::<f32>() / self.val.len() as f32;
        Ok(result)
    }
    fn get_sd(&self) -> Result<f32, Infallible> {
        let mean = self.get_mean()?;
        let result = self
            .val
            .iter()
            .map(|x| f32::powf(x - mean, 2.0))
            .sum::<f32>()
            / self.val.len() as f32;
        Ok(result)
    }
}

impl Parameter for Population {
    fn get_parameter(&self, val: f32) -> Result<f32, Infallible> {
        let mean = self.get_mean()?;
        let sd = self.get_sd()?;
        let result = (val - mean) / sd;
        Ok(result)
    }
}
