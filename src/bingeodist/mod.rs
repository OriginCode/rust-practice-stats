pub trait Prob {
    fn prob(&self, x: u32) -> f32;
    fn cdf(&self, start: u32, end: u32) -> f32 {
        let mut result = 0.0;
        for n in start..=end {
            result += self.prob(n);
        }
        result
    }
}

pub mod bindist;
pub mod geodist;
