use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS};
use super::super::Client;
use super::schemas;

pub fn get_cluster(client: &Client, cluster_id: &str) -> Result<schemas::Cluster, Error> {
    let path = format!("/{}/{}/{}", API_VERSION, CLUSTERS, cluster_id);
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ClusterRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.cluster)
}

pub fn list_clusters(client: &Client) -> Result<Vec<schemas::Cluster>, Error> {
    let path = format!("/{}/{}", API_VERSION, CLUSTERS);
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ListRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.clusters)
}

pub fn create_cluster(
    client: &Client,
    opts: &schemas::CreateOpts,
) -> Result<schemas::Cluster, Error> {
    let root_opts = schemas::CreateOptsRoot { cluster: opts };
    let serialized = serde_json::to_string(&root_opts).map_err(Error::SerializeError)?;

    let path = format!("/{}/{}", API_VERSION, CLUSTERS);
    let req = client.new_request(Method::POST, &path, Some(serialized))?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ClusterRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.cluster)
}

pub fn delete_cluster(client: &Client, cluster_id: &str) -> Result<(), Error> {
    let path = format!("/{}/{}/{}", API_VERSION, CLUSTERS, cluster_id);
    let req = client.new_request(Method::DELETE, &path, None)?;
    client.do_request(req)?;

    Ok(())
}
