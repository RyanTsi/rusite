use std::error::Error;

use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn write_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_name).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

pub async fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_name).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

pub fn current_time() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn str_split_to_vec(str: &str, delimiter: &str) -> Vec<String> {
    str.split(delimiter)
        .map(|item| item.trim().to_string())
        .collect()
}