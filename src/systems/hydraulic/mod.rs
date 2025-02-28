pub mod components;

use crate::traits::System;

pub struct HydraulicSystem {
    // .... hydraulic cache
}

impl System for HydraulicSystem {
    fn update(&mut self, delta_time: f32) {
        // .... hydraulic update
    }
}

impl HydraulicSystem {
    pub fn new() -> Self {
        Self {
            // .... hydraulic initialization
        }
    }
}
