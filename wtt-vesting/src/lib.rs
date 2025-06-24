/// Vesting schedule logic for founder tokens.
///
/// Tokens are locked for a 6-month cliff followed by a 4-year linear vest.

pub const DECIMALS: u128 = 1_000_000_000_000_000_000u128;
/// Total amount locked for the founders (2.1 million WTT in base units).
pub const TOTAL_VEST: u128 = 2_100_000u128 * DECIMALS;

const SIX_MONTHS: u64 = 6 * 30 * 24 * 60 * 60; // approx seconds
const FOUR_YEARS: u64 = 4 * 365 * 24 * 60 * 60; // approx seconds

/// Calculate the amount vested at `current` given the `start` timestamp.
/// Both values are Unix timestamps in seconds.
pub fn vested_amount(start: u64, current: u64) -> u128 {
    if current <= start + SIX_MONTHS {
        0
    } else if current >= start + FOUR_YEARS {
        TOTAL_VEST
    } else {
        let elapsed = current - start - SIX_MONTHS;
        let linear_duration = FOUR_YEARS - SIX_MONTHS;
        (TOTAL_VEST * elapsed as u128) / linear_duration as u128
    }
}

/// Amount still locked at `current`.
pub fn locked_amount(start: u64, current: u64) -> u128 {
    TOTAL_VEST.saturating_sub(vested_amount(start, current))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_vests_before_cliff() {
        let start = 0u64;
        assert_eq!(vested_amount(start, SIX_MONTHS - 1), 0);
    }

    #[test]
    fn full_vest_after_end() {
        let start = 0u64;
        assert_eq!(vested_amount(start, FOUR_YEARS + 1), TOTAL_VEST);
    }

    #[test]
    fn partial_vest_midway() {
        let start = 0u64;
        let halfway = start + (FOUR_YEARS - SIX_MONTHS) / 2 + SIX_MONTHS;
        let vested = vested_amount(start, halfway);
        assert!(vested > 0 && vested < TOTAL_VEST);
    }
}
