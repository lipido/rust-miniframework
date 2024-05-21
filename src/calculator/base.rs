use std::cell::RefCell;

use miniframework::observer;
use miniframework::operation;
pub struct BasicOperation {
    name: String,
    parameter_names: Vec<String>,
    observer_manager: observer::ObserversManager,
    progress: RefCell<f32>,
}

impl BasicOperation {
    pub fn new(name: &str, parameter_names: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            parameter_names: parameter_names.iter().map(|s| s.to_string()).collect(),
            observer_manager: observer::ObserversManager::new(),
            progress: RefCell::new(0.0),
        }
    }
    pub fn notify_observers(&self, originator: &dyn observer::Observable) {
        self.observer_manager.notify_observers(originator)
    }
    pub fn update_progress(&self, new_progress: f32) {
        let mut progress = self.progress.borrow_mut();
        *progress = new_progress
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

    fn progress(&self) -> f32 {
        self.progress.borrow().clone()
    }
}

impl observer::Observable for BasicOperation {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer>) {
        self.observer_manager.add_observer(observer)
    }
    // fn get_value(&self) -> &dyn Any {
    //     &self.progress
    // }
}
