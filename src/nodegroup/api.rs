use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS, NODEGROUPS};
use super::super::Client;
use super::schemas;
use crate::nodegroup::schemas::NodegroupCreateOptsRoot;

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

pub fn get_nodegroup(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
) -> Result<schemas::Nodegroup, Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id
    );
    let req = client.new_request(Method::GET, path.as_str(), None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::NodegroupRoot =
        serde_json::from_str(body.as_str()).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.nodegroup)
}

pub fn create_nodegroup(
    client: &Client,
    cluster_id: &str,
    opts: &schemas::NodegroupCreateOpts,
) -> Result<(), Error> {
    let root_opts = NodegroupCreateOptsRoot { nodegroup: opts };
    let serialized = serde_json::to_string(&root_opts).map_err(Error::SerializeError)?;

    let path = format!(
        "/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS
    );
    let req = client.new_request(Method::POST, path.as_str(), Some(serialized))?;
    client.do_request(req)?;

    Ok(())
}
