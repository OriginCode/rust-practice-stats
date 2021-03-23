mod data;
mod population;
mod sample;
mod bingeodist;

use crate::data::gen_result;
use crate::population::Population;
use crate::population::Parameter;
use crate::sample::Sample;
use crate::bingeodist::BinomialDist;
use crate::bingeodist::GeometricDist;
use crate::bingeodist::Prob;
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    let test_sample = Sample {
        val: vec![1.0, 2.0, 5.0, 10.0],
    };
    let test_population = Population {
        val: vec![2.0, 5.0, 8.0, 7.0],
    };
    let test_bin = BinomialDist {
        n: 10,
        p: 0.5,
    };
    let test_geo = GeometricDist {
        p: 0.25,
    };
    let (sm, ssd) = gen_result(&test_sample)?;
    let (pm, psd) = gen_result(&test_population)?;
    println!("{}, {}", sm, ssd);
    println!("{}, {} (Parameter for n = 10): {}", pm, psd, test_population.get_parameter(10.0)?);
    println!("{}", test_bin.prob(3));
    println!("{}", test_geo.prob(3));
    println!("{}", bingeodist::cdf(&test_geo, 1, 5));
    Ok(())
}
