use app::App;

mod renderer;
mod app;
mod vertex;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(App::new(cc))));
}
