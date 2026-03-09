use gloo_console::__macro::JsValue;
use std::sync::OnceLock;
use web_sys::js_sys::{Array, BigInt, Intl, Object};

pub struct ScoreFormatter {
    locale: String,
    format_type: FormatType,
}

pub fn create_score_formatter(game: &String, stat: &String) -> ScoreFormatter {
    let locale = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.language())
        .unwrap_or_else(|| "en-US".to_string());

    let game_lower = game.to_lowercase();
    let stat_lower = stat.to_lowercase();

    let mut format_type = FormatType::Default;
    if game_lower == "global" && stat_lower == "exp earned" {
        format_type = FormatType::ExpLevel;
    } else if stat_lower == "ingame time" || stat_lower == "hub time" {
        format_type = FormatType::SecondsToTime;
    }

    ScoreFormatter {
        locale,
        format_type,
    }
}

impl ScoreFormatter {
    const MAX_LEVEL: u8 = 100;

    pub fn format_score(&self, score: u64) -> String {
        match self.format_type {
            FormatType::Default => self.standard_format(score),
            FormatType::ExpLevel => self.format_exp_with_level(score),
            FormatType::SecondsToTime => self.format_seconds_to_time(score),
        }
    }

    fn standard_format(&self, score: u64) -> String {
        let options = Object::new();

        let locales = Array::of1(&JsValue::from_str(&self.locale));
        let formatter = Intl::NumberFormat::new(&locales, &options);

        let format_fn = formatter.format();
        let this = JsValue::from(formatter);
        let value = JsValue::from(BigInt::from(score));

        format_fn
            .call1(&this, &value)
            .ok()
            .and_then(|jv| jv.as_string())
            .unwrap_or_else(|| score.to_string())
    }

    fn format_exp_with_level(&self, score: u64) -> String {
        let level = self.calculate_level(score);
        let formatted_exp = self.standard_format(score);

        format!("{formatted_exp} (Level {level})")
    }

    fn format_seconds_to_time(&self, total_seconds: u64) -> String {
        const SECONDS_PER_MINUTE: u64 = 60;
        const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
        const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
        const SECONDS_PER_MONTH: u64 = 30 * SECONDS_PER_DAY;
        const SECONDS_PER_YEAR: u64 = 365 * SECONDS_PER_DAY;
        const MAX_UNITS: usize = 3;

        fn unit(value: u64, singular: &str) -> String {
            if value == 1 {
                format!("{value} {singular}")
            } else {
                format!("{value} {singular}s")
            }
        }

        let years = total_seconds / SECONDS_PER_YEAR;
        let remainder_after_years = total_seconds % SECONDS_PER_YEAR;

        let months = remainder_after_years / SECONDS_PER_MONTH;
        let remainder_after_months = remainder_after_years % SECONDS_PER_MONTH;

        let days = remainder_after_months / SECONDS_PER_DAY;
        let remainder_after_days = remainder_after_months % SECONDS_PER_DAY;

        let hours = remainder_after_days / SECONDS_PER_HOUR;
        let remainder_after_hours = remainder_after_days % SECONDS_PER_HOUR;

        let minutes = remainder_after_hours / SECONDS_PER_MINUTE;
        let seconds = remainder_after_hours % SECONDS_PER_MINUTE;

        let units = [
            (years, "year"),
            (months, "month"),
            (days, "day"),
            (hours, "hour"),
            (minutes, "minute"),
            (seconds, "second"),
        ];

        let mut parts: Vec<String> = units
            .into_iter()
            .filter(|(value, _)| *value > 0)
            .take(MAX_UNITS)
            .map(|(value, name)| unit(value, name))
            .collect();

        if parts.is_empty() {
            parts.push(unit(0, "second"));
        }

        parts.join(", ")
    }

    fn level_thresholds() -> &'static [u64; Self::MAX_LEVEL as usize] {
        static THRESHOLDS: OnceLock<[u64; ScoreFormatter::MAX_LEVEL as usize]> = OnceLock::new();

        THRESHOLDS.get_or_init(|| {
            let mut levels = [0u64; ScoreFormatter::MAX_LEVEL as usize];
            let mut total_exp: u64 = 0;
            let mut required_exp: u64 = 0;

            for level in 0..ScoreFormatter::MAX_LEVEL {
                if level < 10 {
                    required_exp += 500;
                } else if level < 20 {
                    required_exp += 1_000;
                } else {
                    let increase_factor = (level as u64) / 20;
                    required_exp += 1_000 + (increase_factor * 1_000);
                }

                total_exp += required_exp;
                levels[level as usize] = total_exp;
            }

            levels
        })
    }

    fn calculate_level(&self, score: u64) -> u8 {
        let thresholds = Self::level_thresholds();

        let lvl = thresholds.partition_point(|&t| t <= score) as u8;
        lvl.min(Self::MAX_LEVEL)
    }
}

enum FormatType {
    Default,
    ExpLevel,
    SecondsToTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_level() {
        let formatter = ScoreFormatter {
            locale: "en-US".to_string(),
            format_type: FormatType::Default,
        };

        assert_eq!(formatter.calculate_level(7378679), 87);
        assert_eq!(formatter.calculate_level(0), 0);
        assert_eq!(formatter.calculate_level(300), 0);
        assert_eq!(formatter.calculate_level(500), 1);
    }
}
