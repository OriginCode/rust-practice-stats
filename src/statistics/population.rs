use crate::statistics::Data;

pub struct Population {
    pub val: Vec<f32>,
}

impl Data for Population {
    fn get_mean(&self) -> f32 {
        self.val.iter().sum::<f32>() / self.val.len() as f32
    }
    fn get_sd(&self) -> f32 {
        let mean = self.get_mean();
        self.val
            .iter()
            .map(|x| f32::powf(x - mean, 2.0))
            .sum::<f32>()
            / self.val.len() as f32
    }
}
