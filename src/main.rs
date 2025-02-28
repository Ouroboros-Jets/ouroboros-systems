use entry_point::entry_point;

pub mod communication_bus;
pub mod entry_point;
pub mod macros;
pub mod systems;
pub mod traits;
pub mod utils;

fn main() {
    entry_point();
}
