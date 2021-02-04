mod data;
mod population;
mod sample;

use crate::data::gen_result;
use crate::population::Population;
use crate::population::Parameter;
use crate::sample::Sample;
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let test_sample = Sample {
        val: vec![1.0, 2.0, 5.0, 10.0],
    };
    let test_population = Population {
        val: vec![2.0, 5.0, 8.0, 7.0],
    };
    let (sm, ssd) = gen_result(&test_sample)?;
    let (pm, psd) = gen_result(&test_population)?;
    println!("{}, {}", sm, ssd);
    println!("{}, {} (Parameter for n = 10): {}", pm, psd, test_population.get_parameter(10.0)?);
    Ok(())
}
