pub trait System {
    fn update(&mut self, delta_time: f32);
}

pub struct SystemContainer<T: System> {
    component: T,
}

impl<T: System> SystemContainer<T> {
    pub fn new(component: T) -> Self {
        Self { component }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.component.update(delta_time);
    }
}
