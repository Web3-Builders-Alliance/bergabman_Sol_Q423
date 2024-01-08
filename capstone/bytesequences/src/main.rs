use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD_NO_PAD as b64, Engine as _};
use chrono::Utc;
// use kmp::{kmp_find_with_lsp_table, kmp_table};
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};
use tracing::info;
use tracing_subscriber;

use boyer_moore_magiclen::BMByte;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let progs = read_files("./programs_downloaded")?;
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();
    info!("analysis");
    let analysis = analyse_seqences(progs)?;
    let json_string = serde_json::to_string(&analysis).unwrap();
    fileops("full_analysis.json", json_string)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]

struct Table(Vec<Sequence>);

impl Table {
    fn new() -> Self {
        Self(vec![])
    }

    fn save_to_json(&self) -> Result<()> {
        let json_friendly: Vec<_> = self
            .0
            .iter()
            .map(|s| Sequence {
                seq_str_lossy: String::from_utf8_lossy(s.seq.as_slice()).to_string(),
                seq: vec![],
                ..s.clone()
            })
            .collect();

        fileops("".into(), serde_json::to_string_pretty(&json_friendly)?)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Sequence {
    length: usize,
    appear: usize,
    seq_str_lossy: String,
    seq_b64: String,
    seq: Vec<u8>,
}

impl Sequence {
    fn new(seq: Vec<u8>, appear: usize) -> Self {
        let length = seq.len().clone();
        let seq_str_lossy = "".to_string();
        let seq_b64 = b64.encode(&seq);
        Self {
            length,
            appear,
            seq_str_lossy,
            seq_b64,
            seq,
        }
    }
}

#[allow(unused_assignments)]
fn analyse_seqences(mut progs: Vec<Vec<u8>>) -> Result<Table> {
    // sliding window analysis of byte sequences in all files

    // let mut table: Vec<(String, String, String)> = vec![];
    // let mut table: Vec<Sequence> = vec![];
    progs.sort_by(|a, b| b.len().cmp(&a.len()));
  
    let mut table: Table = Table::new();
    let start = std::time::Instant::now();
    let mut adjust = std::time::Instant::now();
    let mut file_count = 0;
    let min_prcnt = (progs.len() as f64 / 100f64) * 70f64;

    for program_data in progs.iter() {
        file_count += 1;
        info!("Starting {} from {}", file_count, progs.len());
        for count in (64..1024).rev() {
            let mut found_seq_skip = 0;
            'window: for seq in program_data.windows(count) {
                if found_seq_skip > 0 {
                    // info!("skip {}", &found_seq_skip);
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
                        &table.0.len()
                    );
                }

                // let _kmp_table = kmp_table(seq);
                let table_appearances = table.0.par_iter().find_map_any(|sequence| {
                    let bmb = BMByte::from(seq.to_vec()).unwrap();
                    bmb.find_first_in(&sequence.seq)
                    // kmp_find_with_lsp_table(seq, &sequence.seq, &kmp_table)
                });

                if table_appearances.is_some() {
                    continue 'window;
                }

                let appearances: Vec<Option<usize>> = progs
                    .par_iter()
                    .map(|program_data| {
                        let bmb = BMByte::from(seq.to_vec()).unwrap();
                        bmb.find_first_in(&program_data)
                        // kmp_find_with_lsp_table(seq, program_data, &kmp_table)
                    })
                    .collect();

                let this_seq_appear = appearances
                    .par_iter()
                    .map(|seen| if seen.is_some() { 1 } else { 0 })
                    .sum();

                if this_seq_appear as f64 > min_prcnt {
                    let this_seq = Sequence::new(seq.to_vec(), this_seq_appear);
                    info!(
                        "{}\nappeared: {} times",
                        this_seq.seq_str_lossy, this_seq.appear
                    );
                    found_seq_skip = seq.len();
                    table.0.push(this_seq);
                    table.save_to_json()?;
                    // fileops(&format!(""), serde_json::to_string_pretty(&table)?)?;
                }
            }
        }
        if table.0.len() > 1 {
            table.save_to_json()?;
            info!("file saved");
        }
    }

    Ok(table)
}

fn read_files(path: &str) -> Result<Vec<Vec<u8>>> {
    let mut files = std::fs::read_dir(path)?;

    let mut file_contents = vec![];

    while let Some(Ok(entry)) = files.next() {
        if entry.file_type()?.is_file() {
            let file_name: String = entry
                .file_name()
                .to_str()
                .ok_or(anyhow!("filename"))?
                .into();
            let mut file = File::open(entry.path())?;
            let mut contents = vec![];

            file.read_to_end(&mut contents)?;
            info!(
                "Size before trimming 0s {}; file {}",
                &contents.len(),
                &file_name
            );
            while contents.ends_with(b"\0") {
                contents.pop();
            }
            info!("Size after trimming 0s {}", &contents.len());
            // file_contents.insert(file_name, contents);
            file_contents.push(contents);
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
