use actix_web::HttpServer;
use gekidan::create_app::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://localhost:8080");

    let server = HttpServer::new(|| create_app())
        .bind(("127.0.0.1", 8080))?;
    server.run().await
}
