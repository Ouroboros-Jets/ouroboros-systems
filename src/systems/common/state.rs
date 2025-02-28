use uom::si::volume::liter;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::pressure::psi;

#[derive(Debug)]
pub struct SystemState {

}
#[derive(Debug)]
pub struct FlightDeckState {

}
#[derive(Debug)]
pub struct ElectricalSystem {

}

#[derive(Debug)]
pub struct HydraulicSystem {
}

#[derive(Debug)]
pub struct HydraulicSystem1 {
    reservoir_volume: liter,
    engine_driven_pump_rpm: revolution_per_minute,
    pre_manifold_pressure: psi,
    post_manifold_pressure: psi,
    // ...
}