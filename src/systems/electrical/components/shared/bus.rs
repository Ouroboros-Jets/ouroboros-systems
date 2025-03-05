use crate::systems::electrical::ElectricalComponent;

use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::power::watt;

pub struct Bus {
    pub(crate) voltage: ElectricPotential,
    pub(crate) power: Power,
}

impl ElectricalComponent for Bus {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(&mut self, _dt: f32) {
        // TODO: we will just avoid power loss in the bus and just distribute power instead.
    }

    fn get_output_power(&self) -> Power {
        self.power
    }

    fn set_input_power(&mut self, power: Power) {
        self.power = power;
    }
    fn get_output_voltage(&self) -> ElectricPotential {
        self.voltage
    }

    fn set_input_voltage(&mut self, voltage: ElectricPotential) {
        self.voltage = voltage;
    }

    fn set_input_current(&mut self, _current: ElectricCurrent) {}
}
