use std::marker::PhantomData;


// Observer design pattern
pub trait Observer<T> {
    fn update(&self, data: &T);
}



pub trait Observable<T, O: Observer<T>>  {
    fn add_observer(&mut self, _observer: O) {
    }
}

impl<T> Observer<T> for Box<dyn Observer<T>> {
    fn update(&self, data: &T) {
        (**self).update(data);
    }
}

pub struct ObserversManager<T, O: Observer<T>> {
    observers: Vec<O>,
    phantom: PhantomData<T>
}

impl <T, O: Observer<T>> ObserversManager<T, O> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            phantom: PhantomData
        }
    }

    pub fn notify_observers(&self, data: &T) {
        for observer in &self.observers {
            observer.update(data)
        }
    }
    pub fn add_observer(&mut self, observer: O) {
        self.observers.push(observer);
    }
}

