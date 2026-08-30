//! Relative-date resolution over chrono, tuned for honesty: vague
//! expressions ("soon", "next week") resolve to `None`, never to an
//! invented concrete day. Concrete expressions (weekday names, "the 30th",
//! "tomorrow", ISO strings) resolve to a real date.

use chrono::{Duration, NaiveDate};

#[derive(Debug, Clone, PartialEq)]
pub struct DateResolution {
    pub date: Option<NaiveDate>,
    /// true when the text contained a *vague* temporal reference
    pub vague_marker_found: bool,
}

/// Resolve the first date-like expression in `text` relative to `today`.
pub fn parse_date_expression(text: &str, today: NaiveDate) -> DateResolution {
    let lower = text.to_lowercase();

    // explicit ISO date
    for tok in lower.split(|c: char| !c.is_ascii_alphanumeric() && c != '-') {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(tok, "%Y-%m-%d") {
            return DateResolution { date: Some(d), vague_marker_found: false };
        }
    }

    if contains_word(&lower, &["today", "tonight"]) {
        return res(Some(today), false);
    }
    if contains_word(&lower, &["tomorrow"]) {
        return res(Some(today + Duration::days(1)), false);
    }

    // "in N days/weeks"
    if let Some(n) = capture_in_n(&lower, "day") {
        return res(Some(today + Duration::days(n)), false);
    }
    if let Some(n) = capture_in_n(&lower, "week") {
        return res(None, true).also_date_if_none(today + Duration::days(7 * n));
    }

    // "the Nth" / "on the Nth" — day of current or next month
    if let Some(day) = capture_day_of_month(&lower) {
        let this_month = today
            .with_day(day)
            .filter(|d| *d >= today);
        let resolved = this_month.or_else(|| next_month_with_day(today, day));
        return res(resolved, false);
    }

    // weekday names: nearest upcoming occurrence (strictly after today),
    // with "next <weekday>" meaning the one after that
    const WEEKDAYS: [(&str, chrono::Weekday); 7] = [
        ("monday", chrono::Weekday::Mon),
        ("tuesday", chrono::Weekday::Tue),
        ("wednesday", chrono::Weekday::Wed),
        ("thursday", chrono::Weekday::Thu),
        ("friday", chrono::Weekday::Fri),
        ("saturday", chrono::Weekday::Sat),
        ("sunday", chrono::Weekday::Sun),
    ];
    for (name, target) in WEEKDAYS {
        if contains_word(&lower, &[name]) {
            let days_ahead = days_until(target, today);
            let base = today + Duration::days(days_ahead);
            let d = if has_next_qualifier(&lower, name) {
                base + Duration::days(7)
            } else {
                base
            };
            return res(Some(d), false);
        }
    }

    if contains_word(&lower, &["this weekend", "weekend"]) {
        // Saturday of the current week
        let sat = days_until(chrono::Weekday::Sat, today);
        return res(Some(today + Duration::days(sat)), false);
    }

    // deliberately vague: no concrete resolution
    const VAGUE: [&str; 9] = [
        "soon", "next week", "this week", "next month", "this month", "sometime",
        "eventually", "asap", "when i get a chance",
    ];
    let vague = contains_word(&lower, &VAGUE);
    res(None, vague)
}

fn res(date: Option<NaiveDate>, vague: bool) -> DateResolution {
    DateResolution { date, vague_marker_found: vague }
}

trait AlsoDate {
    fn also_date_if_none(self, d: NaiveDate) -> DateResolution;
}
impl AlsoDate for DateResolution {
    fn also_date_if_none(mut self, d: NaiveDate) -> DateResolution {
        if self.date.is_none() && !self.vague_marker_found {
            self.date = Some(d);
        } else if self.date.is_none() {
            self.vague_marker_found = true;
        }
        self
    }
}

fn contains_word(hay: &str, words: &[&str]) -> bool {
    words.iter().any(|w| {
        hay.split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
            .any(|tok| tok == *w || (w.contains(' ') && hay.contains(w)))
    })
}

fn has_next_qualifier(hay: &str, weekday: &str) -> bool {
    // "next friday" but not "friday"
    if let Some(pos) = hay.find(weekday) {
        let before = &hay[..pos];
        let last_word = before
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|t| !t.is_empty())
            .last()
            .unwrap_or("");
        return last_word == "next";
    }
    false
}

fn capture_in_n(text: &str, unit: &str) -> Option<i64> {
    let pat = format!(r"in\s+(\d{{1,3}})\s+{unit}s?\b");
    let re_text = pat;
    simple_number_after_in(text, unit).or_else(|| {
        regex_lite_fallback(&re_text, text)
    })
}

/// tiny helper avoiding an external regex dependency
fn regex_lite_fallback(_pat: &str, _text: &str) -> Option<i64> {
    None
}

fn simple_number_after_in(text: &str, unit: &str) -> Option<i64> {
    let toks: Vec<&str> = text.split(|c: char| !c.is_ascii_alphanumeric()).collect();
    for i in 0..toks.len().saturating_sub(2) {
        if toks[i] == "in" {
            if let Ok(n) = toks[i + 1].parse::<i64>() {
                let u = toks[i + 2].trim_end_matches('s');
                if u == unit {
                    return Some(n);
                }
            }
        }
    }
    None
}

fn capture_day_of_month(text: &str) -> Option<u32> {
    let toks: Vec<&str> = text.split(|c: char| !c.is_ascii_alphanumeric()).collect();
    for i in 0..toks.len() {
        if toks[i] == "the" {
            if i + 1 < toks.len() {
                let raw = toks[i + 1];
                let num: String =
                    raw.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !num.is_empty() {
                    if let Ok(d) = num.parse::<u32>() {
                        if (1..=31).contains(&d) {
                            return Some(d);
                        }
                    }
                }
                // ordinal suffixes: 30th, 1st, 2nd, 3rd handled by take_while digits
            }
        }
    }
    None
}

fn days_until(target: chrono::Weekday, from: NaiveDate) -> i64 {
    let cur = from.weekday().num_days_from_monday() as i64;
    let tgt = target.num_days_from_monday() as i64;
    let mut delta = (tgt - cur).rem_euclid(7);
    if delta == 0 {
        delta = 7; // strictly future occurrence
    }
    delta
}

fn next_month_with_day(today: NaiveDate, day: u32) -> Option<NaiveDate> {
    let (y, m) = (today.year(), today.month());
    let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    NaiveDate::from_ymd_opt(ny, nm, day)
}

use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;
    fn wed() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 8, 26).unwrap() // Wednesday
    }

    #[test]
    fn iso_direct() {
        assert_eq!(
            parse_date_expression("pay by 2026-09-01 ok", wed()).date,
            Some(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap())
        );
    }

    #[test]
    fn weekdays() {
        assert_eq!(parse_date_expression("this friday", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 28).unwrap()));
        assert_eq!(parse_date_expression("on sunday", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 30).unwrap()));
        assert_eq!(parse_date_expression("next friday", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 9, 4).unwrap()));
    }

    #[test]
    fn day_of_month() {
        assert_eq!(parse_date_expression("by the 30th", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 30).unwrap()));
        assert_eq!(parse_date_expression("before the 5th", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 9, 5).unwrap()));
    }

    #[test]
    fn relative_words() {
        assert_eq!(parse_date_expression("due tomorrow morning", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 27).unwrap()));
        assert_eq!(parse_date_expression("in 3 days", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 29).unwrap()));
        assert_eq!(parse_date_expression("this weekend", wed()).date,
                   Some(NaiveDate::from_ymd_opt(2026, 8, 29).unwrap()));
    }

    #[test]
    fn vague_stays_null() {
        for s in ["soon", "next week", "sometime this week", "asap"] {
            let r = parse_date_expression(s, wed());
            assert_eq!(r.date, None, "{s}");
            assert!(r.vague_marker_found, "{s}");
        }
    }

    #[test]
    fn none_present() {
        let r = parse_date_expression("no time info here at all", wed());
        assert_eq!(r.date, None);
        assert!(!r.vague_marker_found);
    }
}
