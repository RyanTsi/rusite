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

pub async fn delete_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    if file_path.exists() {
        fs::remove_file(file_path).await?;
    }
    Ok(())
}

pub fn current_time() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn str_split_to_vec(str: &str, delimiter: &str) -> Vec<String> {
    str.split(delimiter)
        .map(|item| item.trim().to_string())
        .collect()
}

pub async fn get_cpu_usage() -> Result<f64, Box<dyn Error>> {
    let content = read_file(Path::new("/proc/stat")).await?;
    let mut lines = content.lines();
        if let Some(first_line) = lines.next() {
        if first_line.starts_with("cpu ") {
            let parts: Vec<u64> = first_line
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap_or(0))
                .collect();

            let user = parts[0];
            let nice = parts[1];
            let system = parts[2];
            let idle = parts[3];

            let total = user + nice + system + idle;
            let used = total - idle;

            // 返回 CPU 使用率百分比（瞬时值）
            return Ok(used as f64 / total as f64 * 100.0);
        }
    }
    Ok(1.0)
}

pub async fn get_memory_usage() -> Result<(usize, usize), Box<dyn Error>> {
    let content = read_file(Path::new("/proc/meminfo")).await?;
    let mut mem_total = 0;
    let mut mem_free = 0;
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            mem_total = line.split_whitespace().nth(1).unwrap().parse()?;
        } else if line.starts_with("MemAvailable:") {
            mem_free = line.split_whitespace().nth(1).unwrap().parse()?;
        }
    }
    let used = mem_total - mem_free;
    Ok((used, mem_total))
}