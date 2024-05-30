
// Observer design pattern
pub trait Observer<T> {
    fn update(&self, data: &T);
}



pub trait Observable<T, O: Observer<T>>  {
    fn add_observer(&mut self, _observer: O) {
    }
}



pub struct ObserversManager<T> {
    observers: Vec<Box<dyn Observer<T>>>,
}

impl <T> ObserversManager<T> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn notify_observers(&self, data: &T) {
        for observer in &self.observers {
            observer.update(data)
        }
    }
    pub fn add_observer(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers.push(observer);
    }
}

