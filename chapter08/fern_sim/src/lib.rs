//! Simulate the growth of ferns, form the level of
//! individual cells onup.

pub mod plant_structures;
mod spores;

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// Simulate growth of a fern for a given number of iterations
pub fn run_simulate(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
