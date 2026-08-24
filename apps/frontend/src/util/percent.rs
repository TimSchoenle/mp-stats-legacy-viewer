//! The one percentage on the site: how a row's score compares with the leader's.

/// Renders `value` as a percentage, rounded to five decimals with trailing zeros trimmed, so
/// `100.0` comes out as `100%` and `33.33333` keeps every digit.
#[must_use]
pub fn format_percent(value: f64) -> String {
    const MAX_DECIMALS: usize = 5;

    let formatted = format!("{value:.MAX_DECIMALS$}");

    if formatted.contains('.') {
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        format!("{trimmed}%")
    } else {
        format!("{formatted}%")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_trailing_zeros() {
        assert_eq!(format_percent(100.0), "100%");
        assert_eq!(format_percent(50.5), "50.5%");
        assert_eq!(format_percent(0.0), "0%");
    }

    #[test]
    fn keeps_needed_decimals() {
        assert_eq!(format_percent(33.333_33), "33.33333%");
        assert_eq!(format_percent(12.5), "12.5%");
    }

    #[test]
    fn rounds_to_max_decimals() {
        assert_eq!(format_percent(99.999_999), "100%");
    }
}
