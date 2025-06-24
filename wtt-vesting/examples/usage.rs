use wtt_vesting::{vested_amount, DECIMALS};

fn main() {
    let start = 0u64; // genesis timestamp placeholder
    let one_year = 365 * 24 * 60 * 60;
    let vested = vested_amount(start, one_year);
    println!("Vested after one year: {} WTT", vested as f64 / DECIMALS as f64);
}
