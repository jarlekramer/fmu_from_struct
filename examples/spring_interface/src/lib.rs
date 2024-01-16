//! Example of how to make an functional mockup interface to a spring model.

pub use fmrs_model::prelude::*;

/// The model which gets an interface
#[derive(Debug, Default, Clone)]
struct SpringModel {
    mass: f64, 
    stiffness: f64,
    damping: f64,
    position: f64,
    velocity: f64,
    acceleration: f64,
}

impl SpringModel {
    fn do_step(&mut self, time_step: f64) {
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

#[derive(FmrsModel, Debug, Default, Clone)]
#[fmi_version = 2]
pub struct SpringInterface {
    #[parameter]
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    #[output] // Every variable below this attribute will be an output variable. Input is also possible but not used here.
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    // The model which gets an interface
    spring_model: Option<SpringModel>, // The option is necessary as default values are required for the FMI interface. Otherwise, it is impossible to initialize the model before parameters are set.
}

impl FmrsModelFunctions for SpringInterface {
    /// INitialize the model after the fmi interface variables has been read and set.
    fn exit_initialization_mode(&mut self) {
        self.spring_model = Some(
            SpringModel {
                mass: self.mass,
                stiffness: self.stiffness,
                damping: self.damping,
                position: self.position,
                velocity: self.velocity,
                acceleration: self.acceleration,
            }
        );
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        if let Some(ref mut model) = self.spring_model {
            model.do_step(time_step);

            self.acceleration = model.acceleration;
            self.velocity     = model.velocity;
            self.position     = model.position;
        } else {
            panic!("Model not initialized!");
        }
    }
}
