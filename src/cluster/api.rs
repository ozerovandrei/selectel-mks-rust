use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS};
use super::super::Client;
use super::schemas;

pub fn get_cluster(client: &Client, cluster_id: &str) -> Result<schemas::Cluster, Error> {
    let path = format!("/{}/{}/{}", API_VERSION, CLUSTERS, cluster_id);
    let req = client.new_request(Method::GET, path.as_str(), None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ClusterRoot =
        serde_json::from_str(body.as_str()).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.cluster)
}
