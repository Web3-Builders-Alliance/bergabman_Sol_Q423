use std::{collections::HashMap, fmt::Debug, fs::File, io::Read, path::PathBuf, str::FromStr};

use amplify_num::u24;
use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::{stream::FuturesUnordered, StreamExt};
use solana_client::nonblocking::rpc_client::{self, RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::{hash, Hash},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
    system_program,
    transaction::Transaction,
};
use tracing::{debug, info, level_filters::LevelFilter, Level};
use tracing_subscriber::{self, EnvFilter};

mod compress;
mod program_idl;
use crate::program_idl::{
    DeployDataArgs, DeployOffsetsArgs, DevCapitalProgram, InitDevConfigArgs, InitDevFundArgs
};

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

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env()?
        .add_directive("hyper=info".parse()?)
        .add_directive("reqwest=info".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        // .with_max_level(match dev_cli.debug > 1 {
        //     true => Level::DEBUG,
        //     _ => Level::DEBUG, // must be INFO in prod
        // })
        .init();
    debug!("debug mode enabled");
    debug!("{:?}", dev_cli);

    let dev_keypair = read_keypair_file(dev_cli.keypair_path.as_ref().unwrap()).expect(&format!(
        "Can't open keypair file {:?}",
        &dev_cli.keypair_path
    ));

    let funder_keypair = read_keypair_file("funG3rQBtHCPR5D3DHsnNgaZenv3nujgNkgzonu2raj.json")
        .expect(&format!(
            "Can't open keypair file {:?}",
            &dev_cli.keypair_path
        ));

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

    let dev_fund_pda = DevCapitalProgram::derive_program_address(&[
        b"dev_fund",
        funder_keypair.pubkey().as_ref(),
        dev_keypair.pubkey().as_ref(),
    ]);

    let dev_config_pda = DevCapitalProgram::derive_program_address(&[
        b"dev_config",
        dev_fund_pda.as_ref(),
        dev_keypair.pubkey().as_ref(),
    ]);

    let deploy_offsets_pda = DevCapitalProgram::derive_program_address(&[
        b"deploy_offsets",
        dev_fund_pda.as_ref(),
        dev_keypair.pubkey().as_ref(),
    ]);

    let deploy_data_pda = DevCapitalProgram::derive_program_address(&[
        b"deploy_data",
        dev_fund_pda.as_ref(),
        dev_keypair.pubkey().to_bytes().as_ref(),
    ]);

    if rpc_client
        .get_account_with_commitment(&dev_config_pda, CommitmentConfig::confirmed())
        .await?
        .value
        .is_none()
    {
        init_dev_config(
            (offsets_6.len() * 3) as u32,
            (offsets_5.len() * 3) as u32,
            compressed_6and5.len() as u32,
            original_program_bytes.len() as u32,
            &rpc_client,
            recent_blockhash,
            &dev_keypair,
            &funder_keypair,
            &dev_fund_pda,
            &dev_config_pda,
            &deploy_offsets_pda,
            &deploy_data_pda,
        )
        .await?;
    } else {
        debug!("Dev config already initialized")
    }

    size_accounts(
        offsets_6.len() as u32,
        offsets_5.len() as u32,
        original_program_bytes.len() as u32,
        &rpc_client,
        recent_blockhash,
        &dev_keypair,
        &dev_fund_pda,
        &dev_config_pda,
        &deploy_offsets_pda,
        &deploy_data_pda,
    )
    .await?;

    let (offsets_chunks, data_chunks) = pack_it(offsets_6, offsets_5, compressed_6and5)?;

    deploy_offsets(
        &rpc_client,
        recent_blockhash,
        &dev_keypair,
        &dev_fund_pda,
        &dev_config_pda,
        &deploy_offsets_pda,
        &deploy_data_pda,
        offsets_chunks,
    )
    .await?;

    deploy_data(
        &rpc_client,
        recent_blockhash,
        &dev_keypair,
        &dev_fund_pda,
        &dev_config_pda,
        &deploy_offsets_pda,
        &deploy_data_pda,
        data_chunks,
    )
    .await?;

    debug!("program finished");
    Ok(())
}

async fn size_accounts(
    offsets_6_len: impl Into<u32>,
    offsets_5_len: impl Into<u32>,
    original_len: impl Into<u32>,
    rpc_client: &RpcClient,
    recent_blockhash: Hash,
    dev: &Keypair,
    dev_fund_pda: &Pubkey,
    dev_config_pda: &Pubkey,
    deploy_offsets_pda: &Pubkey,
    deploy_data_pda: &Pubkey,
) -> Result<()> {
    let offsets_pda_len = 3 + 3 + ((offsets_5_len.into() * 3) + (offsets_6_len.into() * 3)); // 3+3 is the length of offset tables individually, u24+u24
    let data_pda_len = original_len.into();
    let mut account_resize_ixs = vec![];

    let offset_ix = DevCapitalProgram::account_size_offsets_ix(&[
        &dev.pubkey(),
        &dev_fund_pda,
        &dev_config_pda,
        &deploy_offsets_pda,
    ]);

    let mut o_count = 0;

    while o_count * 10240 < offsets_pda_len {
        o_count += 1;
        account_resize_ixs.push(offset_ix.clone());
    }
    debug!("offsets len {}", account_resize_ixs.len());

    let data_ix = DevCapitalProgram::account_size_data_ix(&[
        &dev.pubkey(),
        &dev_fund_pda,
        &dev_config_pda,
        &deploy_data_pda,
    ]);

    let mut d_count = 0;

    while d_count * 10240 < data_pda_len {
        d_count += 1;
        account_resize_ixs.push(data_ix.clone());
    }

    let account_resize_tx = Transaction::new_signed_with_payer(
        &account_resize_ixs,
        Some(&dev.pubkey()),
        &[dev],
        recent_blockhash,
    );

    let size = &account_resize_tx.message_data().len();
    debug!("resize tx size {}", size);
    debug!("{:?}", account_resize_tx);

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &account_resize_tx,
            CommitmentConfig::processed(),
        )
        .await
        .expect("Failed to send transaction");

    info!("AccountSizeData tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", signature);

    Ok(())
}

async fn deploy_offsets(
    rpc_client: &RpcClient,
    recent_blockhash: Hash,
    dev: &Keypair,
    dev_fund: &Pubkey,
    dev_config: &Pubkey,
    deploy_offsets: &Pubkey,
    deploy_data: &Pubkey,
    offsets_chunks: Vec<(u16, Vec<u8>)>,
) -> Result<()> {
    let mut txs = FuturesUnordered::new();
    let chunks_len = offsets_chunks.len();
    let mut toggled = false;
    for (mut index, mut chunk) in offsets_chunks {
        let mut this_chunk = vec![];
        this_chunk.append(&mut index.to_le_bytes().to_vec());
        this_chunk.append(&mut chunk);

        let args = DeployOffsetsArgs { data: this_chunk };

        // let ix = DevCapitalProgram::deploy_offsets_ix(&[
        //     &dev.pubkey(),
        //     dev_fund,
        //     dev_config,
        //     deploy_offsets,
        //     deploy_data,

        // ], &args);

        let tx = DevCapitalProgram::deploy_offsets(
            &[
                &dev.pubkey(),
                dev_fund,
                dev_config,
                deploy_offsets,
                deploy_data,
            ],
            &args,
            Some(&dev.pubkey()),
            &[dev],
            recent_blockhash,
        );
        if !toggled {
            toggled = true;
            debug!("offset tx {:?}", &tx);
        }
        // let tx = rpc_client.send_transaction(&tx);
        txs.push(send_tx(rpc_client, tx));
    }

    let mut success_counter = 0;
    while let Some(sig_result) = txs.next().await {
        if let Ok(sig) = sig_result {
            info!("DeployOffsets tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", sig);
            success_counter += 1;
        }
    }

    debug!("offsets deploy success {}, chunks len {}", success_counter, chunks_len);

    Ok(())
}

async fn send_tx(rpc_client: &RpcClient, tx: Transaction) -> Result<Signature> {
    let sig = rpc_client.send_transaction(&tx).await?;
    Ok(sig)
}

async fn deploy_data(
    rpc_client: &RpcClient,
    recent_blockhash: Hash,
    dev: &Keypair,
    dev_fund: &Pubkey,
    dev_config: &Pubkey,
    deploy_offsets: &Pubkey,
    deploy_data: &Pubkey,
    data_chunks: Vec<(u16, Vec<u8>)>,
) -> Result<()> {
    let mut txs = FuturesUnordered::new();
    let chunks_len = data_chunks.len();
    let mut toggled = false;
    for (mut index, mut chunk) in data_chunks {
        let mut this_chunk = vec![];
        this_chunk.append(&mut index.to_le_bytes().to_vec());
        this_chunk.append(&mut chunk);

        let args = DeployDataArgs { data: this_chunk };

        let tx = DevCapitalProgram::deploy_data(
            &[
                &dev.pubkey(),
                dev_fund,
                dev_config,
                deploy_offsets,
                deploy_data,
            ],
            &args,
            Some(&dev.pubkey()),
            &[dev],
            recent_blockhash,
        );
        if !toggled {
            toggled = true;
            debug!("data tx {:?}", &tx);
        }
        txs.push(send_tx(rpc_client, tx));
    }

    let mut success_counter = 0;
    while let Some(sig_result) = txs.next().await {
        if let Ok(sig) = sig_result {
            info!("DeployData tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", sig);
            success_counter += 1;
        }
    }

    debug!("data deploy success {}, chunks len {}", success_counter, chunks_len);

    Ok(())
}

async fn init_dev_config(
    offsets_6_len: impl Into<u32>,
    offsets_5_len: impl Into<u32>,
    compressed_len: impl Into<u32>,
    original_len: impl Into<u32>,
    rpc_client: &RpcClient,
    recent_blockhash: Hash,
    dev: &Keypair,
    funder: &Keypair,
    dev_fund: &Pubkey,
    dev_config: &Pubkey,
    deploy_offsets: &Pubkey,
    deploy_data: &Pubkey,
) -> Result<()> {
    let args_fund = InitDevFundArgs {
        lamports: 30 * LAMPORTS_PER_SOL,
    };

    let tx_fund = DevCapitalProgram::init_dev_fund(
        &[
            &funder.pubkey(),
            &dev.pubkey(),
            &dev_fund,
            &system_program::id(),
        ],
        &args_fund,
        Some(&funder.pubkey()),
        &[&funder],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&tx_fund)
        .await
        .expect("Failed to send transaction");

    info!("InitDevFund tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", signature, );

    let args_config = InitDevConfigArgs {
        ot_6_len: offsets_6_len.into(),
        ot_5_len: offsets_5_len.into(),
        comp_len: compressed_len.into(),
        orig_len: original_len.into(),
    };

    let tx_config = DevCapitalProgram::init_dev_config(
        &[
            &dev.pubkey(),
            &dev_fund,
            &dev_config,
            &deploy_offsets,
            &deploy_data,
            &system_program::id(),
        ],
        &args_config,
        Some(&dev.pubkey()),
        &[&dev],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&tx_config)
        .await
        .expect("Failed to send transaction");

    info!("InitDevDeploy tx https://explorer.solana.com/transaction/{}?cluster=custom&customUrl=http://localhost:8899", signature, );

    Ok(())
}

fn pack_it(
    offsets_6: Vec<u8>,
    offsets_5: Vec<u8>,
    compressed_data: Vec<u8>,
) -> Result<(Vec<(u16, Vec<u8>)>, Vec<(u16, Vec<u8>)>)> {
    let mut blob: Vec<u8> = vec![];
    let offsets_6_len: u24 = u24::try_from(offsets_6.len() as u32 / 3 as u32).unwrap();
    let offsets_5_len: u24 = u24::try_from(offsets_5.len() as u32 / 3 as u32).unwrap();
    blob.extend_from_slice(&offsets_6_len.to_le_bytes());
    blob.extend_from_slice(&offsets_6);
    blob.extend_from_slice(&offsets_5_len.to_le_bytes());
    blob.extend_from_slice(&offsets_5);
    let offsets_chunks = split_to_chunks(&blob, 900);
    let data_chunks = split_to_chunks(&compressed_data, 900);
    debug!("Offsets chunks len {}", offsets_chunks.len());
    debug!("Data chunks len {}", data_chunks.len());

    Ok((offsets_chunks, data_chunks))
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
