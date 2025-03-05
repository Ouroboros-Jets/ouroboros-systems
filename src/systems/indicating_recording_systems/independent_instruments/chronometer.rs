//The chronometer system is used to display hours and minutes to the
// flight crew.

pub struct Chronometer {
    hours: u32,
    minutes: u32,
}

impl Chronometer {
    pub fn new() -> Chronometer {
        Chronometer {
            hours: 0,
            minutes: 0,
        }
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.hours = hours;
    }

    pub fn get_hours(&self) -> u32 {
        self.hours
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        self.minutes = minutes;
    }

    pub fn get_minutes(&self) -> u32 {
        self.minutes
    }

    pub fn update(&mut self, dt: f32) {
        // set to simulator time
    }
}
