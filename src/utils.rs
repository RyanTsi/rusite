use std::error::Error;
use std::path::Path;
use tokio::{fs, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn write_file(file_path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    let parent_dir = file_path.parent().ok_or("Invalid file path")?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).await?;
    }
    let mut file = fs::File::create(file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

pub async fn read_file(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(file_path).await?;
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