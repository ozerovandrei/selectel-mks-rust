use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS, NODEGROUPS, RESIZE};
use super::super::Client;
use super::schemas;

pub fn get(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
) -> Result<schemas::Nodegroup, Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id
    );
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::NodegroupRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.nodegroup)
}

pub fn list(client: &Client, cluster_id: &str) -> Result<Vec<schemas::Nodegroup>, Error> {
    let path = format!(
        "/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS
    );
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ListRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.nodegroups)
}

pub fn create(client: &Client, cluster_id: &str, opts: &schemas::CreateOpts) -> Result<(), Error> {
    let root_opts = schemas::CreateOptsRoot { nodegroup: opts };
    let serialized = serde_json::to_string(&root_opts).map_err(Error::SerializeError)?;

    let path = format!(
        "/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS
    );
    let req = client.new_request(Method::POST, &path, Some(serialized))?;
    client.do_request(req)?;

    Ok(())
}

pub fn delete(client: &Client, cluster_id: &str, nodegroup_id: &str) -> Result<(), Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id
    );
    let req = client.new_request(Method::DELETE, &path, None)?;
    client.do_request(req)?;

    Ok(())
}

pub fn resize(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    opts: &schemas::ResizeOpts,
) -> Result<(), Error> {
    let root_opts = schemas::ResizeOptsRoot { nodegroup: opts };
    let serialized = serde_json::to_string(&root_opts).map_err(Error::SerializeError)?;

    let path = format!(
        "/{}/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id, RESIZE
    );
    let req = client.new_request(Method::POST, &path, Some(serialized))?;
    client.do_request(req)?;

    Ok(())
}

pub fn update(
    client: &Client,
    cluster_id: &str,
    nodegroup_id: &str,
    opts: &schemas::UpdateOpts,
) -> Result<(), Error> {
    let root_opts = schemas::UpdateOptsRoot { nodegroup: opts };
    let serialized = serde_json::to_string(&root_opts).map_err(Error::SerializeError)?;

    let path = format!(
        "/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, NODEGROUPS, nodegroup_id
    );
    let req = client.new_request(Method::PUT, &path, Some(serialized))?;
    client.do_request(req)?;

    Ok(())
}
