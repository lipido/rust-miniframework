#![deny(elided_lifetimes_in_paths)]

use miniframework::application;
mod calculator;
use calculator::{resta, suma, protected};

fn main() {
    let mut app = application::Application::new();
    app.add_observable_operation(suma::Suma::new());
    app.add_operation(resta::Resta::new());
    app.add_observable_operation(protected::ProtectedOperation::new(suma::Suma::new()));

    app.start();
}
