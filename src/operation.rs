
// Command design pattern

use crate::observer;

pub trait Operation {
    fn display_name(&self) -> &str;
    fn parameter_names(&self) -> &Vec<String>;
    fn run(&self, parameter_values: Vec<String>) -> String;
}

impl<T> observer::Observer<T> for Box<dyn observer::Observer<T>> {
    fn update(&self, data: &T) {
        (**self).update(data);
    }
}
pub trait ObservableOperation: Operation + observer::Observable<f32, Box<dyn observer::Observer<f32>>> {

}
