use clap::Parser;
use tokio_tungstenite::connect_async;

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
    let (ws_stream, _) = connect_async(args.url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
}
