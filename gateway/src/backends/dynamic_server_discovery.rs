use std::collections::{BTreeSet, HashMap};
use async_trait::async_trait;
use pingora_load_balancing::Backend;
use pingora_load_balancing::discovery::ServiceDiscovery;

pub struct DynamicServerDiscovery {}

#[async_trait]
impl ServiceDiscovery for DynamicServerDiscovery {

    async fn discover(&self) -> pingora_error::Result<(BTreeSet<Backend>, HashMap<u64, bool>)> {
        todo!()
    }
}