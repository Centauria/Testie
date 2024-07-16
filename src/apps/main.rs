use eframe::Frame;
use egui::{Align2, Context, Vec2};
use expectrl::repl::ReplSession;
use tokio::runtime::Runtime;

use crate::caller::{julia, py};

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
struct WindowStates {
    settings: bool,
}
pub struct Main {
    settings: Settings,
    window_states: WindowStates,
    python_runtime: Option<ReplSession>,
    tokio_runtime: Runtime,
    status_bar_content: String,
}
impl Default for Main {
    fn default() -> Self {
        Main {
            settings: Settings::default(),
            window_states: WindowStates::default(),
            python_runtime: None,
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            status_bar_content: "".to_string(),
        }
    }
}

impl eframe::App for Main {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {}
                    ui.separator();
                    if ui.button("Preferences").clicked() {
                        self.window_states.settings = !self.window_states.settings;
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        // std::process::exit(0);
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Test", |ui| {
                    if ui.button("Test julia").clicked() {
                        print!("{}", julia::call("src_julia/test.jl", None));
                    }
                    if ui.button("Test python").clicked() {
                        print!("{}", py::call("src_python/test.py", None));
                    }
                    ui.separator();
                    if ui.button("Download python").clicked() {
                        self.tokio_runtime.spawn(py::download_python());
                        self.status_bar_content = "Downloading python ...".to_string();
                    }
                    if ui.button("Download julia").clicked() {
                        self.tokio_runtime.spawn(julia::download_julia());
                        self.status_bar_content = "Downloading julia ...".to_string();
                    }
                })
            });
            ui.heading("Mic Frequency Response Estimation");

            ui.label(format!("BPM {}, sample rate {}", self.settings.bpm, self.settings.sr));
            if ui.button("Numpy").clicked() {
                if let Some(runtime) = &mut self.python_runtime {
                    py::runfile(runtime, "src_python/test.py");
                    println!("{}", py::read_output(runtime, r"\[.*\]".to_owned()));
                }
            }
            if ui.button("quit").clicked() {
                if let Some(runtime) = &mut self.python_runtime {
                    runtime.exit().expect("quit failed");
                }
            }
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(0.0)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(&self.status_bar_content);
                    });
                });
            if self.window_states.settings {
                egui::Window::new("Preferences")
                    .max_size(Vec2 { x: 300.0, y: 200.0 })
                    .collapsible(false)
                    .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
                    .show(ctx, |ui| {
                        self.theme_switcher(ui, ctx);
                        ui.add(egui::Slider::new(&mut self.settings.bpm, 60.0..=240.0).text("BPM"));
                        egui::ComboBox::from_label("Samplerate")
                            .selected_text(format!("{:?}", self.settings.sr))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.settings.sr, 44100, "44100");
                                ui.selectable_value(&mut self.settings.sr, 48000, "48000");
                            });
                        ui.horizontal(|ui| {
                            if ui.button("OK").clicked() {
                                self.window_states.settings = false;
                            }
                            if ui.button("Cancel").clicked() {
                                self.window_states.settings = false;
                            }
                        })
                    });
            }
        });
    }
}

impl Main {
    fn theme_switcher(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Dark").clicked() {
                ctx.set_visuals(egui::Visuals::dark());
            }
            if ui.button("Light").clicked() {
                ctx.set_visuals(egui::Visuals::light());
            }
        });
    }
}
