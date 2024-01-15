//! Example of to model a spring when the model is implemented directly together with the fmi struct.
//! 
//! See the spring_interface example for the same physics, but where the model is in a separate 
//! structure.

pub use fmrs_model::prelude::*;

#[derive(FmrsModel, Debug, Default)]
#[fmi_version = 2]
pub struct Spring {
    #[parameter] // Every variable below this attribute will be a parameter.
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    pub output_message: String, // Test variable to see if the string implementation works.
    #[output] // Every variable below this attribute will be an output variable. Input is also possible but not used here.
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    private_test_variable: bool, // Private variables do not get an FMI-interface, but can be used internally in the model if needed.
}

impl FmrsModelFunctions for Spring {
    /// Currently the only mandatory function in the FmrsModelFunctions trait.
    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let force = -self.stiffness * self.position - self.damping * self.velocity;

        self.acceleration = if self.mass != 0.0 {
            force / self.mass
        } else {
            0.0
        };

        self.velocity += self.acceleration * time_step;
        self.position += self.velocity * time_step;

        println!("Output message: {}", self.output_message);
    }

    /// Optional function that runs after the first initialization of the model. Implemented here 
    /// for test purposes. The test is to see whether the values in the struct is the same as 
    /// defined by the simulation setup (gui or model description file for the simulation)
    fn exit_initialization_mode(&mut self) {
        println!("Exiting initialization mode!");

        dbg!(self);
    }
}