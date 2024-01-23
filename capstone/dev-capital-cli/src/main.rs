use std::{collections::HashMap, fmt::Debug, fs::File, io::Read, path::PathBuf, str::FromStr};

use amplify_num::u24;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::{hash, Hash},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    system_program,
};
use tracing::{debug, info, Level};
use tracing_subscriber;

mod compress;
mod program_idl;
use crate::program_idl::{DevCapitalProgram, InitDevDeployArgs};

const DEVNET_URL: &str = "https://api.devnet.solana.com";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DevCapitalCli {
    /// Operation we want to do (deploy)
    op: String,
    /// Optional PATH to program
    #[arg(short, long, value_name = "program_path")]
    program_path: Option<PathBuf>,

    /// PATH to keypair (json)
    #[arg(short, long, value_name = "keypair_path")]
    keypair_path: Option<PathBuf>,

    #[arg(short, long, value_name = "funder_pubkey")]
    funder_pubkey: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    // #[command(subcommand)]
    // command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dev_cli = DevCapitalCli::parse();
    tracing_subscriber::fmt()
        .with_max_level(match dev_cli.debug > 1 {
            true => Level::DEBUG,
            _ => Level::DEBUG, // must be INFO in prod
        })
        .init();
    debug!("debug mode enabled");
    debug!("{:?}", dev_cli);

    let dev_keypair = read_keypair_file(dev_cli.keypair_path.as_ref().unwrap()).expect(&format!(
        "Can't open keypair file {:?}",
        &dev_cli.keypair_path
    ));
    let funder_pubkey = Pubkey::from_str(&dev_cli.funder_pubkey.unwrap()).unwrap();
    let original_program_bytes = open_file(&dev_cli.program_path.unwrap())?;
    let (offsets_6, compressed_6) = compress_data(&original_program_bytes, 6)?;
    let (offsets_5, compressed_6and5) = compress_data(&compressed_6, 5)?;
    let mut test_decomp = compressed_6and5.clone();

    decompress_data(&offsets_5, 5, &mut test_decomp)?;
    decompress_data(&offsets_6, 6, &mut test_decomp)?;

    debug!("orig sha256 hash  {}", hash(&original_program_bytes));
    debug!("decomp sha256 hash {}", hash(&test_decomp));

    let rpc_client = RpcClient::new("http://localhost:8899".into());
    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    init_dev_deploy(
        (offsets_6.len() * 3) as u32,
        (offsets_5.len() * 3) as u32,
        original_program_bytes.len() as u32,
        &rpc_client,
        recent_blockhash,
        &dev_keypair,
        &funder_pubkey,
    )
    .await?;

    // send_data(
    //     offsets_6,
    //     offsets_5,
    //     compressed_6and5,
    //     original_program_bytes.len(),
    // )
    // .await?;

    debug!("program finished");
    Ok(())
}

async fn init_dev_deploy(
    offsets_6_len: impl Into<u32>,
    offsets_5_len: impl Into<u32>,
    original_len: impl Into<u32>,
    rpc_client: &RpcClient,
    recent_hash: Hash,
    signer: &Keypair,
    funder: &Pubkey,
) -> Result<()> {
    let dev_fund = DevCapitalProgram::derive_program_address(&[
        b"dev_fund",
        funder.as_ref(),
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let dev_config = DevCapitalProgram::derive_program_address(&[
        b"dev_config",
        dev_fund.as_ref(),
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let deploy_offsets = DevCapitalProgram::derive_program_address(&[
        b"deploy_offsets",
        dev_fund.as_ref(),
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let deploy_data = DevCapitalProgram::derive_program_address(&[
        b"deploy_data",
        dev_fund.as_ref(),
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let args = InitDevDeployArgs {
        ot_6_len: offsets_6_len.into(),
        ot_5_len: offsets_5_len.into(),
        orig_len: original_len.into(),
    };

    let tx = DevCapitalProgram::init_dev_deploy(
        &[
            &signer.pubkey(),
            &dev_fund,
            &dev_config,
            &deploy_offsets,
            &deploy_data,
            &system_program::id(),
        ],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        recent_hash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .await
        .expect("Failed to send transaction");

    info!("InitDevDeploy tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", signature, );

    Ok(())
}

async fn send_data(
    offsets_6: Vec<u8>,
    offsets_5: Vec<u8>,
    compressed_data: Vec<u8>,
    orig_len: usize,
) -> Result<()> {
    let mut blob: Vec<u8> = vec![];
    let offsets_6_len: u24 = u24::try_from(offsets_6.len() as u32 / 3 as u32).unwrap();
    let offsets_5_len: u24 = u24::try_from(offsets_5.len() as u32 / 3 as u32).unwrap();
    blob.extend_from_slice(&offsets_6_len.to_le_bytes());
    blob.extend_from_slice(&offsets_6);
    blob.extend_from_slice(&offsets_5_len.to_le_bytes());
    blob.extend_from_slice(&offsets_5);
    // blob.extend_from_slice(&compressed_data);
    let offsets_chunks = split_to_chunks(&blob, 1000);
    let data_chunks = split_to_chunks(&compressed_data, 1000);
    debug!("Offsets chunks len {}", offsets_chunks.len());
    debug!("Data chunks len {}", data_chunks.len());

    // let ix =  ;

    Ok(())
}

fn split_to_chunks(data: &Vec<u8>, chunk_size: usize) -> Vec<(u16, Vec<u8>)> {
    let mut data_chunks: Vec<(u16, Vec<u8>)> = vec![];
    for (index, chunk) in data.chunks(chunk_size).enumerate() {
        data_chunks.push((index as u16, chunk.to_vec()));
    }
    data_chunks
}

fn open_file(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file_buf = vec![];
    File::open(path).and_then(|mut file| file.read_to_end(&mut file_buf))?;
    Ok(file_buf)
}

fn compress_data(data: &Vec<u8>, needle_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
    let needle = vec![0u8; needle_len];
    let mut found_offsets: Vec<u8> = vec![];
    let mut compressed_data: Vec<u8> = vec![];
    let mut skip_counter = 0;

    for index in 0..data.len() {
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }
        if index + needle.len() <= data.len() {
            if data[index..index + needle.len()] == needle {
                // debug!("needle {:?}", )
                // found_offsets.push(u24::try_from(index as u32).unwrap().to_le_bytes());
                found_offsets
                    .append(&mut u24::try_from(index as u32).unwrap().to_le_bytes().to_vec());
                skip_counter += needle.len() - 1;
                continue;
            }
        }
        compressed_data.push(data[index]);
    }

    Ok((found_offsets, compressed_data))
}

fn decompress_data(offsets: &Vec<u8>, length: u8, compressed_data: &mut Vec<u8>) -> Result<()> {
    for offset_ in offsets.chunks_exact(3) {
        let mut offset_bytes: [u8; 3] = [0u8; 3];
        offset_bytes.copy_from_slice(offset_);
        let offset = u24::from_le_bytes(offset_bytes);

        for _ in 0..length {
            compressed_data.insert(offset.into(), 0u8);
            // debug!("{}", &new_data.len());
        }
    }
    Ok(())
}
