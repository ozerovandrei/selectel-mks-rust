use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, KUBEVERSIONS};
use super::super::Client;
use super::schemas;

pub fn list_kube_versions(client: &Client) -> Result<Vec<schemas::KubeVersion>, Error> {
    let path = format!("/{}/{}", API_VERSION, KUBEVERSIONS);
    let req = client.new_request(Method::GET, path.as_str(), None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::KubeVersionsRoot =
        serde_json::from_str(body.as_str()).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.kube_versions)
}
