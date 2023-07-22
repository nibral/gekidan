#[cfg(test)]
mod test_well_known_handler {
    use actix_web::test;
    use gekidan::create_app::create_app;

    #[actix_web::test]
    async fn test() {
        let _ = env_logger::try_init();

        dotenv::from_filename(".env.test").ok();
        let app = test::init_service(create_app()).await;

        let res = test::TestRequest::get().uri("/.well-known/host-meta").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/xml")
    }
}
