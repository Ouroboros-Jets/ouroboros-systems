use crate::systems::electrical::ElectricalComponent;

use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::electrical_resistance::ohm;
use uom::si::f64::*;
use uom::si::power::watt;

pub enum VoltageResponse {
    Linear,
    Binary,
    Regulated,
    Proportional,
}

pub struct GenericDcComponent {
    name: String,
    nominal_voltage: ElectricPotential,
    nominal_power: Power,
    resistance: ElectricalResistance,
    min_voltage: ElectricPotential,
    max_voltage: ElectricPotential,
    voltage_response: VoltageResponse,
    power_factor: f64,

    pub(crate) input_voltage: ElectricPotential,
    input_power: Power,
    input_current: ElectricCurrent,

    is_on: bool,
    load_factor: f64, // could be useful for dimming lights (non displays)
}

impl GenericDcComponent {
    pub fn new(
        name: &str,
        nominal_voltage: f64,
        nominal_power: f64,
        min_voltage: f64,
        max_voltage: f64,
        voltage_response: VoltageResponse,
        power_factor: f64,
    ) -> Self {
        let nom_voltage = ElectricPotential::new::<volt>(nominal_voltage);
        let nom_power = Power::new::<watt>(nominal_power);

        let resistance = if nominal_power > 0.0 && nominal_voltage > 0.0 {
            ElectricalResistance::new::<ohm>((nominal_voltage * nominal_voltage) / nominal_power)
        } else {
            ElectricalResistance::new::<ohm>(f64::INFINITY)
        };

        GenericDcComponent {
            name: name.to_string(),
            nominal_voltage: nom_voltage,
            nominal_power: nom_power,
            resistance,
            min_voltage: ElectricPotential::new::<volt>(min_voltage),
            max_voltage: ElectricPotential::new::<volt>(max_voltage),
            voltage_response,
            power_factor: power_factor.clamp(0.0, 1.0),

            input_voltage: ElectricPotential::new::<volt>(0.0),
            input_power: Power::new::<watt>(0.0),
            input_current: ElectricCurrent::new::<ampere>(0.0),

            is_on: false,
            load_factor: 1.0,
        }
    }

    pub fn set_power_state(&mut self, on: bool) {
        self.is_on = on;
    }

    pub fn set_load_factor(&mut self, factor: f64) {
        self.load_factor = factor.clamp(0.0, 1.0);
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn get_actual_power(&self) -> Power {
        match self.voltage_response {
            VoltageResponse::Binary => {
                if self.input_voltage.value >= self.min_voltage.value && self.is_on {
                    self.nominal_power * self.load_factor * self.power_factor
                } else {
                    Power::new::<watt>(0.0)
                }
            }
            VoltageResponse::Linear => {
                if !self.is_on || self.input_voltage.value < self.min_voltage.value {
                    Power::new::<watt>(0.0)
                } else {
                    let voltage_ratio = self.input_voltage.value / self.nominal_voltage.value;
                    self.nominal_power * voltage_ratio * self.load_factor * self.power_factor
                }
            }
            VoltageResponse::Regulated => {
                if !self.is_on || self.input_voltage.value < self.min_voltage.value {
                    Power::new::<watt>(0.0)
                } else {
                    self.nominal_power * self.load_factor * self.power_factor
                }
            }
            VoltageResponse::Proportional => {
                if !self.is_on || self.input_voltage.value < self.min_voltage.value {
                    Power::new::<watt>(0.0)
                } else {
                    let voltage_factor = ((self.input_voltage.value - self.min_voltage.value)
                        / (self.nominal_voltage.value - self.min_voltage.value))
                        .min(1.0);
                    self.nominal_power * voltage_factor * self.power_factor * self.load_factor
                }
            }
        }
    }
}

impl ElectricalComponent for GenericDcComponent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn update(&mut self, dt: f32) {
        if self.is_on {
            if self.input_voltage.value > self.max_voltage.value {
                println!(
                    "⚠️ OVERVOLTAGE: {}V > {}V max for {}",
                    self.input_voltage.value, self.max_voltage.value, self.name
                );
            } else if self.input_voltage.value < self.min_voltage.value {
                println!(
                    "⚠️ UNDERVOLTAGE: {}V < {}V min for {}",
                    self.input_voltage.value, self.min_voltage.value, self.name
                );
            }

            println!(
                "💡 {} consuming {:.1}W at {:.1}V",
                self.name,
                self.get_actual_power().value,
                self.input_voltage.value
            );
        }
    }

    fn get_output_power(&self) -> Power {
        Power::new::<watt>(0.0)
    }

    fn set_input_power(&mut self, power: Power) {
        self.input_power = power;
    }

    fn get_output_voltage(&self) -> ElectricPotential {
        ElectricPotential::new::<volt>(0.0)
    }

    fn set_input_voltage(&mut self, voltage: ElectricPotential) {
        self.input_voltage = voltage;
    }

    fn get_output_current(&self) -> ElectricCurrent {
        ElectricCurrent::new::<ampere>(0.0)
    }

    fn get_input_current(&self) -> ElectricCurrent {
        if self.input_voltage.value > 0.0 {
            let actual_power = self.get_actual_power();
            ElectricCurrent::new::<ampere>(actual_power.value / self.input_voltage.value)
        } else {
            ElectricCurrent::new::<ampere>(0.0)
        }
    }

    fn set_input_current(&mut self, current: ElectricCurrent) {
        self.input_current = current;
    }
}
