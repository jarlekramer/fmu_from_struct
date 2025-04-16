//! Example of how to make an functional mockup interface to a spring model.

pub use fmu_from_struct::prelude::*;

/// The actual spring model. The FMU is implemented as an interface to this model. Not strictly 
/// speaking necessary for this simple example, but this design pattern is often useful for more
/// complex cases. 
#[derive(Debug, Default, Clone)]
struct SpringModel {
    mass: f64, 
    stiffness: f64,
    damping: f64,
    external_force: f64,
    position: f64,
    velocity: f64,
    acceleration: f64,
}

impl SpringModel {
    /// The do step function which calculates the next acceleration, velocity and position based on 
    /// the current state of the model and the time step
    fn do_step(&mut self, time_step: f64) {
        let force = self.external_force - self.stiffness * self.position - self.damping * self.velocity;

        self.acceleration = if self.mass != 0.0 {
            force / self.mass
        } else {
            0.0
        };

        self.velocity += self.acceleration * time_step;
        self.position += self.velocity * time_step;
    }
}

#[derive(Fmu, Default, Debug, Clone)]
#[fmi_version = 2]
pub struct Spring {
    #[parameter]  // Every variable below this attribute will be a parameter.
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    #[input]  // Every variable below this attribute will be an input.
    pub external_force: f64,
    #[output] // Every variable below this attribute will be an output.
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    /// Optional field to store the fmu info. This field is not necessary to add, and not used in 
    /// this case. The macro will populate the struct fields automatically with information about 
    /// the FMU that is generated during run time (e.g., resource path, name, etc.). This can be 
    /// useful for debugging and logging purposes, and if some parameters would like to use the 
    /// resource path for some reason (e.g., load files using relative paths to the FMU location).
    pub fmu_info: FmuInfo,

    /// The model which gets an interface
    /// The option type is necessary as default values are required for the FMI interface. 
    /// Otherwise, it is impossible to initialize the model before parameters are set.
    spring_model: Option<SpringModel>,
}

impl FmuFunctions for Spring {
    /// Initialize the model after the fmi interface variables has been read and set.
    fn exit_initialization_mode(&mut self) {
        // Example of how to use the fmu_info field. This is not necessary, just for demonstration.
        env_logger::init();
        
        log::info!("Resource path: {:?}", self.fmu_info.resource_path);
        log::info!("Resource path exists: {:?}", self.fmu_info.resource_path.exists());

        // Initialize the model with the parameters set in the fmi interface.
        self.spring_model = Some(
            SpringModel {
                mass: self.mass,
                stiffness: self.stiffness,
                damping: self.damping,
                external_force: self.external_force,
                position: self.position,
                velocity: self.velocity,
                acceleration: self.acceleration,
            }
        );
    }

    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        // Set necessary variables on the model, run do step, and then read the output
        if let Some(ref mut model) = self.spring_model {
            model.external_force = self.external_force;

            model.do_step(time_step);

            self.acceleration = model.acceleration;
            self.velocity     = model.velocity;
            self.position     = model.position;
        } else {
            panic!("Model not initialized!");
        }
    }
}
