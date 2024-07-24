use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client, ResourceExt};

pub async fn pods(client: Client) -> anyhow::Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&ListParams::default()).await? {
        println!("pod `{}`", p.name_any());
    }

    Ok(())
}
