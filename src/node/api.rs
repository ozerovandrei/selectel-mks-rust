use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS, NODEGROUPS, REINSTALL};
use super::super::Client;
use super::schemas;

pub fn get_node(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    node_id: &str,
) -> Result<schemas::Node, Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id, node_id
    );
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::NodeRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.node)
}

pub fn reinstall_node(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    node_id: &str,
) -> Result<(), Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id, node_id, REINSTALL
    );
    let req = client.new_request(Method::POST, &path, None)?;
    client.do_request(req)?;

    Ok(())
}
