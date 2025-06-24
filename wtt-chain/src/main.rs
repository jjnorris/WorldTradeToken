use wtt_chain::{reward, reward_units, total_supply, total_supply_units};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let block_height: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    let r = reward(block_height);
    let supply = total_supply(block_height);
    let r_units = reward_units(block_height);
    let supply_units = total_supply_units(block_height);

    println!("Block {block_height} reward: {r} ({r_units} units)");
    println!("Total supply up to block {block_height}: {supply} ({supply_units} units)");
}
