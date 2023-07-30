use std::sync::Arc;
use url::Url;
use crate::domain::activity_pub::activity_pub::{NodeInfo, NodeInfoLinks, NodeInfoMetadata, NodeInfoServices, NodeInfoSoftware, NodeInfoUsage, NodeInfoUsers, NodeIngoLinkItem, WebFinger, WebFingerLinkItem};
use crate::domain::user::user_repository::UserRepository;

const AP_HOST_META_TEMPLATE: &str = &r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
<Link rel="lrdd" type="application/xrd+xml" template="APP_URL.well-known/webfinger?resource={uri}" />
</XRD>"#;

pub struct ActivityPubService {
    user_repository: Arc<dyn UserRepository>,
}

impl ActivityPubService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        ActivityPubService {
            user_repository,
        }
    }

    pub async fn host_meta(&self, app_url: &str) -> String {
        AP_HOST_META_TEMPLATE.replace("APP_URL", app_url).to_string()
    }

    pub async fn web_finger(&self, resource: &str, app_url: &str) -> Result<WebFinger, ()> {
        // delete "acct:" in start
        let resource = if resource.starts_with("acct:") { &resource["acct:".len()..] } else { &resource };

        // check format as "hoge@foo.example.com"
        let elem: Vec<&str> = resource.split("@").collect();
        if elem.len() != 2 {
            return Err(());
        }

        // check domain
        let parsed_app_url = Url::parse(app_url).unwrap();
        if elem[1].to_string() != parsed_app_url.host().unwrap().to_string() {
            return Err(());
        }

        Ok(WebFinger {
            subject: resource.to_string(),
            links: vec![
                WebFingerLinkItem {
                    rel: "self".to_string(),
                    r#type: "application/activity+json".to_string(),
                    // https://foo.example.com/users/hoge
                    href: format!("{}users/{}", app_url, elem[0]),
                }
            ],
        })
    }

    pub async fn node_info_links(&self, app_url: &str) -> NodeInfoLinks {
        NodeInfoLinks {
            links: vec![
                NodeIngoLinkItem {
                    rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
                    // https://foo.example.com/nodeinfo/2.1
                    href: format!("{}nodeinfo/2.1", app_url),
                }
            ],
        }
    }

    pub async fn node_info(&self) -> NodeInfo {
        let user_count = (&self.user_repository).list().await.unwrap().len();
        NodeInfo {
            version: "2.1".to_string(),
            software: NodeInfoSoftware {
                name: "Gekidan".to_string(),
                version: "0.1".to_string(),
            },
            protocols: vec!["activitypub".to_string()],
            services: NodeInfoServices { inbound: vec![], outbound: vec![] },
            open_registrations: false,
            usage: NodeInfoUsage { users: NodeInfoUsers { total: user_count } },
            metadata: NodeInfoMetadata {},
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use async_trait::async_trait;
    use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
    use crate::domain::error::CommonError;
    use crate::domain::user::user::User;
    use crate::domain::user::user_repository::UserRepository;

    struct MockUserRepository {}

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn add(&self, _new_user: &User) -> Result<(), CommonError> {
            todo!()
        }

        async fn list(&self) -> Result<Vec<User>, CommonError> {
            todo!()
        }

        async fn get(&self, _user_id: &str) -> Result<User, CommonError> {
            todo!()
        }

        async fn delete(&self, _user_id: &str) -> Result<(), CommonError> {
            todo!()
        }

        async fn find(&self, _username: &str) -> Result<Option<User>, CommonError> {
            todo!()
        }
    }

    #[actix_web::test]
    async fn web_finger() {
        let service = ActivityPubService {
            user_repository: Arc::new(MockUserRepository {}),
        };
        let app_url = "https://test.example.com/";

        assert!(service.web_finger("acct:hoge@test.example.com", app_url).await.is_ok());
        assert!(service.web_finger("hoge@test.example.com", app_url).await.is_ok());
        assert!(service.web_finger("hoge@example.com", app_url).await.is_err());
        assert!(service.web_finger("hogehoge", app_url).await.is_err());
        assert!(service.web_finger("", app_url).await.is_err());
    }
}
