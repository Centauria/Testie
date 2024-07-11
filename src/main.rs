#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

use egui;
use eframe;
use std::io;
use std::io::Write;
use std::process::Command;
use jlrs::prelude::*;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    let mut julia = Builder::new().start_local().unwrap();

    julia.with_stack(|mut stack| {
        stack.scope(|mut frame| -> JlrsResult<()> {
            let w = Value::new(&mut frame, 6i32);
            let v4 = unsafe {
                Module::base(&frame)
                    .global(&mut frame, "zeros")?
                    .call(&mut frame, [w])
                    .into_jlrs_result()?
            };
            print!("zeros({}) = ", w.unbox::<i32>()?);
            io::stdout().flush().unwrap();
            unsafe {
                Module::base(&frame)
                    .global(&mut frame, "println")?
                    .call(&mut frame, [v4])
                    .into_jlrs_result()?;
            }
            Ok(())
        })
    }).unwrap();

    let julia_version = Command::new("julia")
        .args(["src_julia/test.jl"])
        .output()
        .expect("failed");
    io::stdout().write_all(&julia_version.stdout).unwrap();

    let python_version = Command::new("python")
        .args(["src_python/test.py"])
        .output()
        .expect("failed");
    io::stdout().write_all(&python_version.stdout).unwrap();

    eframe::run_simple_native("Testie", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
