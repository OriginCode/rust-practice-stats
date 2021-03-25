pub trait Prob {
    fn prob(&self, x: u32) -> f32;
}

pub fn cdf(dist: &impl Prob, x: u32, y: u32) -> f32 {
    let mut result = 0.0;
    for n in x..=y {
        result += dist.prob(n);
    }
    result
}

pub mod bindist;
pub mod geodist;