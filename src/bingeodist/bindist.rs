use crate::bingeodist::Prob;
use factorial::Factorial;

pub struct BinomialDist {
    pub n: u32,
    pub p: f32,
}

fn ncr(n: u32, k: u32) -> u32 {
    return n.factorial() / ((n - k).factorial() * k.factorial());
}

impl Prob for BinomialDist {
    fn prob(&self, x: u32) -> f32 {
        return ncr(self.n, x) as f32
            * self.p.powi(x as i32)
            * (1.0 - self.p).powi((self.n - x) as i32);
    }
}
