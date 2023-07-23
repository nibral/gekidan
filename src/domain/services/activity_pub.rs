pub trait ActivityPubService {
    fn host_meta(&self) -> String;
    fn web_finger(&self) -> String;
    fn node_info_links(&self) -> String;
    fn node_info(&self) -> String;
}
