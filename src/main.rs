#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

mod caller;
mod apps;
mod util;

use egui;
use eframe;
use crate::apps::main::Main;
use crate::caller::{julia, py};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let mut r = julia::call("src_julia/test.jl", None);
    print!("{}", r);

    r = py::call("src_python/test.py", None);
    print!("{}", r);

    eframe::run_native("Testie", options, Box::new(|_cc| Ok(Box::<Main>::default())))
}
