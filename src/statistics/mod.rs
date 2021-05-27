pub trait Data {
    fn get_mean(&self) -> f32;
    fn get_sd(&self) -> f32;
}

pub fn get_param_or_stat(data: impl Data, val: impl Into<f32>) -> f32 {
    let mean = data.get_mean();
    let sd = data.get_sd();
    (val.into() - mean) / sd
}

pub mod population;
pub mod sample;
