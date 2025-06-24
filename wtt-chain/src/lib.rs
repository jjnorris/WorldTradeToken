pub mod reward;
pub mod supply;

pub use reward::{reward, reward_units, DECIMALS};
pub use supply::{total_supply, total_supply_units, MAX_SUPPLY};
