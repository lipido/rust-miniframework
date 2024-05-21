use miniframework::application;

mod calculator;

use calculator::{resta, suma, protected};

fn main() {
    let mut app = application::Application::from_operations(vec![
        Box::new(suma::Suma::new()),
        Box::new(protected::ProtectedOperation::new(protected::ProtectedOperation::new(resta::Resta::new()))),
    ]);
    app.start();
}
