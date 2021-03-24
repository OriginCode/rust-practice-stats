use crate::statistics::Data;
use std::convert::Infallible;

pub struct Sample {
    pub val: Vec<f32>,
}

impl Data for Sample {
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
            / (self.val.len() as f32 - 1.0);
        Ok(result)
    }
}
