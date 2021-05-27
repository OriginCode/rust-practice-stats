pub trait Data {
    fn get_mean(&self) -> f32;
    fn get_sd(&self) -> f32;
    fn get_param_or_stat(&self, val: impl Into<f32>) -> f32 {
        let mean = self.get_mean();
        let sd = self.get_sd();
        (val.into() - mean) / sd
    }
}

pub mod population;
pub mod sample;
