mod common;

#[test]
fn list_kubeversions() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    let client = common::setup();
    let kube_versions = client
        .list_kube_versions()
        .expect("Failed to list Kubernetes versions");

    assert!(!kube_versions.is_empty());
    println!("Kubernetes versions: {:?}", kube_versions);
}
