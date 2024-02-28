use byte_unit::{Byte, UnitType};
use clap::{Parser, Subcommand};
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
        #[command(subcommand)]
        command: KeyCommands,
    },
}

#[derive(Debug, Subcommand)]
enum KeyCommands {
    /// AccountId
    AccountId {
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
    /// AccountIndex
    AccountIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// BountyIndex
    BountyIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// EraIndex
    EraIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// MessageId
    MessageId {
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
    /// PoolId
    PoolId {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// PreimageHash
    PreimageHash {
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
    /// ProposalHash
    ProposalHash {
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
    /// ProposalIndex
    ProposalIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// RefIndex
    RefIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// RegistrarIndex
    RegistrarIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// SessionIndex
    SessionIndex {
        /// Key to search for
        #[arg(short, long)]
        key: u32,
    },
    /// TipHash
    TipHash {
        /// Key to search for
        #[arg(short, long)]
        key: String,
    },
    /// Variant
    Variant {
        /// Key type to search for
        #[arg(short, long)]
        pallet_id: u8,
        /// Key to search for
        #[arg(short, long)]
        variant_id: u8,
    },
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
        Commands::GetEvents { command } => match command {
            KeyCommands::AccountId { key } => {
                let account_id = AccountId32::from_str(key).unwrap();
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::AccountId(Bytes32(
                        account_id.0,
                    ))))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::AccountIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::AccountIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::BountyIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::BountyIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::EraIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::EraIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::MessageId { key } => {}
            KeyCommands::PoolId { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::PoolId(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::PreimageHash { key } => {}
            KeyCommands::ProposalHash { key } => {}
            KeyCommands::ProposalIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::ProposalIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::RefIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::RefIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::RegistrarIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::RegistrarIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::SessionIndex { key } => {
                let events = index
                    .get_events(Key::Substrate(SubstrateKey::SessionIndex(*key)))
                    .await;
                println!("Events: {:?}", events);
            }
            KeyCommands::TipHash { key } => {}
            KeyCommands::Variant {
                pallet_id,
                variant_id,
            } => {
                let events = index
                    .get_events(Key::Variant(*pallet_id, *variant_id))
                    .await;
                println!("Events: {:?}", events);
            }
        },
    }
}
