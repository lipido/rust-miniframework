
// Command design pattern
use crate::observer;

pub trait Operation: observer::Observable {
    fn display_name(&self) -> &str;
    fn parameter_names(&self) -> &Vec<String>;
    fn run(&self, parameter_values: Vec<String>) -> String;
    fn progress(&self) -> f32;
}