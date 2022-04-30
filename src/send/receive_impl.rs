use super::JUMAO_SERVER;
use anyhow::Result;
use reqwest::header::CONTENT_TYPE;

pub fn receive_file(code: &str, output: Option<&str>) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}?code={}", JUMAO_SERVER, code);
    let mut resp = client.get(url).send()?;

    let headers = resp.headers();
    let content_type = match headers.get(CONTENT_TYPE) {
        Some(header) => header.to_str()?,
        None => "text/plain",
    };

    if output.is_none() && !content_type.starts_with("text/") {
        println!("Specify output path to save binary data.");
        return Ok(());
    }

    match output {
        None => {
            println!("{}", resp.text()?);
        }
        Some(path) => {
            let mut file = std::fs::File::create(path)?;
            resp.copy_to(&mut file)?;
        }
    }

    Ok(())
}
