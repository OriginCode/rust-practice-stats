mod data;
mod population;
mod sample;

use crate::data::gen_result;
use crate::population::Population;
use crate::sample::Sample;
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let test_sample = Sample {
        val: vec![1.0, 2.0, 5.0, 10.0],
        n: 4,
    };
    let test_population = Population {
        val: vec![2.0, 5.0, 8.0, 7.0],
        n: 4,
    };
    let (sm, ssd) = gen_result(&test_sample)?;
    let (pm, psd) = gen_result(&test_population)?;
    println!("{}, {}", sm, ssd);
    println!("{}, {}", pm, psd);
    Ok(())
}
