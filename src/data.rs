use std::convert::Infallible;

pub trait Data {
    fn get_mean(&self) -> Result<f32, Infallible>;
    fn get_sd(&self) -> Result<f32, Infallible>;
}

pub fn gen_result<T>(data: &T) -> Result<(f32, f32), Infallible>
where
    T: Data,
{
    let mean = data.get_mean()?;
    let sd = data.get_sd()?;
    println!("Mean: {}", &mean);
    println!("SD: {}", &sd);
    Ok((mean, sd))
}