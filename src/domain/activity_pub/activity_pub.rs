use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use void::Void;

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

#[derive(Serialize)]
pub struct ActivityObject {
    #[serde(rename(serialize = "@context"))]
    pub context: String,
    pub r#type: String,
    pub id: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub content: String,
}

#[derive(Serialize)]
pub struct ActivityNoteItem {
    #[serde(rename(serialize = "@context"))]
    pub context: String,
    pub r#type: String,
    pub id: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub actor: String,
    pub object: ActivityObject,
}

pub struct ActivityItemParams {
    pub app_url: String,
    pub user_id: String,
    pub note_id: String,
    pub content: String,
    pub published: String,
}

impl ActivityNoteItem {
    pub fn new(params: &ActivityItemParams) -> Self {
        ActivityNoteItem {
            context: "https://www.w3.org/ns/activitystreams".to_string(),
            r#type: "Create".to_string(),
            id: format!("{}notes/{}", params.app_url, params.note_id),
            published: params.published.clone(),
            to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
            cc: vec![format!("{}users/{}/followers", params.app_url, params.user_id)],
            actor: format!("{}users/{}", params.app_url, params.user_id),
            object: ActivityObject {
                context: "https://www.w3.org/ns/activitystreams".to_string(),
                r#type: "Note".to_string(),
                id: format!("{}notes/{}", params.app_url, params.note_id),
                published: params.published.clone(),
                to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
                cc: vec![format!("{}users/{}/followers", params.app_url, params.user_id)],
                content: params.content.clone(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct ActivityNoteBox {
    #[serde(rename(serialize = "@context"))]
    pub context: String,
    pub summary: String,
    pub r#type: String,
    #[serde(rename(serialize = "totalItems"))]
    pub total_items: i16,
    #[serde(rename(serialize = "orderedItems"))]
    pub ordered_items: Vec<ActivityNoteItem>,
}

#[derive(Debug, Deserialize)]
pub struct InboxObject {
    pub r#type: String,
    pub id: String,
    pub actor: String,
    pub object: String,
}

impl FromStr for InboxObject {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InboxObject {
            r#type: "".to_string(),
            id: "".to_string(),
            actor: "".to_string(),
            object: s.to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct InboxActivity {
    pub r#type: String,
    pub id: String,
    pub actor: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub object: InboxObject,
}

// https://serde.rs/string-or-struct.html
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err=Void>,
        D: Deserializer<'de>
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
        where
            T: Deserialize<'de> + FromStr<Err=Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error
        {
            Ok(FromStr::from_str(v).unwrap())
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Serialize)]
pub struct FollowAcceptObject {
    pub r#type: String,
    pub actor: String,
    pub object: String,
}

#[derive(Serialize)]
pub struct FollowAccept {
    #[serde(rename(serialize = "@context"))]
    pub context: String,
    pub summary: String,
    pub r#type: String,
    pub actor: String,
    pub object: FollowAcceptObject,
}
