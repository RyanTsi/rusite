use rusite::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv::dotenv().ok();
    env_logger::init();
    router::run_server().await
}