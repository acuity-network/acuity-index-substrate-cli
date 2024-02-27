use clap::Parser;
use hybrid_api::Index;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URL of Substrate node to connect to
    #[arg(short, long)]
    pub url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let index = Index::connect(args.url).await;
}
