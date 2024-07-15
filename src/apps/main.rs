use eframe::Frame;
use egui::Context;
use expectrl::repl::ReplSession;
use crate::caller::py;

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
pub struct Main {
    settings: Settings,
    python_runtime: ReplSession,
}
impl Default for Main {
    fn default() -> Self {
        Main {
            settings: Settings::default(),
            python_runtime: py::python_runtime(),
        }
    }
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
            if ui.button("Numpy").clicked() {
                py::runfile(&mut self.python_runtime, "src_python/test.py");
                println!("{}", py::read_output(&mut self.python_runtime, r"\[.*\]".to_owned()));
            }
            if ui.button("quit").clicked() {
                self.python_runtime.exit().expect("quit failed");
            }
        });
    }
}