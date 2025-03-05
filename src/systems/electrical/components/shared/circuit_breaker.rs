use crate::systems::electrical::ElectricalComponent;

use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::power::watt;
use uom::si::time::second;

pub enum TripCurve {
    Instantaneous,
    ShortDelay(f64), // Constant delay in seconds float
    LongDelay(f64),  // Constant delay in seconds float
    InverseTime,     // Trip time inversely proportional to current
}

pub struct CircuitBreaker {
    name: String,
    rating: ElectricCurrent,
    is_tripped: bool,
    input_voltage: ElectricPotential,
    input_power: Power,
    input_current: ElectricCurrent,
    trip_curve: TripCurve,
    overcurrent_time: f64, // time spent in overcurrent state
    auto_reset: bool,
    reset_delay: f64, // time to wait before resetting
    trip_time: f64,   // time spent in tripped state
}

impl CircuitBreaker {
    pub fn new(
        name: &str,
        rating_amps: f64,
        trip_curve: TripCurve,
        auto_reset: bool,
        reset_delay: f64,
    ) -> Self {
        CircuitBreaker {
            name: name.to_string(),
            rating: ElectricCurrent::new::<ampere>(rating_amps),
            is_tripped: false,
            input_voltage: ElectricPotential::new::<volt>(0.0),
            input_power: Power::new::<watt>(0.0),
            input_current: ElectricCurrent::new::<ampere>(0.0),
            trip_curve,
            overcurrent_time: 0.0,
            auto_reset,
            reset_delay,
            trip_time: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.is_tripped = false;
        self.trip_time = 0.0;
        self.overcurrent_time = 0.0;
    }

    pub fn is_tripped(&self) -> bool {
        self.is_tripped
    }

    pub fn should_trip(&self, dt: f32) -> bool {
        if self.input_current.value <= self.rating.value {
            return false;
        }

        match self.trip_curve {
            TripCurve::Instantaneous => true,
            TripCurve::ShortDelay(delay) => self.overcurrent_time >= delay,
            TripCurve::LongDelay(delay) => self.overcurrent_time >= delay,
            TripCurve::InverseTime => {
                let overload_ratio = self.input_current.value / self.rating.value;
                self.overcurrent_time >= 0.1 / (overload_ratio * overload_ratio)
            }
        }
    }
}

impl ElectricalComponent for CircuitBreaker {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn update(&mut self, dt: f32) {
        let dt_seconds: f64 = dt as f64 / 1000.0;

        if self.is_tripped {
            if self.auto_reset {
                self.trip_time += dt_seconds;
                if self.trip_time >= self.reset_delay {
                    self.reset();
                }
            }
            return;
        }
        if self.input_current.value > self.rating.value {
            self.overcurrent_time += dt_seconds;
        } else {
            self.overcurrent_time = 0.0;
        }
        if self.should_trip(dt) {
            self.is_tripped = true;
            self.trip_time = 0.0;
        }
    }

    fn get_output_power(&self) -> Power {
        if self.is_tripped {
            Power::new::<watt>(0.0)
        } else {
            self.input_power
        }
    }

    fn set_input_power(&mut self, input_power: Power) {
        self.input_power = input_power;
    }

    fn get_output_voltage(&self) -> ElectricPotential {
        if self.is_tripped {
            ElectricPotential::new::<volt>(0.0)
        } else {
            self.input_voltage
        }
    }

    fn set_input_voltage(&mut self, input_voltage: ElectricPotential) {
        self.input_voltage = input_voltage;
    }

    fn get_output_current(&self) -> ElectricCurrent {
        if self.is_tripped {
            ElectricCurrent::new::<ampere>(0.0)
        } else {
            self.input_current
        }
    }
    fn set_input_current(&mut self, current: ElectricCurrent) {
        self.input_current = current;
    }
}
