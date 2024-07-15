use eframe::Frame;
use egui::Context;

struct Settings {
    sr: i32,
    bpm: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            sr: 44100,
            bpm: 120.0,
        }
    }
}
#[derive(Default)]
pub struct Main {
    settings: Settings,
}

impl eframe::App for Main {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mic Frequency Response Estimation");
            ui.add(egui::Slider::new(&mut self.settings.bpm, 60.0..=240.0).text("BPM"));
            egui::ComboBox::from_label("Samplerate")
                .selected_text(format!("{:?}", self.settings.sr))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.sr, 44100, "44100");
                    ui.selectable_value(&mut self.settings.sr, 48000, "48000")
                });
            ui.label(format!("BPM {}, sample rate {}", self.settings.bpm, self.settings.sr));
        });
    }
}