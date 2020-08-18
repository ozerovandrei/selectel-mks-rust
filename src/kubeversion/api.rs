use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, KUBEVERSIONS};
use super::super::Client;
use super::schemas;

pub fn list(client: &Client) -> Result<Vec<schemas::KubeVersion>, Error> {
    let path = format!("/{}/{}", API_VERSION, KUBEVERSIONS);
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::KubeVersionsRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.kube_versions)
}
