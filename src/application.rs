use std::io;

use crate::{observer, operation::ObservableOperation};

use super::operation;



pub struct Application<'a> {
    operations: Vec<Box<dyn operation::Operation +'a>>,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        let app = Application {
            operations: Vec::new(),
        };
        app
    }

    pub fn add_operation<T: operation::Operation + 'a>(&mut self, item: T) {
        let operation = Box::new(item);
        self.operations.push(operation);
    }

    pub fn add_observable_operation<
        T: operation::ObservableOperation<Box<dyn observer::Observer<f32>>> +'a,
    >(
        &mut self,
        item: T,
    ) {
        let mut operation = Box::new(item);

        OperationObserver::observe(&mut operation);

        self.operations.push(operation);
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

struct OperationObserver {}
impl OperationObserver {
    fn observe<T: ObservableOperation<Box<dyn observer::Observer<f32>>>>(
        operation: &mut Box<T>,
    ) {
        let observer = Box::new(OperationObserver {});
        operation.add_observer(observer);
    }
}
impl observer::Observer<f32> for OperationObserver {
    fn update(&self, data: &f32) {
        println!("Progress {}", data);
    }
}
