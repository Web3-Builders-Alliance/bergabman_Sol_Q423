use anyhow::Result;
use kmp::{kmp_find_with_lsp_table, kmp_table};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};
// use tracing::info;
// use tracing_subscriber;

fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();

    let progs = read_files("./programs_downloaded")?;
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    println!("analysis");
    let analysis = analyse_seqences(progs)?;
    let json_string = serde_json::to_string(&analysis).unwrap();
    let _ = fileops("full_analysis.json", json_string);

    Ok(())
}

fn analyse_seqences(progs: HashMap<String, Vec<u8>>) -> Result<Vec<(String, String, String)>> {
    // sliding window analysis of byte sequences in all files
    let mut table: Vec<(String, String, String)> = vec![];
    let start = std::time::Instant::now();
    let mut adjust = std::time::Instant::now();
    let mut file_count = 0;
    for (program_name, program_data) in progs.iter() {
        file_count += 1;
        println!(
            "Starting {}, file count {} from {}",
            &program_name,
            file_count,
            progs.len()
        );
        // let mut byte_sequences: HashMap<String, u32> = HashMap::new();
        for count in (32..8192).rev() {
            'window: for seq in program_data.windows(count) {
                let mut this_seq = (seq, 1);

                if String::from_utf8(seq.to_vec()).is_ok() {
                    let seq_string = String::from_utf8(seq.to_vec()).unwrap();
                    let kmp_table = kmp_table(seq);
                    let table_appearances: Vec<Option<_>> = table
                        .par_iter()
                        .map(|(seq_string, _, _)| {
                            kmp_find_with_lsp_table(seq, seq_string.as_bytes(), &kmp_table)
                        })
                        .filter(|seq| seq.is_some())
                        .collect();
                    if table_appearances.len() > 0 {
                        continue 'window
                    }
                    // for item in table_appearances.iter(){
                    //     if item.is_some() {
                    //         continue 'window;
                    //     }
                    // }
                    // for (table_item, _, _) in table.iter() {
                    //     if table_item.contains(&seq_string) {
                    //         continue 'window;
                    //     }
                    // }
                    let appearances: Vec<Option<usize>> = progs
                        .par_iter()
                        .map(|(_, program_data)| {
                            kmp_find_with_lsp_table(seq, program_data, &kmp_table)
                        })
                        .collect();
                    for thistime in appearances.iter() {
                        if thistime.is_some() {
                            this_seq.1 += 1;
                        }
                    }

                    if this_seq.1 as f64 > (progs.len() as f64 * 0.7) {
                        let prct = this_seq.1 as f64 / (progs.len() as f64 / 100f64);
                        let this_item = (
                            seq_string.clone(),
                            format!(
                                "appeared {} times in {} files ({}%)",
                                this_seq.1,
                                progs.len(),
                                prct
                            ),
                            format!("len {} bytes", seq.len()),
                        );
                        table.push(this_item);
                    }
                }
            }
            let now = std::time::Instant::now();
            if now.duration_since(adjust).as_secs() > 30 {
                adjust = std::time::Instant::now();
                print!("count: {}; ", count);
                print!("runt {}s; ", now.duration_since(start).as_secs());
                print!("table len {}", &table.len());
            }
        }
        if table.len() > 1 {
            println!("table len {}", &table.len());
            let table_string = serde_json::to_string_pretty(&table).unwrap();
            println!("{}", &table_string);
            fileops(&program_name.to_string(), table_string)?;
        }
    }

    Ok(table)
}

fn read_files(path: &str) -> Result<HashMap<String, Vec<u8>>> {
    let mut files = std::fs::read_dir(path)?;

    let mut file_contents = HashMap::new();

    while let Some(Ok(entry)) = files.next() {
        if entry.file_type()?.is_file() {
            let file_name: String = entry.file_name().to_str().unwrap().into(); //.into();
            let mut file = File::open(entry.path())?;
            let mut contents = vec![];

            file.read_to_end(&mut contents)?;
            file_contents.insert(file_name, contents);
        }
    }

    Ok(file_contents)
}

fn fileops(filename: &str, analysis_json: String) -> Result<()> {
    File::create(format!("{}_analysis.json", filename))
        .and_then(|mut file| file.write_all(&analysis_json.as_bytes()))?;
    Ok(())
}
