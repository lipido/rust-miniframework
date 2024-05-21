use std::cell::RefCell;

use super::base;
use miniframework::{observer, operation};

pub struct Suma {
    base: RefCell<base::BasicOperation>,
    
}

impl Suma {
    pub fn new() -> Self {
        Self {
            base: RefCell::new(base::BasicOperation::new("Suma", vec!["sumando A", "sumando B"])),
        }
    }
}

impl operation::Operation for Suma {
    fn display_name(&self) -> String {
        self.base.borrow().display_name()
    }
    
    fn parameter_names(&self) -> Vec<String> {
        self.base.borrow().parameter_names()
    }

    fn progress(&self) -> f32 {
        self.base.borrow().progress()
    }

    fn run(&self, parameter_values: Vec<String>) -> String {
        if parameter_values.len() != 2 {
            panic!("This operation requires exactly two integers")
        }

        let mut result = String::new();

        self.base.borrow_mut().update_progress(0.5);
        self.base.borrow().notify_observers(self);

        if let (Ok(sumando_a), Ok(sumando_b)) = (
            parameter_values[0].parse::<i32>(),
            parameter_values[1].parse::<i32>(),
        ) {
            return (sumando_a + sumando_b).to_string();
        } else {
            result.push_str("Parameter must be numbers!");
        }

        result
    }
    

}

impl observer::Observable for Suma {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer>) {
        self.base.borrow_mut().add_observer(observer)
    }
    // fn get_value(&self) -> Any {
    //     self.base.borrow().get_value().clone()
    // }

}
