use retry::delay::Fixed;
use retry::retry;
use selectel_mks::cluster;
use selectel_mks::error::Error;
use selectel_mks::Client;

const CLUSTER_CREATE_INTERVAL: u64 = 10_000;
const CLUSTER_CREATE_RETRIES: usize = 60;

const CLUSTER_DELETE_INTERVAL: u64 = 10_000;
const CLUSTER_DELETE_RETRIES: usize = 30;

/// Function wraps cluster creation with retries.
/// It panics in case of errors.
pub fn create_cluster_or_panic(
    client: &Client,
    opts: &cluster::schemas::CreateOpts,
) -> cluster::schemas::Cluster {
    let create_resp = client
        .create_cluster(opts)
        .expect("unable to create a cluster");

    wait_for_cluster_active_status_or_panic(
        client,
        &create_resp.id,
        CLUSTER_CREATE_INTERVAL,
        CLUSTER_CREATE_RETRIES,
    );

    println!("Created cluster {}", create_resp.id);

    create_resp
}

/// Function wraps cluster deletion with retries.
/// It panics in case of errors.
pub fn delete_cluster_or_panic(client: &Client, cluster_id: &str) {
    client
        .delete_cluster(cluster_id)
        .expect("unable to delete a cluster");

    wait_for_cluster_deletion_or_panic(
        client,
        cluster_id,
        CLUSTER_DELETE_INTERVAL,
        CLUSTER_DELETE_RETRIES,
    );

    println!("Deleted cluster {}", cluster_id);
}

/// Function wraps waiting of the cluster active status.
/// It panics in case of errors.
pub fn wait_for_cluster_active_status_or_panic(
    client: &Client,
    cluster_id: &str,
    interval: u64,
    retries: usize,
) {
    println!(
        "Waiting for cluster {} to become active, retries: {}, retry interval: {} ms",
        cluster_id, retries, interval
    );

    retry(
        Fixed::from_millis(interval).take(retries),
        || match client.get_cluster(cluster_id) {
            Ok(resp) => match resp.status {
                cluster::schemas::Status::Active => Ok(()),
                _ => {
                    let msg = format!("cluster {} is not active yet", cluster_id);
                    println!("{}", msg);

                    Err(msg)
                }
            },
            Err(e) => match e {
                Error::HttpError(status, _) => {
                    let msg = format!(
                        "got unexpected HTTP status code {} while getting status of the cluster {}",
                        status, cluster_id
                    );
                    println!("{}", msg);

                    Err(msg)
                }
                _ => {
                    let msg = format!(
                        "got unexpected error while getting status of the cluster {}: {:?}",
                        cluster_id, e
                    );
                    println!("{}", msg);

                    Err(msg)
                }
            },
        },
    )
    .expect("unable to wait for cluster to become active");
}

/// Function wraps waiting of the cluster 404 HTTP errors on get request.
/// It panics in case of errors.
pub fn wait_for_cluster_deletion_or_panic(
    client: &Client,
    cluster_id: &str,
    interval: u64,
    retries: usize,
) {
    println!(
        "Waiting for cluster {} to become deleted, retries: {}, retry interval: {} ms",
        cluster_id, retries, interval
    );

    retry(
        Fixed::from_millis(interval).take(retries),
        || match client.get_cluster(cluster_id) {
            Err(e) => match e {
                Error::HttpError(status, _) => match status {
                    404 => Ok(()),
                    _ => {
                        let msg = format!(
                            "got unexpected HTTP status code {} while getting status of the cluster {}",
                            status, cluster_id
                        );
                        println!("{}", msg);

                        Err(msg)
                    }
                },
                _ => {
                    let msg = format!(
                        "got unexpected error {:?} while getting status of the cluster {}",
                        e, cluster_id
                    );
                    println!("{}", msg);

                    Err(msg)
                }
            },
            _ => {
                let msg = format!("cluster {} is not deleted yet", cluster_id);
                println!("{}", msg);

                Err(msg)
            }
        },
    ).expect("unable to wait for cluster to become deleted");
}
