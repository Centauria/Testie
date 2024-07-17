use eframe::emath::{Align2, Vec2};
use egui::{Context, Ui};
use crate::apps::{View, Window};

pub struct Settings {
    pub(crate) sr: i32,
    pub(crate) bpm: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sr: 44100,
            bpm: 120.0,
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
enum SettingsTab {
    Common,
    Audio,
    MIDI,
    Network,
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self::Common
    }
}
#[derive(Default)]
pub struct SettingsWindow {
    tab: SettingsTab,
    pub(crate) state: Settings,
}

impl Window for SettingsWindow {
    fn show(&mut self, ctx: &Context, open: &mut bool) {
        egui::Window::new("Preferences")
            .default_size(Vec2 { x: 300.0, y: 200.0 })
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
            .show(ctx, |ui| {
                use View as _;
                self.ui(ctx, ui);
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        *open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        *open = false;
                    }
                });
            });
    }
}

impl View for SettingsWindow {
    fn ui(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, SettingsTab::Common, "Common");
            ui.selectable_value(&mut self.tab, SettingsTab::Audio, "Audio");
            ui.selectable_value(&mut self.tab, SettingsTab::MIDI, "MIDI");
            ui.selectable_value(&mut self.tab, SettingsTab::Network, "Network");
        });
        ui.separator();
        match self.tab {
            SettingsTab::Common => {
                self.theme_switcher(ui, ctx);
                ui.add(egui::Slider::new(&mut self.state.bpm, 60.0..=240.0).text("BPM"));
            }
            SettingsTab::Audio => {
                egui::ComboBox::from_label("Samplerate")
                    .selected_text(format!("{:?}", self.state.sr))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.state.sr, 44100, "44100");
                        ui.selectable_value(&mut self.state.sr, 48000, "48000");
                    });
            }
            SettingsTab::MIDI => {}
            SettingsTab::Network => {}
        }
    }
}
impl SettingsWindow {
    fn theme_switcher(&mut self, ui: &mut Ui, ctx: &Context) {
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