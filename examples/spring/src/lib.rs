pub use fmrs_model::prelude::*;

#[derive(FmrsModel)]
pub struct Spring {
    #[parameter]
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    #[output]
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    private_float: f64,
    private_bool: bool,
}

impl Spring {
    pub fn do_step(&mut self, step_size: f64) {
        let force = -self.stiffness * self.position - self.damping * self.velocity;
        self.acceleration = force / self.mass;
        self.velocity += self.acceleration * step_size;
        self.position += self.velocity * step_size;
    }
}