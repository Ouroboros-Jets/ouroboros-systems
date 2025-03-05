pub mod components;

use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::IntoNeighbors;
use std::any::Any;
use std::collections::HashMap;
use std::path::Component;
use uom::si::electric_current::ampere;
use uom::si::electrical_resistance::ohm;
use uom::si::f64::*;

pub trait ElectricalComponent: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn update(&mut self, dt: f32);
    fn get_output_power(&self) -> Power;
    fn set_input_power(&mut self, power: Power);
    fn get_output_voltage(&self) -> ElectricPotential;
    fn set_input_voltage(&mut self, voltage: ElectricPotential);
    fn get_output_current(&self) -> ElectricCurrent {
        let voltage = self.get_output_voltage();
        if voltage.value > 0.0 {
            ElectricCurrent::new::<ampere>(self.get_output_power().value / voltage.value)
        } else {
            ElectricCurrent::new::<ampere>(0.0)
        }
    }
    fn get_input_current(&self) -> ElectricCurrent {
        self.get_output_current()
    }
    fn set_input_current(&mut self, current: ElectricCurrent);
}

pub trait ElectricalComponentExt {
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
}

impl ElectricalComponentExt for Box<dyn ElectricalComponent> {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (**self).as_any().downcast_ref::<T>()
    }

    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (**self).as_any_mut().downcast_mut::<T>()
    }
}

pub struct ElectricalSystem {
    pub(crate) graph: DiGraph<String, ()>,
    pub(crate) components: HashMap<NodeIndex, Box<dyn ElectricalComponent>>,
    node_voltage: HashMap<NodeIndex, ElectricPotential>,
    pub(crate) edge_current: HashMap<(NodeIndex, NodeIndex), ElectricCurrent>,
    wire_resistance: HashMap<(NodeIndex, NodeIndex), ElectricalResistance>,
}

impl ElectricalSystem {
    pub fn new() -> Self {
        ElectricalSystem {
            graph: DiGraph::new(),
            components: HashMap::new(),
            node_voltage: HashMap::new(),
            edge_current: HashMap::new(),
            wire_resistance: HashMap::new(),
        }
    }
    pub fn add_component<C: ElectricalComponent + 'static>(
        &mut self,
        name: &str,
        component: C,
    ) -> NodeIndex {
        let node = self.graph.add_node(name.to_string());
        self.components.insert(node, Box::new(component));
        node
    }

    pub fn connect_with_wire(&mut self, from: NodeIndex, to: NodeIndex, resistance: f64) {
        self.graph.add_edge(from, to, ());
        self.edge_current
            .insert((from, to), ElectricCurrent::new::<ampere>(0.0));
        self.wire_resistance
            .insert((from, to), ElectricalResistance::new::<ohm>(resistance));
    }

    pub fn connect_no_resistance(&mut self, from: NodeIndex, to: NodeIndex) {
        self.connect_with_wire(from, to, 0.001);
    }

    pub fn calculate_current_flow(&mut self) {
        for edge in self.graph.edge_indices() {
            if let Some((from, to)) = self.graph.edge_endpoints(edge) {
                if let (Some(from_comp), Some(to_comp)) =
                    (self.components.get(&from), self.components.get(&to))
                {
                    let v_from = from_comp.get_output_voltage();
                    let v_to = to_comp.get_output_voltage();
                    let voltage_diff = v_from - v_to;

                    if let Some(resistance) = self.wire_resistance.get(&(from, to)) {
                        let current = if resistance.value > 0.0 {
                            ElectricCurrent::new::<ampere>(voltage_diff.value / resistance.value)
                        } else {
                            ElectricCurrent::new::<ampere>(0.0)
                        };

                        self.edge_current.insert((from, to), current);
                    }
                }
            }
        }
    }

    pub fn update_system(&mut self, dt: f32) {
        if let Ok(sorted_nodes) = toposort(&self.graph, None) {
            for node in &sorted_nodes {
                if let Some(component) = self.components.get_mut(node) {
                    component.update(dt);
                    let output_voltage = component.get_output_voltage();
                    self.node_voltage.insert(*node, output_voltage);
                }
            }

            self.calculate_current_flow();

            for node in sorted_nodes {
                if let Some(component) = self.components.get_mut(&node) {
                    let output_voltage = component.get_output_voltage();
                    let output_power = component.get_output_power();

                    for neighbor in self.graph.neighbors(node) {
                        if let Some(neighbor_component) = self.components.get_mut(&neighbor) {
                            neighbor_component.set_input_voltage(output_voltage);
                            neighbor_component.set_input_power(output_power);

                            if let Some(current) = self.edge_current.get(&(node, neighbor)) {
                                neighbor_component.set_input_current(*current);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_current(&self, from: NodeIndex, to: NodeIndex) -> Option<ElectricCurrent> {
        self.edge_current.get(&(from, to)).copied()
    }

    pub fn check_overcurrent(
        &self,
        max_current: f64,
    ) -> Vec<(NodeIndex, NodeIndex, ElectricCurrent)> {
        let mut overcurrents = Vec::new();

        for ((from, to), current) in &self.edge_current {
            if current.value > max_current {
                overcurrents.push((*from, *to, *current));
            }
        }
        overcurrents
    }
}
