// Proxy design pattern with generic and trait bounds
use miniframework::operation;
use miniframework::observer;
use miniframework::operation::ObservableOperation;
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
        false
    }
}
impl <O: operation::Operation> operation::Operation for ProtectedOperation<O> {
    fn display_name(&self) -> &str {
        &self.name
    }

    fn parameter_names(&self) -> &Vec<String> {
        self.base.parameter_names()
    }
    

    fn run(&self, parameter_values: Vec<String>) -> String {
        if ProtectedOperation::<O>::is_full_version() {
            self.base.run(parameter_values)
        } else {
            return String::from("Not in full version")
        }
    }
    
    
}
impl<O: ObservableOperation<Box<dyn observer::Observer<f32>>>> observer::Observable<f32, Box<dyn observer::Observer<f32>>> for ProtectedOperation<O>  {
    fn add_observer(&mut self, observer: Box<dyn observer::Observer<f32>
        >) {
        self.base.add_observer(observer);
    }
}

impl<O: ObservableOperation<Box<dyn observer::Observer<f32>>>> ObservableOperation<Box<dyn observer::Observer<f32>>> for ProtectedOperation<O> {}