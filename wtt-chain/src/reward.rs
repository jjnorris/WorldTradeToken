
/// Number of base units that equal one WTT (18 decimals).
pub const DECIMALS: u128 = 1_000_000_000_000_000_000;

/// Floating point reward for convenience.
///
/// The formula follows a logarithmic decay:
/// reward = 50 / sqrt(block_height / 5_000_000 + 1)
pub fn reward(block_height: u64) -> f64 {
    50.0 / ((block_height as f64 / 5_000_000.0) + 1.0).sqrt()
}

/// Reward expressed in smallest units (WTT * 10^18).
pub fn reward_units(block_height: u64) -> u128 {
    let reward = reward(block_height);
    (reward * DECIMALS as f64).round() as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_start() {
        assert!((reward(0) - 50.0).abs() < 1e-8);
    }

    #[test]
    fn reward_decay() {
        let early = reward(5_000_000);
        assert!(early < 50.0);
        let later = reward(50_000_000);
        assert!(later < early);
    }

    #[test]
    fn reward_units_start() {
        assert_eq!(reward_units(0), 50 * DECIMALS);
    }
}
