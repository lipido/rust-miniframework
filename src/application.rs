use std::{cell::RefCell, io, rc::Rc};

use crate::observer;

use super::operation;

pub struct Application {
    operations: Vec<Rc<RefCell<dyn operation::Operation>>>,
}

impl Application {
    pub fn from_operations() -> Self {
        
        let app = Application { operations: Vec::new() };
        app
    }


    // Helper method to add a concrete item without forcing clients to use Rc and RefCell
    pub fn add_operation<T: operation::Operation + 'static>(&mut self, item: T) {
        let operation: Rc<RefCell<dyn operation::Operation>> = Rc::new(RefCell::new(item));

        let observer = Box::new(OperationObserver {
            operation: Rc::clone(&operation),
        });

        operation.borrow_mut().add_observer(observer);
        self.operations.push(operation);
    }
    pub fn start(&mut self) {
        
        loop {
            self.menu();
            let option = self.select_option();
            let parameter_values = self.get_parameter_values(option);
            let result = self.operations[option - 1].borrow().run(parameter_values);

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
            println!("{}. {}", i + 1, ele.borrow().display_name());
        }
    }

    fn get_parameter_values(&self, operation_index: usize) -> Vec<String> {
        let operation = self.operations[operation_index - 1].as_ref();

        let mut parameter_values: Vec<String> = Vec::new();
    
        for parameter_name in operation.borrow().parameter_names() {
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
    operation: Rc<RefCell<dyn operation::Operation>>
}
impl observer::Observer for OperationObserver {
    fn update(&self, _originator: &dyn observer::Observable) {
        
        println!("Progress of {}: {}", self.operation.borrow().display_name(), self.operation.borrow().progress());
        
    }
}
