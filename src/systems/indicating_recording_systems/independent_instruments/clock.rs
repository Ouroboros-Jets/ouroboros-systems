//The clock system is used during pre-flight and in-flight activities to
// provide the flight crew with the UTC (Universal Time Coordinated), ET
// (Elapsed Time), and CHR (Chrono Time) functions.

pub enum ClockMode {
    UTC,
    ET,
    CHR,
}

pub struct Clock {
    mode: ClockMode,
    time: u32,
    utc: u32,
    et: u32,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            mode: ClockMode::UTC,
            time: 0,
            utc: 0,
            et: 0,
        }
    }

    pub fn set_mode(&mut self, mode: ClockMode) {
        self.mode = mode;
    }

    pub fn set_time(&mut self, time: u32) {
        self.time = time;
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }

    pub fn set_utc(&mut self, utc: u32) {
        self.utc = utc;
    }

    pub fn get_utc(&self) -> u32 {
        self.utc
    }

    pub fn set_et(&mut self, et: u32) {
        self.et = et;
    }

    pub fn get_et(&self) -> u32 {
        self.et
    }

    pub fn update(&mut self, dt: f32) {
        match self.mode {
            ClockMode::UTC => {
                // TODO: Grab from simulator
                self.time += dt as u32;
                self.utc = self.time;
            }
            ClockMode::ET => {
                self.time += dt as u32;
                self.et = self.time;
            }
            ClockMode::CHR => {
                // TODO: Grab from simulator
                self.time += dt as u32;
            }
        }
    }
}
