use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS, NODEGROUPS};
use super::super::Client;
use super::schemas;

pub fn list_nodegroups(
    client: &Client,
    cluster_id: &str,
) -> Result<Vec<schemas::Nodegroup>, Error> {
    let path = format!(
        "/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS
    );
    let req = client.new_request(Method::GET, path.as_str(), None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::NodegroupsRoot =
        serde_json::from_str(body.as_str()).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.nodegroups)
}
