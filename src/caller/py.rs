use std::process::{Command};
use expectrl::repl::ReplSession;
use expectrl::{Regex};
use crate::util;

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

pub fn python_runtime() -> ReplSession {
    let mut session = expectrl::repl::spawn_python().unwrap();
    session.execute("import os").unwrap();
    session.execute(format!("os.chdir(r'{}')", util::get_current_working_dir())).unwrap();
    session
}
pub fn runfile(session: &mut ReplSession, filename: &str) {
    session.send_line(format!("exec(open('{filename}').read())")).unwrap();
}
pub fn read_output(session: &mut ReplSession, regex_string: String) -> String {
    let found = session.expect(Regex(regex_string)).unwrap();
    String::from_utf8_lossy(found.get(0).unwrap()).to_string()
}