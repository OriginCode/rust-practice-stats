mod data;
mod sample;
mod population;

use std::convert::Infallible;
use crate::data::gen_result;
use crate::sample::Sample;
use crate::population::Population;

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
