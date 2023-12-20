pub use fmrs_model::prelude::*;

use spring_model::Spring as SpringModel;

#[derive(FmrsModel, Debug, Default)]
#[fmi_version = 3]
pub struct Spring {
    #[parameter]
    pub parameter_file_path: String,
    #[output] // Every variable below this attribute will be an output variable. Input is also possible but not used here.
    pub position: f64,
    pub velocity: f64,
    pub acceleration: f64,

    /// The model which gets an interface
    spring_model: Option<SpringModel>, // Option necessary to easily create a default value?
}

impl Spring {
    /// Init the external model. Needs implementation on the fmi side.
    pub fn init_model(&mut self) {
        self.spring_model = Some(
            SpringModel::from_file(&self.parameter_file_path).unwrap()
        );
    }

    pub fn do_step(&mut self, _current_time: f64, time_step: f64) {
        if let Some(model) == self.spring_model {
            model.do_step(time_step);

            self.acceleration = model.acceleration;
            self.velocity     = model.velocity;
            self.position     = model.position;
        } else {
            panic!("Model not initialized!");
        }
    }
}
