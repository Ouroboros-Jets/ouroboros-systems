use crate::systems::electrical::ElectricalComponent;
use std::any::Any;

use uom::si::angular_velocity::revolution_per_minute;
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::electrical_resistance::ohm;
use uom::si::f64::*;
use uom::si::frequency::hertz;
use uom::si::power::watt;
use uom::si::ratio::ratio;
use uom::si::time::millisecond;

pub struct Generator {
    num_poles: f64,
    rated_power: Power,                        //Watts
    rated_voltage: ElectricPotential,          // Volt
    rated_frequency: Frequency,                // Hz
    efficiency: Ratio,                         // Percent
    internal_resistance: ElectricalResistance, // Ohm
    mechanical_input_power: Power,             //Watt
    rpm: AngularVelocity,                      // RPM
    output_power: Power,                       // Watts
    output_voltage: ElectricPotential,         // Volts
    spin_up_time: Time,                        // Ms
    current_rpm: AngularVelocity,              // RPM
    is_on: bool,
    time_on: Time, // Ms
    phase_count: u8,
}

impl Generator {
    pub fn new(
        num_poles: f64,
        rated_power: f64,
        rated_voltage: f64,
        rated_frequency: f64,
        efficiency: f64,
        internal_resistance: f64,
        spin_up_time: f64,
        phase_count: u8,
    ) -> Self {
        Self {
            num_poles,
            rated_power: Power::new::<watt>(rated_power),
            rated_voltage: ElectricPotential::new::<volt>(rated_voltage),
            rated_frequency: Frequency::new::<hertz>(rated_frequency),
            efficiency: Ratio::new::<ratio>(efficiency * 100.0),
            internal_resistance: ElectricalResistance::new::<ohm>(internal_resistance),
            mechanical_input_power: Power::new::<watt>(0.0),
            rpm: AngularVelocity::new::<revolution_per_minute>(rated_frequency * 60.0 / num_poles),
            output_power: Power::new::<watt>(0.0),
            output_voltage: ElectricPotential::new::<volt>(0.0),
            spin_up_time: Time::new::<millisecond>(spin_up_time),
            current_rpm: AngularVelocity::new::<revolution_per_minute>(0.0),
            is_on: false,
            time_on: Time::new::<millisecond>(0.0),
            phase_count,
        }
    }

    pub fn set_mechanical_input(&mut self, power: f64, rpm: f64) {
        if self.is_on {
            self.mechanical_input_power = Power::new::<watt>(power);
            self.rpm = AngularVelocity::new::<revolution_per_minute>(rpm);
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        self.time_on = Time::new::<millisecond>(0.0);
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.output_voltage = ElectricPotential::new::<volt>(0.0);
        self.output_power = Power::new::<watt>(0.0);
        self.current_rpm = AngularVelocity::new::<revolution_per_minute>(0.0);
    }
}

impl ElectricalComponent for Generator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn update(&mut self, dt: f32) {
        if !self.is_on {
            self.output_power = Power::new::<watt>(0.0);
            self.output_voltage = ElectricPotential::new::<volt>(0.0);
            self.current_rpm = AngularVelocity::new::<revolution_per_minute>(0.0);
            return;
        }
        println!("Generator is on");
        self.time_on += Time::new::<millisecond>(dt as f64);

        let spin_progress = if self.spin_up_time.get::<millisecond>() > 0.0 {
            (self.time_on.get::<millisecond>() / self.spin_up_time.get::<millisecond>() * 1000.0)
                .min(1.0)
        } else {
            1.0
        };

        self.current_rpm = AngularVelocity::new::<revolution_per_minute>(
            self.rpm.get::<revolution_per_minute>() * spin_progress,
        );

        let expected_rpm = self.rated_frequency.get::<hertz>() * 60.0 / self.num_poles;
        let efficiency_factor = if self.current_rpm.get::<revolution_per_minute>() >= expected_rpm {
            self.efficiency.get::<ratio>()
        } else {
            self.efficiency.get::<ratio>()
                * (self.rpm.get::<revolution_per_minute>() / expected_rpm)
        };

        let available_electrical_power =
            self.mechanical_input_power.get::<watt>() * efficiency_factor;
        self.output_power =
            Power::new::<watt>(available_electrical_power.min(self.rated_power.get::<watt>()));

        let current = self.output_power.get::<watt>() / self.rated_voltage.get::<volt>()
            * self.phase_count as f64;
        let voltage_drop = current * self.internal_resistance.get::<ohm>();
        self.output_voltage = ElectricPotential::new::<volt>(
            (self.rated_voltage.get::<volt>() - voltage_drop).max(0.0),
        );
    }

    fn get_output_power(&self) -> Power {
        self.output_power
    }

    fn set_input_power(&mut self, _power: Power) {
        // Generators do not take input power
    }

    fn get_output_voltage(&self) -> ElectricPotential {
        self.output_voltage
    }

    fn set_input_voltage(&mut self, _voltage: ElectricPotential) {
        // Generators don't take input, this will just be generated in the update
    }

    fn get_output_current(&self) -> ElectricCurrent {
        if self.output_voltage.value > 0.0 {
            ElectricCurrent::new::<ampere>(self.output_power.value / self.output_voltage.value)
        } else {
            ElectricCurrent::new::<ampere>(0.0)
        }
    }

    fn set_input_current(&mut self, _current: ElectricCurrent) {
        // No need to set on generator
    }
}
