use hyper::Method;
use serde_json;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, KUBEVERSIONS};
use super::super::MKS;
use super::schemas;

pub fn list_kube_versions(mks: MKS) -> Result<Vec<schemas::KubeVersion>, Error> {
    let path = format!("/{}/{}", API_VERSION, KUBEVERSIONS);
    let req = mks.new_request(Method::GET, path.as_str(), None)?;
    let res = mks.do_request(req)?;

    let deserialized: schemas::KubeVersionsRoot = serde_json::from_str(res.body.as_str())
        .map_err(|err| Error::DeserializeError(err, res.body))?;

    Ok(deserialized.kube_versions)
}
