use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client};

pub async fn get_pods_names() -> Vec<String> {
    let client = Client::try_default()
        .await
        .expect("unable to create kube client");
    let pods: Api<Pod> = Api::default_namespaced(client);

    let params = ListParams::default();
    let p = pods.list(&params).await.expect("unable to list pods");
    p.into_iter().map(|p| p.metadata.name.unwrap()).collect()
}
