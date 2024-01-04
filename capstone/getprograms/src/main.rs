use anyhow::Result;
use futures::prelude::stream::StreamExt;
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const WS_URL: &str = "wss://api.devnet.solana.com";
const DEVNET_URL: &str = "https://api.devnet.solana.com";

#[tokio::main]
async fn main() -> Result<()> {
    // get all programs owned by the bpf upgradeable loader

    let rpc = RpcClient::new(DEVNET_URL.into());
    // let program_accs = rpc.get_program_accounts_with_config(&bpf_loader).await?;
    // println!("program accs len {}", program_accs.len());

    let bpf_loader = Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111").unwrap();

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
            let program_id = acc.value.pubkey;
            if let Ok(UiAccountData::Json(parsedacc)) =
                UiAccountData::try_from(acc.value.account.data)
            {
                let program_data_id: String = parsedacc.parsed["info"]["programData"]
                    .to_string()
                    .trim_matches('"')
                    .to_string();

                if let Ok(program_data) = rpc
                    .get_account_data(&Pubkey::from_str(&program_data_id).unwrap())
                    .await
                {
                    let mut file = File::create(format!("{}.so", program_id)).await?;
                    file.write_all(&program_data).await?;
                }
            }
        }
    }
    Ok(())
}
