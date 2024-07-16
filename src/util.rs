use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use futures_util::stream::StreamExt;
use zip::ZipArchive;

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

pub async fn download(url: String, path: &Path) -> Result<String, String> {
    let proxy = reqwest::Proxy::http("socks5://127.0.0.1:10808")
        .or(Err("Error while creating proxy"))?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .or(Err("Error while build client"))?;
    let response = client.get(&url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = response
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    let fname = response
        .url()
        .path_segments()
        .and_then(|seg| seg.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("temp.zip");
    let fname = path.join(fname);
    let mut dest = File::create(&fname)
        .or(Err(format!("Failed to create file {}", fname.display())))?;
    let mut downloaded_size: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        dest.write_all(&chunk).or(Err("Error while writing to file".to_string()))?;
        let new = min(downloaded_size + (chunk.len() as u64), total_size);
        // let p_new = (new as f32 / total_size as f32 * 100.0f32) as u32;
        // let p_old = (downloaded_size as f32 / total_size as f32 * 100.0f32) as u32;
        downloaded_size = new;
    }
    let filename = fname.to_str().expect("");
    Ok(filename.to_string())
}

pub fn decompress(filename: String, path: &Path) -> Result<(), String> {
    let mut archive = ZipArchive::new(
        File::open(filename.clone())
            .or(Err("Error while opening archive"))?)
        .or(Err("Error while analyzing archive"))?;
    archive.extract(path).unwrap();
    std::fs::remove_file(Path::new(&filename))
        .or(Err("Error while deleting archive"))?;
    Ok(())
}