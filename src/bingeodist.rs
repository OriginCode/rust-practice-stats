use factorial::Factorial;

pub struct BinomialDist {
    pub n: u32,
    pub p: f32,
}

pub struct GeometricDist {
    pub p: f32,
}

fn ncr(n: u32, k: u32) -> u32 {
    return n.factorial() / ((n - k).factorial() * k.factorial());
}

pub trait Prob {
    fn prob(&self, x: u32) -> f32;
}

impl Prob for BinomialDist {
    fn prob(&self, x: u32) -> f32 {
        return ncr(self.n, x) as f32
            * self.p.powi(x as i32)
            * (1.0 - self.p).powi((self.n - x) as i32);
    }
}

impl Prob for GeometricDist {
    fn prob(&self, x: u32) -> f32 {
        return self.p
            * (1.0 - self.p).powi((x - 1) as i32)
    }
}

pub fn cdf(dist: &impl Prob, x: u32, y: u32) -> f32 {
    let mut result = 0.0;
    for n in x..=y {
        result += dist.prob(n);
    }
    return result
}
