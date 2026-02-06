mod utils;

const FILE_PATH: &str = "./data/optical_pulse_150ps.csv";
const THRESHOLD_VOLTAGE: f64 = 0.01;
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

    let freqs: Vec<f64> = edge_indices
        .windows(2)
        .map(|w| SAMPLING_FREQ / (w[1] - w[0]) as f64)
        .collect();

    let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
    let freq_mean = freqs.iter().sum::<f64>() / freqs.len() as f64;

    let variance = intervals
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / intervals.len() as f64;

    let freq_variance = freqs
        .iter()
        .map(|&x| {
            let diff = x as f64 - freq_mean;
            diff * diff
        })
        .sum::<f64>()
        / freqs.len() as f64;

    println!(
        "繰り返し周期の平均: {:.2?}ns，標準偏差: {:.2?}ns",
        mean * 1.0e9 / SAMPLING_FREQ,
        variance.sqrt() * 1.0e9 / SAMPLING_FREQ
    );
    println!(
        "繰り返し周波数の平均: {:.2?}MHz，標準偏差: {:.2?}kHz",
        freq_mean * 1.0e-6,
        freq_variance.sqrt() * 1.0e-3
    );

    println!("\n総処理時間: {:.2?}", start.elapsed());

    Ok(())
}
