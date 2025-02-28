use std::time::{Duration, Instant};

pub struct DeltaTime {
    last_time: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }

    pub fn update_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;
        delta.as_secs_f32()
    }
}
