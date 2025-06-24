/// Utility to calculate cumulative supply given a block height.
/// This integrates the reward formula up to the provided block.
use crate::reward::reward_units;

/// Maximum possible supply in base units (42 million WTT).
pub const MAX_SUPPLY: u128 = 42_000_000u128 * crate::reward::DECIMALS;

/// Floating point approximation of total supply for convenience.
pub fn total_supply(block_height: u64) -> f64 {
    let mut supply = 0f64;
    for h in 0..=block_height {
        supply += super::reward::reward(h);
        if (supply * crate::reward::DECIMALS as f64) >= MAX_SUPPLY as f64 {
            return MAX_SUPPLY as f64 / crate::reward::DECIMALS as f64;
        }
    }
    if (supply * crate::reward::DECIMALS as f64) > MAX_SUPPLY as f64 {
        MAX_SUPPLY as f64 / crate::reward::DECIMALS as f64
    } else {
        supply
    }
}

/// Total supply in base units (WTT * 10^18).
pub fn total_supply_units(block_height: u64) -> u128 {
    let mut supply: u128 = 0;
    for h in 0..=block_height {
        supply += reward_units(h);
        if supply >= MAX_SUPPLY {
            return MAX_SUPPLY;
        }
    }
    if supply > MAX_SUPPLY { MAX_SUPPLY } else { supply }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_increases() {
        let s1 = total_supply(10);
        let s2 = total_supply(11);
        assert!(s2 > s1);
    }

    #[test]
    fn supply_units_consistency() {
        let float = total_supply(0);
        let units = total_supply_units(0);
        assert_eq!(units, (float * crate::reward::DECIMALS as f64).round() as u128);
    }

    #[test]
    fn supply_capped() {
        let high = total_supply_units(100_000_000);
        assert!(high <= MAX_SUPPLY);
    }
}
