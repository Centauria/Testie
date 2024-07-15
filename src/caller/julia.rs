use std::io;
use std::io::Write;
use std::process::Command;
use jlrs::prelude::*;

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