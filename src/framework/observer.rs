use std::marker::PhantomData;


// Observer design pattern
pub trait Observer<D> {
    fn update(&self, data: &D);
}



pub trait Observable<D, O: Observer<D>>  {
    fn add_observer(&mut self, _observer: O) {
    }
}



pub struct ObserversManager<D, O: Observer<D>> {
    observers: Vec<O>,
    phantom: PhantomData<D>
}

impl <D, O: Observer<D>> ObserversManager<D, O> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            phantom: PhantomData
        }
    }

    pub fn notify_observers(&self, data: &D) {
        for observer in &self.observers {
            observer.update(data)
        }
    }
    pub fn add_observer(&mut self, observer: O) {
        self.observers.push(observer);
    }
}

