use std::fs::File;
use std::io::Write;

use memmap2::Mmap;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 1_000_000;
const OVERLAP_SIZE: usize = 10;

pub(crate) fn load_voltage(file_path: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    print!("ファイルをメモリマップ中...");
    std::io::stdout().flush()?;
    let file = File::open(file_path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    println!("完了");

    print!("\n行を分割中...");
    std::io::stdout().flush()?;
    let voltage: Vec<f64> = mmap
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.last() == Some(&b'\r') {
                &line[..line.len() - 1]
            } else {
                line
            }
        })
        .map(|line| {
            if let Some(pos) = line.iter().position(|&b| b == b',') {
                // 2列目の値（電圧）を返す
                fast_float::parse::<f64, &[u8]>(&line[pos + 1..])
                    .map_err(|e| format!("パース失敗: {}", e))
            } else {
                Err("1列以下しか値が存在しない行があります".into())
            }
        })
        .collect::<Result<Vec<f64>, String>>()?;
    println!("完了\nレコード長: {}", voltage.len());

    Ok(voltage)
}

pub(crate) fn detect_edge(voltage: Vec<f64>, threshold_voltage: f64) -> Vec<usize> {
    print!("\nエッジ検出中...");
    let mut edge_indices: Vec<usize> = (0..(voltage.len() + CHUNK_SIZE - 1) / CHUNK_SIZE)
        .map(|i| i * CHUNK_SIZE)
        .par_bridge()
        .map(|start_index| {
            let mut chunk_edge_indices = Vec::new();
            let end =
                (start_index + CHUNK_SIZE + OVERLAP_SIZE).min(voltage.len().saturating_sub(1));
            for index in start_index..end {
                if voltage[index] < threshold_voltage && threshold_voltage < voltage[index + 1] {
                    chunk_edge_indices.push(index);
                }
            }
            chunk_edge_indices
        })
        .collect::<Vec<Vec<usize>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<usize>>();
    edge_indices.sort();
    edge_indices.dedup();
    println!("完了");

    edge_indices
}
