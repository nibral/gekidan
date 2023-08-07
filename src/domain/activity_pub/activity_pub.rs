use serde::Serialize;

#[derive(Serialize)]
pub struct WebFinger {
    pub subject: String,
    pub links: Vec<WebFingerLinkItem>,
}

#[derive(Serialize)]
pub struct WebFingerLinkItem {
    pub rel: String,
    pub r#type: String,
    pub href: String,
}

#[derive(Serialize)]
pub struct NodeInfoLinks {
    pub links: Vec<NodeIngoLinkItem>,
}

#[derive(Serialize)]
pub struct NodeIngoLinkItem {
    pub rel: String,
    pub href: String,
}

#[derive(Serialize)]
pub struct NodeInfo {
    pub version: String,
    pub software: NodeInfoSoftware,
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    #[serde(rename(serialize = "openRegistrations"))]
    pub open_registrations: bool,
    pub usage: NodeInfoUsage,
    pub metadata: NodeInfoMetadata,
}

#[derive(Serialize)]
pub struct NodeInfoSoftware {
    pub name: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct NodeInfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

#[derive(Serialize)]
pub struct NodeInfoUsage {
    pub users: NodeInfoUsers,
}

#[derive(Serialize)]
pub struct NodeInfoUsers {
    pub total: usize,
}

#[derive(Serialize)]
pub struct NodeInfoMetadata {}

#[derive(Serialize)]
pub struct PersonPublicKey {
    pub id: String,
    pub owner: String,
    #[serde(rename(serialize = "publicKeyPem"))]
    pub public_key_pem: String,
}

#[derive(Serialize)]
pub struct Person {
    #[serde(rename(serialize = "@context"))]
    pub context: Vec<String>,
    pub id: String,
    pub r#type: String,
    #[serde(rename(serialize = "preferredUsername"))]
    pub preferred_username: String,
    pub inbox: String,
    pub outbox: String,
    #[serde(rename(serialize = "sharedInbox"))]
    pub shared_inbox: String,
    #[serde(rename(serialize = "publicKey"))]
    pub public_key: PersonPublicKey,
    pub featured: String,
    #[serde(rename(serialize = "manuallyApprovesFollowers"))]
    pub manually_approves_followers: bool,
    pub discoverable: bool,
}
