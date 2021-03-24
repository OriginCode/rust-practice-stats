use std::convert::Infallible;

pub trait Data {
    fn get_mean(&self) -> Result<f32, Infallible>;
    fn get_sd(&self) -> Result<f32, Infallible>;
}

pub mod sample;
pub mod population;