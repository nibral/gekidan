#[cfg(test)]
mod test_activity_pub_controller {
    use std::env;
    use actix_web::test;
    use serde::Deserialize;
    use gekidan::app::factory::create_app;

    #[actix_web::test]
    async fn test() {
        let _ = env_logger::try_init();

        env::set_var("ENV", "test");
        let appA = test::init_service(create_app()).await;

        // host-meta
        let res = test::TestRequest::get().uri("/.well-known/host-meta").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/xml");

        // webfinger
        let res = test::TestRequest::get()
            .uri("/.well-known/webfinger?resource=acct:hoge@test.example.com")
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/jrd+json; charset=utf-8");

        // nodeinfo links
        #[derive(Deserialize)]
        struct NodeInfoLinkItem {
            rel: String,
        }
        #[derive(Deserialize)]
        struct NodeInfoLinks {
            links: Vec<NodeInfoLinkItem>,
        }
        let res = test::TestRequest::get().uri("/.well-known/nodeinfo").send_request(&app).await;
        assert!(res.status().is_success());
        assert_eq!(res.headers().get("Content-Type").unwrap().to_str().unwrap(), "application/json");
        let body: NodeInfoLinks = test::read_body_json(res).await;
        assert_eq!(body.links[0].rel, "http://nodeinfo.diaspora.software/ns/schema/2.1");

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