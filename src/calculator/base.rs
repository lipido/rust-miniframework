use miniframework::framework::{observer, operation};



pub struct BasicOperation<O: observer::Observer<f32>> {
    name: String,
    parameter_names: Vec<String>,
    observer_manager: observer::ObserversManager<f32, O>,
}

impl<O: observer::Observer<f32>> BasicOperation<O> {
    pub fn new(name: &str, parameter_names: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            parameter_names: parameter_names.iter().map(|s| s.to_string()).collect(),
            observer_manager: observer::ObserversManager::new(),
        }
    }
    pub fn notify_observers(&self, data: &f32) {
        self.observer_manager.notify_observers(data)
    }
}

impl<O: observer::Observer<f32>> operation::Operation for BasicOperation<O> {
    fn display_name(&self) -> &str {
        &self.name
    }

    fn parameter_names(&self) -> &Vec<String> {
        &self.parameter_names
    }

    fn run(&self, _parameter_values: Vec<String>) -> String {
        panic!("Method not implemented")
    }

}

impl<O: observer::Observer<f32>> observer::Observable<f32, O> for BasicOperation<O> {
    fn add_observer(&mut self, observer: O) {
        self.observer_manager.add_observer(observer)
    }
}
