use std::f64::consts::PI;
use uom::si::f64::*;
use uom::si::force::newton;
use uom::si::length::millimeter;
use uom::si::mass::kilogram;
use uom::si::pressure::psi;
use uom::si::ratio::ratio;
use uom::si::velocity::meter_per_second;
use uom::si::volume_rate::cubic_meter_per_second;

pub struct HydraulicActuator {
    bore_diameter: Length,
    rod_diameter: Length,
    stroke_length: Length,
    current_position: Length,
    current_velocity: Velocity,
    current_acceleration: Acceleration,

    fluid_bulk_modulus: Pressure,
    fluid_density: MassDensity,
    fluid_viscosity: DynamicViscosity,

    valve_max_flow_rate: VolumeRate,
    valve_opening: Ratio,

    cap_end_pressure: Pressure,
    rod_end_pressure: Pressure,
    cap_end_volume: Volume,
    rod_end_volume: Volume,

    internal_leakage_coefficient: f64,
    external_leakage_coefficient: f64,
    static_friction: Force,
    dynamic_friction_coefficient: f64,

    external_force: Force,
}

impl HydraulicActuator {
    pub fn new(
        bore_diameter: Length,
        rod_diameter: Length,
        stroke_length: Length,
        fluid_bulk_modulus: Pressure,
        fluid_density: MassDensity,
        fluid_viscosity: DynamicViscosity,
        valve_max_flow_rate: VolumeRate,
        static_friction: Force,
        dynamic_friction_coefficient: f64,
        internal_leakage_coefficient: f64,
        external_leakage_coefficient: f64,
    ) -> Self {
        let current_position = Length::new::<millimeter>(0.0);

        let cap_end_area = calculate_area(bore_diameter);
        let rod_end_area = calculate_area(rod_diameter) - calculate_area(rod_diameter);

        let cap_end_volume = cap_end_area * current_position;
        let rod_end_volume = rod_end_area * (stroke_length - current_position);

        let system_pressure = Pressure::new::<psi>(0.0);

        HydraulicActuator {
            bore_diameter,
            rod_diameter,
            stroke_length,
            current_position,
            current_velocity: Velocity::new::<meter_per_second>(0.0),
            current_acceleration: Acceleration::default(),
            fluid_bulk_modulus,
            fluid_density,
            fluid_viscosity,
            valve_max_flow_rate,
            valve_opening: Ratio::new::<ratio>(0.0),
            cap_end_pressure: system_pressure,
            rod_end_pressure: system_pressure,
            cap_end_volume,
            rod_end_volume,
            internal_leakage_coefficient,
            external_leakage_coefficient,
            static_friction,
            dynamic_friction_coefficient,
            external_force: Force::new::<newton>(0.0),
        }
    }

    pub fn set_valve_opening(&mut self, opening: Ratio) {
        self.valve_opening = clamp(opening, Ratio::new::<ratio>(0.0), Ratio::new::<ratio>(1.0));
    }

    pub fn set_external_force(&mut self, force: Force) {
        self.external_force = force;
    }

    pub fn set_supply_pressure(&mut self, pressure: Pressure) {
        //TODO: In the full implementation, this would adjust the pressures based on valve position
        // For simplicity, we're setting it directly
        if self.valve_opening.value > 0.0 {
            self.cap_end_pressure = pressure;
        }
    }

    pub fn update(&mut self, delta_time: Time) {
        let cap_end_area = calculate_area(self.bore_diameter);
        let rod_end_area = calculate_area(self.rod_diameter) - calculate_area(self.rod_diameter);

        let max_flow = self.valve_max_flow_rate * self.valve_opening;

        let delta_p = self.cap_end_pressure - self.rod_end_pressure;

        let internal_leakage = self.internal_leakage_coefficient * delta_p.value;
        let internal_leakage_flow = VolumeRate::new::<cubic_meter_per_second>(internal_leakage);

        let cap_external_leakage = self.external_leakage_coefficient * self.cap_end_pressure.value;
        let rod_external_leakage = self.external_leakage_coefficient * self.rod_end_pressure.value;

        let cap_external_leakage_flow =
            VolumeRate::new::<cubic_meter_per_second>(cap_external_leakage);
        let rod_external_leakage_flow =
            VolumeRate::new::<cubic_meter_per_second>(rod_external_leakage);

        let cap_end_flow = if self.valve_opening.value > 0.0 {
            max_flow
        } else {
            VolumeRate::new::<cubic_meter_per_second>(0.0)
        };
        let rod_end_flow = if self.valve_opening.value < 0.0 {
            -max_flow
        } else {
            VolumeRate::new::<cubic_meter_per_second>(0.0)
        };

        let net_cap_flow = cap_end_flow - internal_leakage_flow - cap_external_leakage_flow;
        let net_rod_flow = rod_end_flow - internal_leakage_flow - rod_external_leakage_flow;

        let cap_pressure_change =
            -(self.fluid_bulk_modulus * (net_cap_flow * delta_time / self.cap_end_volume));
        let rod_pressure_change =
            -(self.fluid_bulk_modulus * (net_rod_flow * delta_time / self.rod_end_volume));

        self.cap_end_pressure = self.cap_end_pressure + cap_pressure_change;
        self.rod_end_pressure = self.rod_end_pressure + rod_pressure_change;

        let cap_force = self.cap_end_pressure * cap_end_area;
        let rod_force = self.rod_end_pressure * rod_end_area;
        let hydraulic_force = cap_force - rod_force;

        let friction_force = if self.current_velocity.value.abs() < 1e-6 {
            clamp(
                hydraulic_force + self.external_force,
                -self.static_friction,
                self.static_friction,
            )
        } else {
            let direction = if self.current_velocity.value > 0.0 {
                1.0
            } else {
                -1.0
            };
            Force::new::<newton>(
                direction * self.dynamic_friction_coefficient * self.current_velocity.value.abs(),
            )
        };

        let net_force = hydraulic_force + self.external_force - friction_force;

        // Acceleration
        // F = ma, for m we will use the effective mass or the piston mass + the fluid mass, which we will have to calculate later ignoring it for now
        // TODO: Replace constant value
        let effective_mass = Mass::new::<kilogram>(1.0);
        self.current_acceleration = net_force / effective_mass;

        // Velocity
        // V = V0 + at
        self.current_velocity = self.current_velocity + self.current_acceleration * delta_time;

        // Position over time
        // X = X0 + vt + 0.5at^2
        let position_change = self.current_velocity * delta_time
            + 0.5 * self.current_acceleration * delta_time * delta_time;
        self.current_position = self.current_position + position_change;

        self.current_position = clamp(
            self.current_position,
            Length::new::<millimeter>(0.0),
            self.stroke_length,
        );

        if (self.current_position.value <= 0.0
            || self.current_position.value >= self.stroke_length.value)
            && ((self.current_position.value <= 0.0 && self.current_velocity.value < 0.0)
                || (self.current_position.value >= self.stroke_length.value
                    && self.current_velocity.value > 0.0))
        // im sorry for this
        {
            self.current_velocity = Velocity::new::<meter_per_second>(0.0);
            self.current_acceleration = Acceleration::default();
        }

        self.cap_end_volume = cap_end_area * self.current_position;
        self.rod_end_volume = rod_end_area * (self.stroke_length - self.current_position);
    }

    pub fn position(&self) -> Length {
        self.current_position
    }

    pub fn velocity(&self) -> Velocity {
        self.current_velocity
    }

    pub fn pressure(&self) -> Pressure {
        self.cap_end_pressure
    }

    pub fn extension_ratio(&self) -> Ratio {
        Ratio::new::<ratio>(self.current_position.value / self.stroke_length.value)
    }
}

// area of a circle given the diameter
fn calculate_area(diameter: Length) -> Area {
    let radius = diameter / 2.0;
    PI * radius * radius
}

fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
