#[cfg(test)]
mod test_user_management_controller {
    use std::env;
    use actix_web::test;
    use gekidan::app::factory::create_app;
    use gekidan::presentation::controllers::user_management::{UserListResponse, UserResponse};
    use migrations::{Migrator, MigratorTrait};
    use sea_orm::Database;

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

        // list
        let res = test::TestRequest::get().uri("/admin/users")
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserListResponse = test::read_body_json(res).await;
        assert_eq!(body.users.len(), 0);

        // add
        let res = test::TestRequest::post().uri("/admin/users")
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge", "display_name": "Hoge One"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());

        // add
        let res = test::TestRequest::post().uri("/admin/users")
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge2", "display_name": "Hoge Two"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());

        // list
        let res = test::TestRequest::get().uri("/admin/users")
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserListResponse = test::read_body_json(res).await;
        assert_eq!(body.users.len(), 2);
        let uid1 = body.users[0].id.clone();
        let uid2 = body.users[1].id.clone();

        // get
        let res = test::TestRequest::get().uri(&format!("/admin/users/{}", &uid1))
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserResponse = test::read_body_json(res).await;
        assert_eq!(body.username, "hoge");

        // update with duplicate username (fail)
        let res = test::TestRequest::put().uri(&format!("/admin/users/{}", &uid2))
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge", "display_name": "Hoge Two"}"#)
            .send_request(&app)
            .await;
        assert!(!res.status().is_success());

        // update
        let res = test::TestRequest::put().uri(&format!("/admin/users/{}", &uid2))
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge_two", "display_name": "Hoge Two"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserResponse = test::read_body_json(res).await;
        assert_eq!(body.username, "hoge_two");

        // get
        let res = test::TestRequest::get().uri(&format!("/admin/users/{}", &uid2))
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserResponse = test::read_body_json(res).await;
        assert_eq!(body.username, "hoge_two");

        // delete
        let res = test::TestRequest::delete().uri(&format!("/admin/users/{}", &uid1))
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());

        // list
        let res = test::TestRequest::get().uri("/admin/users")
            .append_header(api_key.clone())
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserListResponse = test::read_body_json(res).await;
        assert_eq!(body.users.len(), 1);

        // list without admin api-key (fail)
        let res = test::TestRequest::get().uri("/admin/users")
            .send_request(&app)
            .await;
        assert!(!res.status().is_success());
    }
}
