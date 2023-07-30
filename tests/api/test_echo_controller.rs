#[cfg(test)]
mod test_echo_controller {
    use std::env;
    use std::str::from_utf8;
    use actix_web::test;
    use gekidan::app::factory::create_app;

    #[actix_web::test]
    async fn test() {
        let _ = env_logger::try_init();

        env::set_var("ENV", "test");
        let app = test::init_service(create_app()).await;

        let res = test::TestRequest::get().uri("/").send_request(&app).await;
        assert!(res.status().is_success());
        let bytes = test::read_body(res).await;
        let body = from_utf8(&bytes).unwrap();
        assert_eq!(body, "ok");
    }
}
