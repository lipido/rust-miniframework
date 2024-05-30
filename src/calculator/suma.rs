
use super::base;
use miniframework::{observer, operation::{self, ObservableOperation}};

pub struct Suma {
    base: base::BasicOperation<Box<dyn observer::Observer<f32>>>,
    
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


    fn run(&self, parameter_values: Vec<String>) -> String {
        if parameter_values.len() != 2 {
            panic!("This operation requires exactly two integers")
        }

        let mut result = String::new();

        self.base.notify_observers(&0.5);

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

impl observer::Observable<f32, Box<dyn observer::Observer<f32>>> for Suma {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer<f32>>) {
        self.base.add_observer(observer)
    }
}

impl ObservableOperation<Box<dyn observer::Observer<f32>>> for Suma {}

