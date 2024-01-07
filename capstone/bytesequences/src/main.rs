use anyhow::{Result, anyhow};
use base64::{engine::general_purpose::STANDARD_NO_PAD as b64, Engine as _};
use chrono::Utc;
use kmp::{kmp_find_with_lsp_table, kmp_table};
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};
use tracing::info;
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let progs = read_files("./programs_downloaded")?;
    // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    info!("analysis");
    let analysis = analyse_seqences(progs)?;
    let json_string = serde_json::to_string(&analysis).unwrap();
    fileops("full_analysis.json", json_string)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Sequence {
    seq: Vec<u8>,
    length: usize,
    appear: usize,
    seq_b64: String,
    seq_str: Option<String>,
}
impl Sequence {
    fn new(seq: Vec<u8>, appear: usize) -> Self {
        let seq_b64 = b64.encode(&seq);
        let length = seq.len().clone();
        let seq_str =
            String::from_utf8(seq.clone().to_vec()).map_or_else(|_| None, |str| Some(str));
        Self {
            seq,
            length,
            appear,
            seq_b64,
            seq_str,
        }
    }
}

#[allow(unused_assignments)]
fn analyse_seqences(progs: HashMap<String, Vec<u8>>) -> Result<Vec<Sequence>> {
    // sliding window analysis of byte sequences in all files

    // let mut table: Vec<(String, String, String)> = vec![];
    let mut table: Vec<Sequence> = vec![];
    let start = std::time::Instant::now();
    let mut adjust = std::time::Instant::now();
    let mut file_count = 0;
    let min_prcnt = (progs.len() as f64 / 100f64) * 70f64;
    
    for (program_name, program_data) in progs.iter() {
        file_count += 1;
        info!(
            "Starting {}, file count {} from {}",
            &program_name,
            file_count,
            progs.len()
        );
        for count in (64..1024).rev() {
            let mut found_seq_skip = 0;
            'window: for seq in program_data.windows(count) {
                if found_seq_skip > 0 {
                    found_seq_skip -= 1;
                    continue 'window;
                }
                let now = std::time::Instant::now();
                if now.duration_since(adjust).as_secs() > 180 {
                    adjust = std::time::Instant::now();
                    info!(
                        "seq_len: {}; runt {}s; table_len {} ",
                        count,
                        now.duration_since(start).as_secs(),
                        &table.len()
                    );
                }

                let kmp_table = kmp_table(seq);
                let table_appearances = table.par_iter().find_map_any(|sequence| {
                    kmp_find_with_lsp_table(seq, &sequence.seq, &kmp_table)
                });

                if table_appearances.is_some() {
                    continue 'window;
                }

                let appearances: Vec<Option<usize>> = progs
                    .par_iter()
                    .map(|(_, program_data)| kmp_find_with_lsp_table(seq, program_data, &kmp_table))
                    .collect();

                let this_seq_appear = appearances
                    .par_iter()
                    .map(|seen| if seen.is_some() { 1 } else { 0 })
                    .sum();

                if this_seq_appear as f64 > min_prcnt {
                    info!("seq_appeared: {} times", this_seq_appear);
                    found_seq_skip = seq.len();
                    table.push(Sequence::new(seq.to_vec(), this_seq_appear));
                    fileops(&program_name.to_string().replace(".so", ""), serde_json::to_string(&table)?)?;
                }
            }
        }
        if table.len() > 1 {
            fileops(&program_name.to_string().replace(".so", ""), serde_json::to_string(&table)?)?;
            info!("file saved {}", &program_name.to_string());
        }
    }

    Ok(table)
}

fn read_files(path: &str) -> Result<HashMap<String, Vec<u8>>> {
    let mut files = std::fs::read_dir(path)?;

    let mut file_contents = HashMap::new();

    while let Some(Ok(entry)) = files.next() {
        if entry.file_type()?.is_file() {
            let file_name = entry.file_name().to_str().ok_or(anyhow!("filename"))?.into(); 
            let mut file = File::open(entry.path())?;
            let mut contents = vec![];

            file.read_to_end(&mut contents)?;
            info!("Size before trimming 0s {}; file {}", &contents.len(), &file_name);
            while contents.ends_with(b"\0") {
                contents.pop();
            }
            info!("Size after trimming 0s {}", &contents.len());
            file_contents.insert(file_name, contents);
        }
    }

    Ok(file_contents)
}

fn fileops(filename: &str, input_string: String) -> Result<()> {
    let time = Utc::now();
    File::create(format!(
        "{}_{}.json",
        time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        filename
    ))
    .and_then(|mut file| file.write_all(&input_string.as_bytes()))?;
    Ok(())
}
