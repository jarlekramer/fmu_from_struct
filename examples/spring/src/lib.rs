pub use fmrs_model::prelude::*;

#[derive(FmrsModel)]
pub struct Spring {
    mass: f64,
    stiffness: f64,
    damping: f64,
    position: f64,
    velocity: f64,
    acceleration: f64,
}

impl Spring {
    pub fn do_step(&mut self, step_size: f64) {
        let force = -self.stiffness * self.position - self.damping * self.velocity;
        self.acceleration = force / self.mass;
        self.velocity += self.acceleration * step_size;
        self.position += self.velocity * step_size;
    }
}