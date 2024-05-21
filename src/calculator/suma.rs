
use super::base;
use miniframework::{observer, operation};

pub struct Suma {
    base: base::BasicOperation,
    
}

impl Suma {
    pub fn new() -> Self {
        Self {
            base: base::BasicOperation::new("Suma", vec!["sumando A", "sumando B"]),
        }
    }
}

impl operation::Operation for Suma {
    fn display_name(&self) -> &str {
        self.base.display_name()
    }
    
    fn parameter_names(&self) -> &Vec<String> {
        self.base.parameter_names()
    }

    fn progress(&self) -> f32 {
        self.base.progress()
    }

    fn run(&self, parameter_values: Vec<String>) -> String {
        if parameter_values.len() != 2 {
            panic!("This operation requires exactly two integers")
        }

        let mut result = String::new();

        self.base.update_progress(0.5);
        self.base.notify_observers(self);

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
        self.base.add_observer(observer)
    }
    // fn get_value(&self) -> Any {
    //     self.base.borrow().get_value().clone()
    // }

}
