#[cfg(test)]
mod test_activity_pub_controller {
    use std::env;
    use actix_web::test;
    use sea_orm::Database;
    use serde::Deserialize;
    use gekidan::app::factory::create_app;
    use gekidan::presentation::controllers::user_management::UserResponse;
    use migrations::{Migrator, MigratorTrait};

    #[actix_web::test]
    async fn test() {
        let _ = env_logger::try_init();

        env::set_var("ENV", "test");
        let app = test::init_service(create_app()).await;

        // setup database
        let db = Database::connect(dotenv::var("DATABASE_URL").unwrap()).await.unwrap();
        let _ = Migrator::fresh(&db).await;

        // auth header
        let api_key = ("x-admin-api-key", dotenv::var("ADMIN_API_KEY").unwrap());

        // add user
        let res = test::TestRequest::post().uri("/admin/users")
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge", "display_name": "Hoge One"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserResponse = test::read_body_json(res).await;
        let uid = body.id;

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

        // actor redirect (UserID -> Username)
        let app_url = dotenv::var("APP_URL").unwrap();
        let res = test::TestRequest::get().uri(&format!("/users/{}", uid)).send_request(&app).await;
        assert_eq!(res.headers().get("Location").unwrap().to_str().unwrap(), format!("{}@hoge", app_url));

        // actor
        #[derive(Deserialize)]
        struct Actor {
            id: String,
        }
        let res = test::TestRequest::get().uri("/@hoge").send_request(&app).await;
        assert!(res.status().is_success());
        let body: Actor = test::read_body_json(res).await;
        assert_eq!(body.id, format!("{}users/{}", app_url, uid));
    }
}
