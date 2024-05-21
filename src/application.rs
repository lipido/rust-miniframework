use std::io;

use crate::observer;

use super::operation;

pub struct Application {
    operations: Vec<Box<dyn operation::Operation>>,
}

impl Application {
    pub fn from_operations(operations: Vec<Box<dyn operation::Operation>>) -> Self {
        
        let mut app = Application { operations };
        for operation in app.operations.iter_mut() {
            operation.add_observer(Box::new(OperationObserver{}));
        }
        app
    }

    pub fn operations(&self) -> &Vec<Box<dyn operation::Operation>> {
        &self.operations
    }

    pub fn start(&mut self) {
        
        loop {
            self.menu();
            let option = self.select_option();
            let parameter_values = self.get_parameter_values(option);
            let result = self.operations[option - 1].run(parameter_values);

            println!("{}", result);
        }
    }

    fn select_option(&self) -> usize {
        loop {
            let mut input = String::new();
            println!("Please enter an option number:");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            if let Ok(number) = input.parse::<usize>() {
                println!("You entered the number: {}", number);
                if number >= 1 && number <= self.operations.len() {
                    return number;
                } else {
                    println!("The option {} does not exist. Try another", number);
                }
            } else {
                println!("Invalid input, please enter a valid number.");
            }
        }
    }

    fn menu(&self) {
        println!("Menu\n----");
        for (i, ele) in self.operations.iter().enumerate() {
            println!("{}. {}", i + 1, ele.display_name());
        }
    }

    fn get_parameter_values(&self, operation_index: usize) -> Vec<String> {
        let operation = self.operations[operation_index - 1].as_ref();

        let mut parameter_values: Vec<String> = Vec::new();
    
        for parameter_name in operation.parameter_names() {
            println!("{}: ", parameter_name);
    
            let mut input = String::new();
            io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
            parameter_values.push(input.trim().to_string());
            
        }
    
        parameter_values
    
    }
}

struct OperationObserver {

}
impl observer::Observer for OperationObserver {
    fn update(&self, _originator: &dyn observer::Observable) {
        println!("update {}!", _originator.get_value().downcast_ref::<u32>().expect("not an u32"));
        
    }
}
