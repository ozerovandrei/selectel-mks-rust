use selectel_mks::Client;
use selectel_mks::{cluster, nodegroup};

fn main() {
    // Get endpoint for the needed region:
    //  - ru-1: https://ru-1.mks.selcloud.ru
    //  - ru-2: https://ru-2.mks.selcloud.ru
    //  - ru-3: https://ru-3.mks.selcloud.ru
    //  - ru-7: https://ru-7.mks.selcloud.ru
    //  - ru-8: https://ru-8.mks.selcloud.ru
    let endpoint = "https://ru-3.mks.selcloud.ru";

    // Get project-scoped token value.
    let token = "token_value";

    // Initialize a new client.
    let client = Client::new(endpoint, token).expect("failed to initialize MKS client");

    // Prepare nodegroup options.
    let nodes_count = 2;
    let local_volumes = false;
    let availability_zone = "ru-3a"; // should be available in the selected region
    let nodegroup_opts =
        nodegroup::schemas::CreateOpts::new(nodes_count, local_volumes, &availability_zone)
            .with_cpus(2)
            .with_ram_mb(2048)
            .with_volume_gb(10)
            .with_volume_type("fast.ru-3a"); // should be available in the selected zone

    // Prepare cluster options.
    let name = "my-cluster";
    let kube_version = "1.17.9";
    let region = "ru-3";
    let cluster_opts = cluster::schemas::CreateOpts::new(name, &kube_version, &region)
        .with_nodegroups(vec![nodegroup_opts]);

    // Create a new cluster.
    let cluster_resp = client
        .create_cluster(&cluster_opts)
        .expect("unable to create a cluster");

    // Get cluster status.
    let status = client
        .get_cluster(&cluster_resp.id)
        .expect("unable to get cluster");

    println!("cluster status: {:?}", status);
}
