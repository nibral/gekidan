use actix_web::HttpServer;
use gekidan::app::factory::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://localhost:8080");

    HttpServer::new(|| create_app())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
