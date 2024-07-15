use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use jlrs::prelude::*;
use crate::util;

pub fn call(filename: &str, args: Option<Vec<&str>>) -> String {
    let mut filename_and_args = vec![filename];
    if let Some(args) = args {
        for arg in args {
            filename_and_args.push(arg);
        }
    }
    let python_output = Command::new("julia")
        .args(filename_and_args)
        .output()
        .expect("failed");
    String::from_utf8(python_output.stdout).unwrap()
}

#[allow(dead_code)]
fn julia_runtime_example() {
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
}

pub fn download_julia() {
    let mut julia_path = PathBuf::new();
    julia_path.push(util::get_current_working_dir());
    julia_path.push("runtime");
    julia_path.push("julia");
    if !julia_path.exists() {
        std::fs::create_dir_all(&julia_path).unwrap();
        util::download(
            "https://julialang-s3.julialang.org/bin/winnt/x64/1.10/julia-1.10.4-win64.zip".to_owned(),
            &julia_path)
            .expect("download error");
    }
}