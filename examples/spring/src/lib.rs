pub use fmrs_model::prelude::*;

#[derive(FmrsModel, Debug, Default)]
#[fmi_version = 2]
pub struct Spring {
    #[parameter] // Every variable below this attribute will be a parameter.
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    #[output] // Every variable below this attribute will be an output variable. Input is also possible but not used here.
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    private_test_variable: bool, // Private variables do not get an FMI-interface, but can be used internally in the model if needed.
}

impl Spring {
    /// Currently the only mandatory function. Should probably be a trait in the future...
    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let force = -self.stiffness * self.position - self.damping * self.velocity;

        self.acceleration = if self.mass != 0.0 {
            force / self.mass
        } else {
            0.0
        };

        self.velocity += self.acceleration * time_step;
        self.position += self.velocity * time_step;
    }
}