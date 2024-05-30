
use miniframework::framework::{observer, operation};

use super::base::BasicOperation;

pub struct Resta {
    base: BasicOperation<Box<dyn observer::Observer<f32>>>
}

impl Resta {
    pub fn new() -> Self {
        Self {
            base: BasicOperation::new(
                "Resta", 
                vec!["minuanedo", "sustraendo"])
        }
    }
}

impl operation::Operation for Resta {
    fn display_name(&self) -> &str {
        self.base.display_name()
    }
    fn parameter_names(&self) -> &Vec<String> {
        self.base.parameter_names()
    }

    fn run(&self, parameter_values: Vec<String>) -> String {
        if parameter_values.len() != 2 {
            panic!("This operation requires exactly two integers")
        }

        let mut result = String::new();

        if let (Ok(minuendo), Ok(sustraendo)) = (
            parameter_values[0].parse::<i32>(),
            parameter_values[1].parse::<i32>(),
        ) {
            return (minuendo - sustraendo).to_string();
        } else {
            result.push_str("Parameter must be numbers!");
        }

        result
    }

    
}
