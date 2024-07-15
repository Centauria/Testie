use std::process::Command;


pub fn call(filename: &str, args: Option<Vec<&str>>) -> String {
    let mut filename_and_args = vec![filename];
    if let Some(args) = args {
        for arg in args {
            filename_and_args.push(arg);
        }
    }
    let python_output = Command::new("python")
        .args(filename_and_args)
        .output()
        .expect("failed");
    String::from_utf8(python_output.stdout).unwrap()
}