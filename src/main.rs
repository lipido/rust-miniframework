use miniframework::application;

mod calculator;

use calculator::{resta, suma, protected};

fn main() {
    // let mut app = application::Application::from_operations(vec![
    //     Box::new(suma::Suma::new()),
    //     Box::new(protected::ProtectedOperation::new(protected::ProtectedOperation::new(resta::Resta::new()))),
    // ]);
    let mut app = application::Application::new();
    
    app.add_observable_operation(suma::Suma::new());
    app.add_operation(resta::Resta::new());
    app.add_observable_operation(protected::ProtectedOperation::new(suma::Suma::new()));

    app.start();
}
