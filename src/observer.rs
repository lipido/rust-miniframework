
// Observer design pattern
pub trait Observer {
    fn update(&self, originator: &dyn Observable);
}


pub trait Observable  {
    fn add_observer(&mut self, _observer: Box<dyn Observer>) {

    }

    // fn get_value(&self) -> &dyn Any {
    //     panic!("ey");
    // }
}



pub struct ObserversManager {
    observers: Vec<Box<dyn Observer>>,
}

impl ObserversManager {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn notify_observers(&self, originator: &dyn Observable) {
        for observer in &self.observers {
            observer.update(originator)
        }
    }
    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
}

