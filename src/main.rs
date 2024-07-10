#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

use egui;
use eframe;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> eframe::Result {
    let path = std::env::var("PATH").unwrap();
    let mut paths: Vec<&str> = path.split(";").collect();
    paths.extend([r"D:\PF\anaconda3\DLLs",
        r"D:\PF\anaconda3\Lib",
        r"D:\PF\anaconda3",
        r"D:\PF\anaconda3\Lib\site-packages",
        r"D:\PF\anaconda3\Lib\site-packages\win32",
        r"D:\PF\anaconda3\Lib\site-packages\win32\lib",
        r"D:\PF\anaconda3\Lib\site-packages\Pythonwin"]);
    std::env::set_var("PATH", paths.join(";"));
    pyo3::prepare_freethreaded_python();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    Python::with_gil(|py| -> PyResult<()> {
        let sys = py.import_bound("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        let path: Vec<String> = sys.getattr("path")?.extract()?;

        let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        println!("PYTHONPATH={:?}", path);
        Ok(())
    }).expect("panic");

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
