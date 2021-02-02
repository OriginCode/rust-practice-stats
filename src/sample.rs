use crate::data::Data;
use std::convert::Infallible;

pub struct Sample {
    pub val: Vec::<f32>,
    pub n: i32,
}

impl Data for Sample {
    fn get_mean(&self) -> Result<f32, Infallible> {
        let mut total = 0.0;
        for i in &self.val {
            total += i;
        }
        let result = total as f32 / self.n as f32;
        Ok(result)
    }
    fn get_sd(&self) -> Result<f32, Infallible> {
        let mean = &self.get_mean()?;
        let mut total = 0.0;
        for i in &self.val {
            total += f32::powf(i - mean, 2.0);
        }
        let result = total as f32 / (self.n as f32 - 1.0);
        Ok(result)
    }
}