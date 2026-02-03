mod utils;

const FILE_PATH: &str = "./data/150p.csv";
const THRESHOLD_VOLTAGE: f64 = 0.1;
const SAMPLING_FREQ: f64 = 1.25e11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let voltage = utils::load_voltage(FILE_PATH)?;
    let edge_indices = utils::detect_edge(voltage, THRESHOLD_VOLTAGE);

    println!("\n=====計算結果=====\nエッジ数: {}", edge_indices.len());
    let intervals: Vec<f64> = edge_indices
        .windows(2)
        .map(|w| (w[1] - w[0]) as f64)
        .collect();

    let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;

    let variance = intervals
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / intervals.len() as f64;

    println!(
        "繰り返し周期の平均: {:.2?}ns，標準偏差: {:.2?}ns",
        mean * 1.0e9 / SAMPLING_FREQ,
        variance.sqrt() * 1.0e9 / SAMPLING_FREQ
    );

    println!("\n総処理時間: {:.2?}", start.elapsed());

    Ok(())
}
