use byte_unit::{Byte, UnitType};
use clap::{Parser, Subcommand, ValueEnum};
use hybrid_api::{Bytes32, Index, Key, SubstrateKey};
use std::str::FromStr;
use subxt::utils::AccountId32;

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
    /// Query how much disk space is used by the index
    SizeOnDisk,
    /// Query for event variants the chain supports
    GetVariants,
    /// Query for events with a key
    GetEvents {
        /// Key type to search for
        #[arg(long)]
        key_type: KeyType,
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, ValueEnum)]
pub enum KeyType {
    AccountId,
    AccountIndex,
    BountyIndex,
    EraIndex,
    MessageId,
    PoolId,
    PreimageHash,
    ProposalHash,
    ProposalIndex,
    RefIndex,
    RegistrarIndex,
    SessionIndex,
    TipHash,
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
            println!(
                "Size on disk: {}",
                Byte::from_u64(size).get_appropriate_unit(UnitType::Binary)
            );
        }
        Commands::GetVariants => {
            let variants = index.get_variants().await;
            println!("Variants: {:?}", variants);
        }
        Commands::GetEvents { key_type, key } => match key_type {
            KeyType::AccountId => {
                let account_id = AccountId32::from_str(key).unwrap();
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::AccountId(Bytes32(
                        account_id.0,
                    ))))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyType::AccountIndex => {}
            KeyType::BountyIndex => {}
            KeyType::EraIndex => {}
            KeyType::MessageId => {}
            KeyType::PoolId => {}
            KeyType::PreimageHash => {}
            KeyType::ProposalHash => {}
            KeyType::ProposalIndex => {}
            KeyType::RefIndex => {}
            KeyType::RegistrarIndex => {}
            KeyType::SessionIndex => {}
            KeyType::TipHash => {}
        },
    }
}
