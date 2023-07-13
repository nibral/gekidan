#[cfg(test)]
mod test_root_controllers {
    use std::str::from_utf8;
    use actix_web::test;
    use gekidan::create_app::create_app;

    #[actix_web::test]
    async fn test() {
        env_logger::init();

        let app = test::init_service(create_app()).await;

        let res = test::TestRequest::get().uri("/").send_request(&app).await;
        assert!(res.status().is_success());
        let bytes = test::read_body(res).await;
        let body = from_utf8(&bytes).unwrap();
        assert_eq!(body, "ok");
    }
}
