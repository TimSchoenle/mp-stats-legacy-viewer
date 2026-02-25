use gloo_console::__macro::JsValue;
use std::sync::OnceLock;
use web_sys::js_sys::{Array, BigInt, Intl, Object};

pub struct ScoreFormatter {
    locale: String,
    format_type: FormatType,
}

pub fn create_score_formatter(game: &String,
                              stat: &String,
) -> ScoreFormatter {
    let locale = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.language())
        .unwrap_or_else(|| "en-US".to_string());

    let game_lower = game.to_lowercase();
    let stat_lower = stat.to_lowercase();

    let mut format_type = FormatType::Default;
    if game_lower == "global" && stat_lower == "exp earned" {
        format_type = FormatType::ExpLevel;
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
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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