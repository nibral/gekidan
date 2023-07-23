#[cfg(test)]
mod test_well_known_handler {
    use actix_web::test;
    use serde::Deserialize;
    use gekidan::create_app::create_app;

    #[actix_web::test]
    async fn test() {
        let _ = env_logger::try_init();

        dotenv::from_filename(".env.test").ok();
        let app = test::init_service(create_app()).await;

        // host-meta
        let res = test::TestRequest::get().uri("/.well-known/host-meta").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/xml");

        // nodeinfo links
        #[derive(Deserialize)]
        struct NodeInfoLinks {
            rel: String,
        }
        let res = test::TestRequest::get().uri("/.well-known/nodeinfo").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/json");
        let body: NodeInfoLinks = test::read_body_json(res).await;
        assert_eq!(body.rel, "http://nodeinfo.diaspora.software/ns/schema/2.1");

        // nodeinfo
        #[derive(Deserialize)]
        struct Software {
            name: String,
        }
        #[derive(Deserialize)]
        struct NodeInfo {
            software: Software,
        }
        let res = test::TestRequest::get().uri("/nodeinfo/2.1").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/json");
        let body: NodeInfo = test::read_body_json(res).await;
        assert_eq!(body.software.name, "Gekidan");
    }
}
