use miniframework::observer;
use miniframework::operation;
pub struct BasicOperation {
    name: String,
    parameter_names: Vec<String>,
    observer_manager: observer::ObserversManager<f32>,
}

impl BasicOperation {
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

impl operation::Operation for BasicOperation {
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

impl observer::Observable<f32, Box<dyn observer::Observer<f32>>> for BasicOperation {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer<f32>>) {
        self.observer_manager.add_observer(observer)
    }
}
