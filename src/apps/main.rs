use eframe::Frame;
use egui::Context;
use expectrl::repl::ReplSession;
use tokio::runtime::Runtime;

use crate::apps::settings::SettingsWindow;
use crate::apps::Window;
use crate::caller::{julia, py};

#[derive(Default)]
struct WindowStates {
    settings: bool,
}
pub struct Main {
    window_states: WindowStates,
    python_runtime: Option<ReplSession>,
    tokio_runtime: Runtime,
    status_bar_content: String,
    settings_window: SettingsWindow,
}
impl Default for Main {
    fn default() -> Self {
        Main {
            window_states: WindowStates::default(),
            python_runtime: py::python_runtime(),
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            status_bar_content: "".to_string(),
            settings_window: SettingsWindow::default(),
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
                        print!("{}", py::call(&self.settings_window.state.python_path,
                                              "src_python/test.py",
                                              None));
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

            ui.label(format!("BPM {}, sample rate {}", self.settings_window.state.bpm, self.settings_window.state.sr));
            if ui.button("Numpy").clicked() {
                if let Some(runtime) = &mut self.python_runtime {
                    println!("{}", py::run_file(runtime, "src_python/test.py"));
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
                self.settings_window.show(ctx, &mut self.window_states.settings);
            }
        });
    }
}
