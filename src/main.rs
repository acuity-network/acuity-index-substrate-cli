use clap::{Parser, Subcommand};
use hybrid_api::Index;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// URL of Substrate node to connect to
    #[arg(short, long)]
    pub url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Indexer status.
    Status,
    /// Query how much disk space is used by the index.
    SizeOnDisk,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut index = Index::connect(cli.url).await;

    match &cli.command {
        Commands::Status => {
            let status = index.status().await;
            println!("{}", status);
        }
        Commands::SizeOnDisk => {
            let size = index.size_on_disk().await;
            println!("Size on disk: {}", size);
        }
    }
}
