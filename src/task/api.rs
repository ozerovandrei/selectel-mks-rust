use hyper::Method;

use super::super::error::Error;
use super::super::resource_url::{API_VERSION, CLUSTERS, TASKS};
use super::super::Client;
use super::schemas;

pub fn get(client: &Client, cluster_id: &str, task_id: &str) -> Result<schemas::Task, Error> {
    let path = format!(
        "/{}/{}/{}/{}/{}",
        API_VERSION, CLUSTERS, cluster_id, TASKS, task_id
    );
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::TaskRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.task)
}

pub fn list(client: &Client, cluster_id: &str) -> Result<Vec<schemas::Task>, Error> {
    let path = format!("/{}/{}/{}/{}", API_VERSION, CLUSTERS, cluster_id, TASKS);
    let req = client.new_request(Method::GET, &path, None)?;
    let body = client.do_request(req)?;

    let deserialized: schemas::ListRoot =
        serde_json::from_str(&body).map_err(|err| Error::DeserializeError(err, body))?;

    Ok(deserialized.tasks)
}
