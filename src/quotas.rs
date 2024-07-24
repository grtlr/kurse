use k8s_openapi::api::core::v1::ResourceQuota;
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

pub async fn quotas(client: Client) -> anyhow::Result<()> {
    let quotas: Api<ResourceQuota> = Api::default_namespaced(client.clone());
    for q in quotas.list(&ListParams::default()).await? {
        //println!("found quota {}, {:#?}", q.name_any(), q.status);
        print_quota(q);
    }

    Ok(())
}
