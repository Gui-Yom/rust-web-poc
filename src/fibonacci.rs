const GOLDEN_RATIO: f64 = 1.6180339887;

pub fn fibo(n: u32) -> u64 {
    (GOLDEN_RATIO.powi(n as i32) / (5.0_f64).sqrt() as f64 + 0.5).trunc() as u64
}
