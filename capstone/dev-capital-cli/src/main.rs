use std::{fmt::Debug, fs::File, io::Read, path::PathBuf, str::FromStr, collections::HashMap};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use tracing::{debug, info, Level};
use tracing_subscriber;
use solana_sdk::hash::hash;

mod compress;
use compress::LookupTable;

#[tokio::main]
async fn main() -> Result<()> {
    let dev_cli = DevCapitalCli::parse();
    tracing_subscriber::fmt()
        // .with_max_level(Level::DEBUG)
        .with_max_level(match dev_cli.debug > 1 {
            true => Level::DEBUG,
            _ => Level::DEBUG,
        })
        .init();
    debug!("debug mode enabled");
    debug!("{:?}", dev_cli);

    let maybe_disc = hash(b"account:DevDeployData").as_ref()[..8].to_vec();
    debug!("{:?}", &maybe_disc);

    // let compressed = open_file(&dev_cli.program_path.unwrap()).and_then(|program_data| compress_file_5x0(program_data))?;


    debug!("program finished");
    Ok(())
}

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

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    // #[command(subcommand)]
    // command: Option<Commands>,
}

fn open_file(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file_buf = vec![];
    File::open(path).and_then(|mut file| file.read_to_end(&mut file_buf))?;
    Ok(file_buf)
}

fn compress_file_5x0(program_data: Vec<u8>) -> Result<Vec<u8>> {
    let mut lookup_table = LookupTable::new();
    let needle6 = vec![0u8;6];
    let needle5 = vec![0u8;5];
    // let needle = vec![3,4,5,6,7];

    let mut skip_counter = 0;
    let mut found_offsets5= vec![];
    let mut found_offsets6= vec![];

    let mut new_data6:Vec<u8> = vec![];
    for index in 0..program_data.len() {
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }
        if index+needle6.len()<=program_data.len() {
            if program_data[index..index+needle6.len()] == needle6 {
                // debug!("needle {:?}", )
                found_offsets6.push(index);
                skip_counter += needle6.len()-1;
                continue;
            }
        }
        new_data6.push(program_data[index]);
    }
    debug!("data len {}", program_data.len());
    debug!("6 offsets length {}", found_offsets6.len()*3);
    debug!("6 new data len {}", new_data6.len());
    debug!("6 final len {}", new_data6.len()+(found_offsets6.len()*3));
    debug!("{:?}", &found_offsets6[..10]);
    /// EXPAND 
    
    // debug!("orig     \n{:?}", &program_data[..100]);
    // debug!("new      \n{:?}", &new_data6[..100]);

    let mut new_data5:Vec<u8> = vec![];
    for index in 0..new_data6.len() {
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }
        if index+needle5.len()<=new_data6.len() {
            if new_data6[index..index+needle5.len()] == needle5 {
                // debug!("needle {:?}", )
                found_offsets5.push(index);
                skip_counter += needle5.len()-1;
                continue;
            }
        }
        new_data5.push(new_data6[index]);
    }
    // debug!("orig6    \n{:?}", &new_data6[..100]);
    // debug!("new5     \n{:?}", &new_data5[..100]);
    debug!("5 offsets length {}", found_offsets5.len()*3);
    debug!("5 new data len {}", new_data5.len());
    debug!("5 final len {}", new_data5.len()+(found_offsets5.len()*3));
    debug!("{:?}", &found_offsets5[..10]);
    
    for offset in found_offsets5 {
        for _ in 0..needle5.len() {
            new_data5.insert(offset, 0u8);
            // debug!("{}", &new_data.len());
        }
    }

    for offset in found_offsets6 {
        for _ in 0..needle6.len() {
            new_data5.insert(offset, 0u8);
            // debug!("{}", &new_data.len());
        }
    }
    debug!("orig sha256 hash  {}", hash(&program_data));
    debug!("check sha256 hash {}", hash(&new_data5));

    // debug!("expanded \n{:?}", &new_data6[..100]);
    debug!("expanded len {}", new_data5.len());
    
    Ok(vec![])
}
