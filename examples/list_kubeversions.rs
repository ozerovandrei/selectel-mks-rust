use selectel_mks::Client;

fn main() {
    // Get endpoint for the needed region:
    //  - ru-1: https://ru-1.mks.selcloud.ru
    //  - ru-2: https://ru-2.mks.selcloud.ru
    //  - ru-3: https://ru-3.mks.selcloud.ru
    //  - ru-7: https://ru-7.mks.selcloud.ru
    //  - ru-8: https://ru-8.mks.selcloud.ru
    let endpoint = "https://ru-8.mks.selcloud.ru";

    // Get project-scoped token value.
    let token = "token_value";

    // Initialize a new client.
    let client = Client::new(endpoint, token).expect("failed to initialize MKS client");

    // Get all Kubernetes versions.
    let kube_versions = client
        .list_kube_versions()
        .expect("failed to list Kubernetes versions");

    println!("Kubernetes versions: {:?}\n", kube_versions);
}
