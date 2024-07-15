use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

pub fn download(url: String, path: &Path) -> Result<(), reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|seg| seg.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.zip");
        let fname = path.join(fname);
        File::create(fname).expect("file create error")
    };
    let content = response.bytes()?;
    dest.write_all(&*content).expect("copy error");
    Ok(())
}