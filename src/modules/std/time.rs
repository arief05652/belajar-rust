// Docs: https://doc.rust-lang.org/std/time/index.html

use std::time::{Duration, Instant, SystemTime};

// kalo mau pakai datetime bisa pakai `chrono` dari crates.io

/* Duration: dipakai untuk mengukur jarak waktu
 * Instant: dipakai untuk mengukur lamanya proses kode
 * SystemTime: untuk mendapatkan waktu sekarang dengan akurat
 */

pub fn time_data() {
    duration();
    instant();
    system_time();
}

fn duration() {
    let data = Duration::from_mins(5); // 300s
    println!("Duration-min: {:?}", data);
}

fn instant() {
    let start = Instant::now();
    calculate(|a, b| a * b, 12123123, 1239120312);
    println!("Instant: {:?}", start.elapsed());
}

fn calculate<F>(func: F, a: u64, b: u64) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    func(a, b)
}

fn system_time() {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!("SystemTime: {time:?}");
}
