use async_trait::async_trait;

#[async_trait]
pub trait ActivityPubService: Sync + Send {
    async fn host_meta(&self) -> String;
    async fn web_finger(&self, resource: String) -> Result<String, ()>;
    async fn node_info_links(&self) -> String;
    async fn node_info(&self) -> String;
}
