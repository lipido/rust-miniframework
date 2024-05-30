
// Command design pattern

use super::observer;


pub trait Operation {
    fn display_name(&self) -> &str;
    fn parameter_names(&self) -> &Vec<String>;
    fn run(&self, parameter_values: Vec<String>) -> String;
}


pub trait ObservableOperation<O: observer::Observer<f32>>: Operation + observer::Observable<f32, O> {

}
