use crate::systems::electrical::components::{
    ac::generator::Generator,
    dc::generic_dc_component::{GenericDcComponent, VoltageResponse},
    shared::bus::Bus,
    shared::circuit_breaker::{CircuitBreaker, TripCurve},
};
use crate::systems::electrical::{ElectricalComponent, ElectricalComponentExt, ElectricalSystem};
use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::power::watt;
pub mod air_conditioning;
pub mod airborne_auxiliary_power;
pub mod auto_flight;
pub mod cabin_systems;
pub mod central_maintenance_system;
pub mod common;
pub mod communications;
pub mod electrical;
pub mod engine;
pub mod fire_protection;
pub mod flight_controls;
pub mod fuel;
pub mod hydraulic;
pub mod ice_and_rain_protection;
pub mod indicating_recording_systems;
pub mod lights;
pub mod navigation;
pub mod oxygen;
pub mod pneumatic;
pub mod water_waste;

// We will construct the entire aircraft from here.
// I constructed this outside of the main loop so any value inside this struct will be preserved between frames.
pub struct E170Systems {
    electrical_system: ElectricalSystem,
    elapsed_time: f32,
    generator_node: petgraph::graph::NodeIndex,
    generator_on: bool,
}

impl E170Systems {
    pub fn new() -> E170Systems {
        let mut electrical_system = ElectricalSystem::new();

        // Test electrical system construction

        let main_generator = Generator::new(2.0, 90000.0, 115.0, 400.0, 0.95, 0.05, 0.0, 3);
        let generator_node = electrical_system.add_component("Main Generator", main_generator);

        let main_bus = Bus {
            voltage: ElectricPotential::new::<volt>(28.0),
            power: Power::new::<watt>(0.0),
        };

        let main_bus_node = electrical_system.add_component("Main Bus", main_bus);

        electrical_system.connect_no_resistance(generator_node, main_bus_node);

        let avionics_cb =
            CircuitBreaker::new("Avionics CB", 15.0, TripCurve::ShortDelay(0.2), false, 0.0);
        let avionics_cb_node = electrical_system.add_component("Avionics CB", avionics_cb);

        let lights_cb =
            CircuitBreaker::new("Lights CB", 10.0, TripCurve::ShortDelay(0.1), true, 5.0);

        let lights_cb_node = electrical_system.add_component("Lights CB", lights_cb);

        electrical_system.connect_no_resistance(main_bus_node, avionics_cb_node);
        electrical_system.connect_no_resistance(main_bus_node, lights_cb_node);

        let test_display = GenericDcComponent::new(
            "Test Display",
            28.0,
            120.0,
            21.0,
            32.0,
            VoltageResponse::Regulated,
            0.85,
        );

        let test_display_node = electrical_system.add_component("Test Display", test_display);

        let test_light = GenericDcComponent::new(
            "Test Light",
            28.0,
            200.0,
            20.0,
            32.0,
            VoltageResponse::Binary,
            0.9,
        );

        let test_light_node = electrical_system.add_component("Test Light", test_light);

        electrical_system.connect_with_wire(avionics_cb_node, test_display_node, 0.01);
        electrical_system.connect_with_wire(lights_cb_node, test_light_node, 0.02);

        E170Systems {
            electrical_system,
            elapsed_time: 0.0,
            generator_node,
            generator_on: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;
        if !self.generator_on && self.elapsed_time > 3.0 {
            if let Some(component) = self
                .electrical_system
                .components
                .get_mut(&self.generator_node)
            {
                if let Some(generator) = component.downcast_mut::<Generator>() {
                    generator.turn_on();
                    generator.set_mechanical_input(80000.0, 6000.0); // Set some power and RPM
                    println!(
                        "🔌 Generator turned ON after {} seconds",
                        self.elapsed_time / 1000.0
                    );
                    self.generator_on = true;
                }
            }
        }
        self.electrical_system.update_system(dt);

        let overcurrents = self.electrical_system.check_overcurrent(20.0);
        if !overcurrents.is_empty() {
            println!(
                "\n⚠️ OVERCURRENT DETECTED in {} connections:",
                overcurrents.len()
            );
            for (from, to, current) in &overcurrents {
                let from_name = self.electrical_system.graph.node_weight(*from).unwrap();
                let to_name = self.electrical_system.graph.node_weight(*to).unwrap();
                println!(
                    "  - {from_name} → {to_name}: {:.2} A (EXCESSIVE!)",
                    current.value
                );
            }
        }
    }
}
