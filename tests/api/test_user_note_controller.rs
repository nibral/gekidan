#[cfg(test)]
mod test_user_note_controller {
    use std::env;
    use actix_web::test;
    use gekidan::app::factory::create_app;
    use gekidan::presentation::controllers::user_management::UserResponse;
    use gekidan::presentation::controllers::user_note::{UserNoteListResponse, UserNoteResponse};
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

        // add user
        let api_key = ("x-admin-api-key", dotenv::var("ADMIN_API_KEY").unwrap());
        let res = test::TestRequest::post().uri("/admin/users")
            .append_header(api_key.clone())
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"username": "hoge", "display_name": "Hoge One"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserResponse = test::read_body_json(res).await;
        let uid = body.id;

        // list
        let res = test::TestRequest::get().uri(&format!("/users/{}/notes", uid))
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteListResponse = test::read_body_json(res).await;
        assert_eq!(body.notes.len(), 0);

        // add
        let res = test::TestRequest::post().uri(&format!("/users/{}/notes", uid))
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"content": "foobarbaz111"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteResponse = test::read_body_json(res).await;
        assert_eq!(body.content, "foobarbaz111");
        let nid = body.id.clone();

        // add
        let res = test::TestRequest::post().uri(&format!("/users/{}/notes", uid))
            .append_header(("Content-Type", "application/json"))
            .set_payload(r#"{"content": "foobarbaz222"}"#)
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteResponse = test::read_body_json(res).await;
        assert_eq!(body.content, "foobarbaz222");

        // list
        let res = test::TestRequest::get().uri(&format!("/users/{}/notes", uid))
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteListResponse = test::read_body_json(res).await;
        assert_eq!(body.notes.len(), 2);

        // get
        let res = test::TestRequest::get().uri(&format!("/users/{}/notes/{}", uid, nid))
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteResponse = test::read_body_json(res).await;
        assert_eq!(body.content, "foobarbaz111");

        // delete
        let res = test::TestRequest::delete().uri(&format!("/users/{}/notes/{}", uid, nid))
            .send_request(&app)
            .await;
        assert!(res.status().is_success());

        // list
        let res = test::TestRequest::get().uri(&format!("/users/{}/notes", uid))
            .send_request(&app)
            .await;
        assert!(res.status().is_success());
        let body: UserNoteListResponse = test::read_body_json(res).await;
        assert_eq!(body.notes.len(), 1);
    }
}
