use k8s_openapi::api::core::v1::{Pod, PodStatus, ResourceQuota};
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};

fn print_quota(q: ResourceQuota) {
    println!("Quota `{}`", q.name_any());
    println!("----------");
    if let Some(status) = q.status {
        match (status.used, status.hard) {
            (Some(used), Some(hard)) => {
                for (used_name, used_quantity) in used.iter() {
                    println!(
                        "{used_name}: {}/{}",
                        used_quantity.0,
                        hard.get(used_name).unwrap().0
                    )
                }
            }
            _ => unimplemented!(),
        }
    } else {
        println!("no status available")
    }
    println!("");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    let quotas: Api<ResourceQuota> = Api::default_namespaced(client.clone());
    for q in quotas.list(&ListParams::default()).await? {
        //println!("found quota {}, {:#?}", q.name_any(), q.status);
        print_quota(q);
    }

    // println!("Pod status");
    // println!("----------");
    // // Read pods in the configured namespace into the typed interface from k8s-openapi
    // let pods: Api<Pod> = Api::default_namespaced(client);
    // for p in pods.list(&ListParams::default()).await? {
    //     println!("pod `{}`: {:#?}", p.name_any(), p.status.and_then(|s: PodStatus| Some(s)));
    // }
    Ok(())
}
