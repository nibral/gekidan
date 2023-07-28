use std::sync::Arc;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::activity_pub::ActivityPubService;
use crate::domain::services::app_config::AppConfigService;

const AP_HOST_META_TEMPLATE: &str = &r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
<Link rel="lrdd" type="application/xrd+xml" template="https://rust-ap-example.room4.dev/.well-known/webfinger?resource={uri}" />
</XRD>"#;

pub struct ActivityPubServiceImpl {
    pub app_config_service: Arc<dyn AppConfigService>,
    pub user_repository: Arc<dyn UserRepository>,
}

impl ActivityPubServiceImpl {
    pub fn new(
        app_config_service: Arc<dyn AppConfigService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        ActivityPubServiceImpl {
            app_config_service,
            user_repository,
        }
    }
}

#[async_trait]
impl ActivityPubService for ActivityPubServiceImpl {
    async fn host_meta(&self) -> String {
        let app_url = &self.app_config_service.get_app_config().app_url;
        AP_HOST_META_TEMPLATE.replace("APP_URL", app_url).to_string()
    }

    async fn web_finger(&self, resource: String) -> Result<String, ()> {
        let resource = if resource.starts_with("acct:") { &resource["acct:".len()..] } else { &resource };

        // check format as "foo@fqdn"
        let elem: Vec<&str> = resource.split("@").collect();
        if elem.len() != 2 {
            return Err(());
        }

        // check domain
        let app_url_host = &self.app_config_service.get_app_config().app_url_host;
        if elem[1] != app_url_host {
            return Err(());
        }

        // return web finger
        #[derive(Serialize)]
        struct Links {
            rel: String,
            r#type: String,
            href: String,
        }
        #[derive(Serialize)]
        struct WebFinger {
            subject: String,
            links: Vec<Links>,
        }

        let app_url = &self.app_config_service.get_app_config().app_url;
        Ok(json!(WebFinger{
            subject: resource.to_string(),
            links: vec![
                Links {
                    rel: "self".to_string(),
                    r#type: "application/activity+json".to_string(),
                    href: format!("{}{}", app_url, elem[0])
                },
            ]
        }).to_string())
    }

    async fn node_info_links(&self) -> String {
        #[derive(Serialize)]
        struct Links {
            rel: String,
            href: String,
        }

        let app_url = &self.app_config_service.get_app_config().app_url;
        let rel = "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string();
        let href = format!("{}nodeinfo/2.1", app_url);

        json!(Links{rel, href}).to_string()
    }

    async fn node_info(&self) -> String {
        #[derive(Serialize)]
        struct Software {
            name: String,
            version: String,
        }
        #[derive(Serialize)]
        struct Services {
            inbound: Vec<String>,
            outbound: Vec<String>,
        }
        #[derive(Serialize)]
        struct Users {
            total: usize,
        }
        #[derive(Serialize)]
        struct Usage {
            users: Users,
        }
        #[derive(Serialize)]
        struct Metadata {}

        #[derive(Serialize)]
        struct Nodeinfo {
            version: String,
            software: Software,
            protocols: Vec<String>,
            services: Services,
            #[serde(rename(serialize = "openRegistrations"))]
            open_registrations: bool,
            usage: Usage,
            metadata: Metadata,
        }

        let users = (&self.user_repository).list().await.unwrap().len();
        let info = Nodeinfo {
            version: "2.1".to_string(),
            software: Software {
                name: "Gekidan".to_string(),
                version: "0.1".to_string(),
            },
            protocols: vec!["activitypub".to_string()],
            services: Services { inbound: vec![], outbound: vec![] },
            open_registrations: false,
            usage: Usage { users: Users { total: users } },
            metadata: Metadata {},
        };

        json!(info).to_string()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use async_trait::async_trait;
    use crate::domain::error::CommonError;
    use crate::domain::models::app_config::AppConfig;
    use crate::domain::models::user::User;
    use crate::domain::models::user_rsa_key::UserRsaKey;
    use crate::domain::repositories::user::UserRepository;
    use crate::domain::services::activity_pub::ActivityPubService;
    use crate::domain::services::app_config::AppConfigService;
    use crate::services::activity_pub::ActivityPubServiceImpl;

    struct MockAppConfigService {
        app_config: AppConfig,
    }

    impl MockAppConfigService {
        fn new() -> Self {
            MockAppConfigService {
                app_config: AppConfig {
                    app_url: "http://test.example.com/".to_string(),
                    app_url_host: "test.example.com".to_string(),
                    admin_api_key: "IamAdmin1234".to_string(),
                }
            }
        }
    }

    impl AppConfigService for MockAppConfigService {
        fn get_app_config(&self) -> &AppConfig {
            &self.app_config
        }
    }

    struct MockUserRepository {}

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, _new_user: &User) -> Result<User, CommonError> {
            todo!()
        }

        async fn list(&self) -> Result<Vec<User>, CommonError> {
            todo!()
        }

        async fn get(&self, _user_id: String) -> Result<User, CommonError> {
            todo!()
        }

        async fn delete(&self, _user_id: String) -> Result<(), CommonError> {
            todo!()
        }

        async fn find_by_username_with_rsa_key(&self, _username: String) -> Result<(User, UserRsaKey), CommonError> {
            todo!()
        }
    }

    #[actix_web::test]
    async fn web_finger() {
        let service = ActivityPubServiceImpl {
            app_config_service: Arc::new(MockAppConfigService::new()),
            user_repository: Arc::new(MockUserRepository {}),
        };

        assert!(
            service.web_finger("acct:hoge@test.example.com".to_string()).await.is_ok()
        );
        assert!(
            service.web_finger("hoge@test.example.com".to_string()).await.is_ok()
        );
        assert!(
            service.web_finger("hoge@example.com".to_string()).await.is_err()
        );
        assert!(
            service.web_finger("hogehoge".to_string()).await.is_err()
        );
        assert!(
            service.web_finger("".to_string()).await.is_err()
        );
    }
}
