#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

mod caller;

use egui;
use eframe;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Centauria".to_owned();
    let mut bpm = 120;
    let mut sr = 44100;

    let mut r = caller::julia::call("src_julia/test.jl", None);
    print!("{}", r);

    r = caller::py::call("src_python/test.py", None);
    print!("{}", r);

    eframe::run_simple_native("Testie", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mic Frequency Response Estimation");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut bpm, 60..=240).text("BPM"));
            egui::ComboBox::from_label("Samplerate")
                .selected_text(format!("{:?}", sr))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut sr, 44100, "44100");
                    ui.selectable_value(&mut sr, 48000, "48000")
                });
            ui.label(format!("Hello '{name}', BPM {bpm}, sample rate {sr}"));
        });
    })
}
