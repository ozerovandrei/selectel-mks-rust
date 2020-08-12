mod common;

use mks;

#[test]
fn list_kubeversions() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let client = common::setup();
    let kube_versions = mks::kubeversion::api::list_kube_versions(client)
        .expect("Failed to list Kubernetes versions");

    assert!(!kube_versions.is_empty());
    println!("Kubernetes versions: {:?}", kube_versions);
}
