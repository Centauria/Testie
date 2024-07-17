use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use expectrl::repl::ReplSession;
use tokio::fs::{OpenOptions, read_to_string};
use tokio::io::AsyncWriteExt;
use crate::util;

pub fn call(python_path: &str, filename: &str, args: Option<Vec<&str>>) -> String {
    let mut filename_and_args = vec![filename];
    if let Some(args) = args {
        for arg in args {
            filename_and_args.push(arg);
        }
    }
    let python_output = Command::new(python_path)
        .args(filename_and_args)
        .output()
        .expect("failed");
    String::from_utf8(python_output.stdout).unwrap()
}
#[allow(dead_code)]
fn add_to_path() {
    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from(r"D:\PF\anaconda3\envs\forecast"));
        paths.push(PathBuf::from(r"C:\Users\centa\.julia\juliaup\julia-1.10.2+0.x64.w64.mingw32\bin"));
        let new_path = env::join_paths(paths).unwrap();
        env::set_var("PATH", &new_path);
    }
    if let Some(path) = env::var_os("PATH") {
        println!("{}", path.to_str().expect("failed"));
        std::io::stdout().flush().unwrap();
    }
}

pub async fn download_python() {
    let mut python_path = PathBuf::new();
    python_path.push(util::get_current_working_dir());
    python_path.push("runtime");
    python_path.push("python");
    if !python_path.exists() {
        std::fs::create_dir_all(&python_path).unwrap();
        let filename = util::download(
            "https://www.python.org/ftp/python/3.12.4/python-3.12.4-embed-amd64.zip".to_owned(),
            &python_path)
            .await
            .expect("");
        util::decompress(filename, python_path.as_path()).expect("Error while decompressing");
        let get_pip = util::download(
            "https://bootstrap.pypa.io/get-pip.py".to_owned(),
            &python_path)
            .await
            .expect("");
        call(python_path.join("python.exe").to_str().unwrap(),
             get_pip.as_str(),
             None);
        let content = read_to_string(python_path.join("python312._pth"))
            .await.expect("Cannot read python312._pth");
        let mut pth = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(python_path.join("python312._pth"))
            .await
            .expect("Cannot write to python312._pth");
        let pattern = regex::Regex::new(r"^#import site$").unwrap();
        let mut buf = Vec::<u8>::new();
        for line in content.lines() {
            if pattern.is_match(line) {
                writeln!(buf, "import site").unwrap();
            } else {
                writeln!(buf, "{}", line).unwrap();
            }
            pth.write_all(&buf).await.expect("Cannot write to python312._pth");
            buf.truncate(0);
        }
    }
}
#[allow(dead_code)]
pub fn python_runtime() -> Option<ReplSession> {
    let mut python_path = PathBuf::new();
    python_path.push(util::get_current_working_dir());
    python_path.push("runtime");
    python_path.push("python");
    python_path.push("python.exe");
    if python_path.exists() {
        let mut session = ReplSession::new(
            expectrl::spawn(&python_path.to_str().unwrap()).unwrap(),
            ">>>".to_owned(),
            Some("import sys; sys.exit()".to_owned()),
            false);
        session.execute("import os").unwrap();
        session.execute(format!("os.chdir(r'{}')", util::get_current_working_dir())).unwrap();
        Some(session)
    } else { None }
}
pub fn runfile(session: &mut ReplSession, filename: &str) {
    session.send_line(format!("exec(open('{filename}').read())")).unwrap();
}
pub fn read_output(session: &mut ReplSession, regex_string: String) -> String {
    let found = session.expect(expectrl::Regex(regex_string)).unwrap();
    String::from_utf8_lossy(found.get(0).unwrap()).to_string()
}