use anyhow::Result;
use kmp::{kmp_find_with_lsp_table, kmp_table};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

fn main() -> Result<()> {
    let progs = read_files("./programs_downloaded")?;
    println!("analysis");
    let analysis = analyse_seqences(progs)?;
    println!("writing");
    let json_string = serde_json::to_string(&analysis).unwrap();
    let _ = fileops("full_analysis.json", json_string);

    Ok(())
}

fn analyse_seqences(progs: HashMap<String, Vec<u8>>) -> Result<Vec<(String, String, String)>> {
    // sliding window analysis of byte sequences in all files
    let mut table = vec![];
    for (_program_name, program_data) in progs.iter() {
        // let mut byte_sequences: HashMap<String, u32> = HashMap::new();
        for count in (32..256).rev() {
            for seq in program_data.windows(count) {
                let mut this_seq = (seq, 1);
                if String::from_utf8(seq.to_vec()).is_ok() {
                    let seq_string = String::from_utf8(seq.to_vec()).unwrap();
                    let kmp_table = kmp_table(seq);
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

                    if this_seq.1 > ((progs.len() * 100) / 70) {
                        let this_item = (
                            seq_string.clone(),
                            format!("{} appearances", this_seq.1),
                            format!("{} len", seq.len()),
                        );
                        if !table.contains(&this_item) {
                            let mut shorter_seq = false;
                            for table_item in &table {
                                if table_item.0.contains(&seq_string) {
                                    shorter_seq = true;
                                }
                            }
                            if !shorter_seq {
                                table.push(this_item);
                            }
                        }
                    }
                }
            }
            println!("count: {}", count);
            println!("table len {}", &table.len());
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
