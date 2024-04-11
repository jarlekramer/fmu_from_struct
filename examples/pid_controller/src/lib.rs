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
    pub reverse_error_sign: bool,
    #[input]
    pub set_point: f64,
    pub input: f64,
    #[output]
    pub output: f64,

    previous_error: f64,
    integral_term: f64,
}

impl FmuFunctions for PIDController {
    fn do_step(&mut self, _current_time: f64, time_step: f64) {
        let error = if self.reverse_error_sign {
            self.input - self.set_point
        } else {
            self.set_point - self.input
        };

        let error_derivative = (error - self.previous_error) / time_step;

        self.integral_term += self.integral_gain * error * time_step;

        if self.integral_term > self.max_value {
            self.integral_term = self.max_value;
        } else if self.integral_term < self.min_value {
            self.integral_term = self.min_value;
        }

        let proportional_term = self.proportional_gain * error;
        let derivative_term   = self.derivative_gain * error_derivative;

        let control_signal = proportional_term + derivative_term + self.integral_term;

        let mut do_anti_windup = false;

        if control_signal > self.max_value {
            self.output = self.max_value;
            do_anti_windup = true;
        } else if control_signal < self.min_value {
            self.output = self.min_value;
            do_anti_windup = true;
        } else {
            self.output = control_signal;
        }

        if do_anti_windup {
            self.integral_term -= control_signal - self.output;
        }

        self.previous_error = error;
    }
}