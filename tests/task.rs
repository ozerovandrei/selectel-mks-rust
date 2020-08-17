use selectel_mks::cluster;

pub mod common;

#[test]
fn task_crud() {
    if !common::integration_tests_are_enabled() {
        return;
    }

    // Retrieve needed variables from env.
    let region = common::get_region();
    let kube_version = common::get_kube_version();

    // Prepare MKS client.
    let client = common::setup();

    // Prepare create options.
    let name = "task-crud";
    let create_opts = cluster::schemas::CreateOpts::new(name, &kube_version, &region);

    // Create a new cluster.
    let cluster = common::cluster_common::create_cluster_or_panic(&client, &create_opts);

    // List all tasks.
    let tasks = client
        .list_tasks(&cluster.id)
        .expect("failed to cluster all tasks");
    assert!(!tasks.is_empty());
    println!("All tasks: {:?}\n", tasks);

    // Get the first task.
    let task = client
        .get_task(&cluster.id, &tasks[0].id)
        .expect("failed to get the first task");
    println!("First task: {:?}\n", task);

    // Delete the created cluster.
    common::cluster_common::delete_cluster_or_panic(&client, &cluster.id);
}
