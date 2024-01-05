use anyhow::Result;
use futures::{FutureExt, StreamExt};
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::bpf_loader_upgradeable::{self, UpgradeableLoaderState};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, account_utils::StateMut};
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const WS_URL: &str = "wss://api.devnet.solana.com";
const DEVNET_URL: &str = "https://api.devnet.solana.com";

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<()> {
    // get all programs owned by the bpf upgradeable loader

    let rpc = RpcClient::new(DEVNET_URL.into());
    let bpf_loader = bpf_loader_upgradeable::id();

    let config = RpcProgramAccountsConfig {
        filters: None,
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::JsonParsed),
            data_slice: None,
            commitment: Some(CommitmentConfig::finalized()),
            min_context_slot: None,
        },
        with_context: None,
    };
    let pubsub_client = PubsubClient::new(WS_URL).await?;
    let (mut stream, _) = pubsub_client
        .program_subscribe(&bpf_loader, Some(config))
        .await?;

    while let Some(acc) = stream.next().await {
        if acc.value.account.executable {
            let program_id = acc.value.pubkey.clone();
            if let Ok(UiAccountData::Json(parsed_acc)) =
                UiAccountData::try_from(acc.value.account.data.clone())
            {
                let program_data_id = parsed_acc.parsed["info"]["programData"]
                    .to_string()
                    .trim_matches('"')
                    .to_string();

                if let Ok(program_data_account) = rpc
                    .get_account(&Pubkey::from_str(&program_data_id).unwrap())
                    .await
                {
                    if let Ok(UpgradeableLoaderState::ProgramData {
                        upgrade_authority_address,
                        slot,
                    }) = program_data_account.state()
                    {
                        if (slot + 100) < acc.context.slot {
                            continue;
                        }
                        println!("acc {:#?}", &acc);
                        println!("program last deploy slot {}", slot);
                        println!("current slot {}", acc.context.slot);
                        fileops(&program_id, &program_data_account.data).await?;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn fileops(program_id: &str, program_data: &Vec<u8>) -> Result<()> {
    File::create(format!("{}.so", program_id))
        .then(|file| async { file?.write_all(&program_data).await })
        .await?;
    Ok(())
}
