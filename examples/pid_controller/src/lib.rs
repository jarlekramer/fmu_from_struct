pub use fmu_from_struct::prelude::*;

#[derive(Debug, Default, Clone, Fmu)]
#[fmi_version = 2]
/// A basic PID controller with anti-windup.
/// 
/// Source: <https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller>
pub struct PIDController {
    #[parameter]
    pub proportional_gain: f64,
    pub derivative_gain: f64,
    pub integral_gain: f64,
    pub max_value: f64,
    pub min_value: f64,
    pub reverse_output: bool,
    #[input]
    pub set_point: f64,
    pub input: f64,
    #[output]
    pub output: f64,

    previous_error: f64,
    integral_state: f64,
}

impl FmuFunctions for PIDController {
    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let error = self.set_point - self.input;

        let error_derivative = (error - self.previous_error) / time_step;
        self.previous_error = error;

        self.integral_state += error * time_step;

        let integral_check = self.integral_gain * self.integral_state;

        if integral_check > self.max_value && self.integral_gain != 0.0{
            self.integral_state = self.max_value / self.integral_gain;
        } else if integral_check < self.min_value  && self.integral_gain != 0.0 {
            self.integral_state = self.min_value / self.integral_gain;
        }

        let proportional = self.proportional_gain * error;
        let integral     = self.integral_gain * self.integral_state;
        let derivative   = self.derivative_gain * error_derivative;

        self.output = proportional + derivative + integral;

        if self.reverse_output {
            self.output = -self.output;
        }

        if self.output > self.max_value {
            self.output = self.max_value;
        } else if self.output < self.min_value {
            self.output = self.min_value;
        }
    }
}