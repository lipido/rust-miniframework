// Proxy design pattern with generic and trait bounds
use miniframework::operation;
use miniframework::observer;
pub struct ProtectedOperation<O: operation::Operation> {
    base: O,
    name: String
}

impl<O: operation::Operation> ProtectedOperation<O> {
    pub fn new(base: O) -> Self {
        let mut res = ProtectedOperation {
            base,
            name: String::new()
        };

        res.name = res.base.display_name().to_string() + " (premium)";

        res
    }
    fn is_full_version() -> bool {
        true
    }
}
impl <O: operation::Operation> operation::Operation for ProtectedOperation<O> {
    fn display_name(&self) -> &str {
        &self.name
    }

    fn parameter_names(&self) -> &Vec<String> {
        self.base.parameter_names()
    }
    
    fn progress(&self) -> f32 {
        self.base.progress()
    }

    fn run(&self, parameter_values: Vec<String>) -> String {
        if ProtectedOperation::<O>::is_full_version() {
            self.base.run(parameter_values)
        } else {
            return String::from("Not in full version")
        }
    }
    
    
}
impl <O: operation::Operation + observer::Observable> observer::Observable for ProtectedOperation<O>  {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer>) {
        self.base.add_observer(observer);
    }
}