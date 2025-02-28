use eframe::egui;

use crate::if_gui;
use crate::traits::SystemContainer;
use crate::utils::delta_time::DeltaTime;

pub fn entry_point() {
    env_logger::init();
    if_gui!({
        use eframe::egui;
        use std::thread;



        println!("Simulation starting...");

        thread::spawn(|| {
            external_entry_point();
        });

        println!("Simulation running, starting GUI...");

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(), ..Default::default()
        };

       eframe::run_native("E-170 Systems", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Ok(Box::<GuiState>::default())
       }))

    }else {
       println!("Gui is disabled");
    });
}

fn external_entry_point() {
    // we will create a simulation loop here since we are running outside the Simulator
    use std::time::Duration;

    let mut delta_time = DeltaTime::new();

    let mut hydraulic_system =
        SystemContainer::new(crate::systems::hydraulic::HydraulicSystem::new());

    loop {
        // we will first fetch the simulation data and update our state

        // then we will simulate the next tick
        let delta = delta_time.update_time();
        print!("\x1B[2J\x1B[1;1H");
        println!("Delta time: {}", &delta);

        hydraulic_system.update(delta);

        // finally we will update the simulation data using the new state, this will allow for a single threaded simulation
        // if we want to use the mt simulation, we just need to have a simulation data writer/reader thread, simulation thread and a communication method (like a channel or a bus)

        // we will sleep for 16ms to simulate a 60fps loop
        std::thread::sleep(Duration::from_millis(16));
    }
}

struct GuiState {
    page: Page,
}

enum Page {
    Home,
    Electrical,
    Hydraulic,
    Fuel,
    Engine,
    BleedAir,
    APU,
    Pressurization,
}

impl Default for GuiState {
    fn default() -> Self {
        Self { page: Page::Home }
    }
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("E170 Systems");
            ui.add(
                egui::Image::new(egui::include_image!("../../assets/ouroboros.png"))
                    .max_width(400.0),
            );

            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.page = Page::Home;
                }
                if ui.button("Electrical").clicked() {
                    self.page = Page::Electrical;
                }
                if ui.button("Hydraulic").clicked() {
                    self.page = Page::Hydraulic;
                }
                if ui.button("Fuel").clicked() {
                    self.page = Page::Fuel;
                }
                if ui.button("Engine").clicked() {
                    self.page = Page::Engine;
                }
                if ui.button("Bleed Air").clicked() {
                    self.page = Page::BleedAir;
                }
                if ui.button("APU").clicked() {
                    self.page = Page::APU;
                }
                if ui.button("Pressurization").clicked() {
                    self.page = Page::Pressurization;
                }
            });

            match self.page {
                Page::Home => {
                    ui.label("Welcome to the E170 Systems");
                }
                Page::Electrical => {
                    ui.label("Electrical");
                }
                Page::Hydraulic => {
                    ui.label("Hydraulic");
                }
                Page::Fuel => {
                    ui.label("Fuel");
                }
                Page::Engine => {
                    ui.label("Engine");
                }
                Page::BleedAir => {
                    ui.label("Bleed Air");
                }
                Page::APU => {
                    ui.label("APU");
                }
                Page::Pressurization => {
                    ui.label("Pressurization");
                }
            }
        });
    }
}
