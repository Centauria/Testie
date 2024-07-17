pub mod main;
mod settings;

trait Window {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

trait View {
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}