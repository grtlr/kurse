use clap::{Parser, Subcommand};
use kube::Client;

mod pods;
mod quotas;

#[derive(Subcommand)]
enum Command {
    Quotas,
    Pods,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    match cli.command {
        Command::Quotas => quotas::quotas(client.clone()).await?,
        Command::Pods => pods::pods(client).await?,
    }

    Ok(())
}
