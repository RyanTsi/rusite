use std::path::Path;
use std::fs;
use rusite::router;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pre_check()?;
    dotenv::dotenv().ok();
    env_logger::init();
    router::run_server().await?;
    Ok(())
}

fn pre_check() -> Result<(), Box<dyn Error>> {
    let static_dir = Path::new("./static");
    if !std::path::Path::new(static_dir).exists() {
        fs::create_dir_all(static_dir)?;
    }
    Ok(())
}