use crate::bingeodist::Prob;

pub struct GeometricDist {
    pub p: f32,
}

impl Prob for GeometricDist {
    fn prob(&self, x: u32) -> f32 {
        self.p * (1.0 - self.p).powi((x - 1) as i32)
    }
}
