use super::JUMAO_SERVER;
use anyhow::Result;
use reqwest::blocking::multipart;

const MAX_SIZE: u64 = 1024 * 1024 * 15;

pub fn send_intro() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(JUMAO_SERVER).send()?;
    println!("{}", resp.text()?);
    Ok(())
}

pub fn send_file(path: &str) -> Result<()> {
    let file = std::fs::metadata(path)?;

    if !file.is_file() {
        println!("{} is not a file", path);
        return Ok(());
    }

    if file.len() > MAX_SIZE {
        println!("{} is too large", path);
        return Ok(());
    }

    let form = multipart::Form::new().file("file", path)?;
    let client = reqwest::blocking::Client::new();
    let resp = client.post(JUMAO_SERVER).multipart(form).send()?;

    println!("{}", resp.text()?);

    Ok(())
}
