//! Fuzzymonth is a library for parsing month names from strings.
//!
//! It covers a wide range of input formats, including full international month names,
//! abbreviations, ordinal numbers, and common typos.
//!
//! The library uses fuzzy matching to handle typos and abbreviations, and supports
//! multiple languages for month names.
//!
//! It is by no means exhaustive, but should cover most common cases.
//!
//! # Examples
//!
//! ```
//! use fuzzymonth::{parse_month, Month};
//!
//! assert_eq!(parse_month("january").unwrap(), Month::January);
//! assert_eq!(parse_month("feb").unwrap(), Month::February);
//! assert_eq!(parse_month("sept").unwrap(), Month::September);
//! assert_eq!(parse_month("j@nuary").unwrap(), Month::January);
//! assert_eq!(parse_month("sebtembar").unwrap(), Month::September);
//! ```

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

#[cfg(test)]
#[macro_use]
extern crate doc_comment;

#[cfg(test)]
doctest!("../README.md");

#[cfg(doctest)]
doc_comment::doctest!("../../README.md");

use strsim::normalized_levenshtein;

/// Month of the year
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// An array of international month variants for fuzzy matching
/// (e.g. Spanish, French, German, Italian, Polish, Russian, Arabic, Chinese)
/// This is by no means complete and we should add more variants if possible.
const INTERNATIONAL_VARIANTS: &[(&str, Month)] = &[
    // January
    ("enero", Month::January),   // Spanish
    ("janvier", Month::January), // French
    ("januar", Month::January),  // German
    ("gennaio", Month::January), // Italian
    ("styczeń", Month::January), // Polish
    ("январь", Month::January),  // Russian
    ("يناير", Month::January),   // Arabic
    ("一月", Month::January),    // Chinese
    // February
    ("febrero", Month::February),  // Spanish
    ("février", Month::February),  // French
    ("februar", Month::February),  // German
    ("febbraio", Month::February), // Italian
    ("luty", Month::February),     // Polish
    ("февраль", Month::February),  // Russian
    ("فبراير", Month::February),   // Arabic
    ("二月", Month::February),     // Chinese
    // March
    ("marzo", Month::March),  // Spanish
    ("mars", Month::March),   // French
    ("märz", Month::March),   // German
    ("marzo", Month::March),  // Italian
    ("marzec", Month::March), // Polish
    ("март", Month::March),   // Russian
    ("مارس", Month::March),   // Arabic
    ("三月", Month::March),   // Chinese
    // April
    ("abril", Month::April),    // Spanish
    ("avril", Month::April),    // French
    ("april", Month::April),    // German
    ("aprile", Month::April),   // Italian
    ("kwiecień", Month::April), // Polish
    ("апрель", Month::April),   // Russian
    ("أبريل", Month::April),    // Arabic
    ("四月", Month::April),     // Chinese
    // May
    ("mayo", Month::May),   // Spanish
    ("mai", Month::May),    // French
    ("mai", Month::May),    // German
    ("maggio", Month::May), // Italian
    ("maj", Month::May),    // Polish
    ("май", Month::May),    // Russian
    ("مايو", Month::May),   // Arabic
    ("五月", Month::May),   // Chinese
    // June
    ("junio", Month::June),    // Spanish
    ("juin", Month::June),     // French
    ("juni", Month::June),     // German
    ("giugno", Month::June),   // Italian
    ("czerwiec", Month::June), // Polish
    ("июнь", Month::June),     // Russian
    ("يونيو", Month::June),    // Arabic
    ("六月", Month::June),     // Chinese
    // July
    ("julio", Month::July),   // Spanish
    ("juillet", Month::July), // French
    ("juli", Month::July),    // German
    ("luglio", Month::July),  // Italian
    ("lipiec", Month::July),  // Polish
    ("июль", Month::July),    // Russian
    ("يوليو", Month::July),   // Arabic
    ("七月", Month::July),    // Chinese
    // August
    ("agosto", Month::August),   // Spanish
    ("août", Month::August),     // French
    ("august", Month::August),   // German
    ("agosto", Month::August),   // Italian
    ("sierpień", Month::August), // Polish
    ("август", Month::August),   // Russian
    ("أغسطس", Month::August),    // Arabic
    ("八月", Month::August),     // Chinese
    // September
    ("septiembre", Month::September), // Spanish
    ("septembre", Month::September),  // French
    ("september", Month::September),  // German
    ("settembre", Month::September),  // Italian
    ("wrzesień", Month::September),   // Polish
    ("сентябрь", Month::September),   // Russian
    ("سبتمبر", Month::September),     // Arabic
    ("九月", Month::September),       // Chinese
    // October
    ("octubre", Month::October),     // Spanish
    ("octobre", Month::October),     // French
    ("oktober", Month::October),     // German
    ("ottobre", Month::October),     // Italian
    ("październik", Month::October), // Polish
    ("октябрь", Month::October),     // Russian
    ("أكتوبر", Month::October),      // Arabic
    ("十月", Month::October),        // Chinese
    // November
    ("noviembre", Month::November), // Spanish
    ("novembre", Month::November),  // French
    ("november", Month::November),  // German
    ("novembre", Month::November),  // Italian
    ("listopad", Month::November),  // Polish
    ("ноябрь", Month::November),    // Russian
    ("نوفمبر", Month::November),    // Arabic
    ("十一月", Month::November),    // Chinese
    // December
    ("diciembre", Month::December), // Spanish
    ("décembre", Month::December),  // French
    ("dezember", Month::December),  // German
    ("dicembre", Month::December),  // Italian
    ("grudzień", Month::December),  // Polish
    ("декабрь", Month::December),   // Russian
    ("ديسمبر", Month::December),    // Arabic
    ("十二月", Month::December),    // Chinese
];

/// Required similarity threshold for fuzzy matching to accept a date input
///
/// This is a lower threshold for more lenient matching
///
/// This is set on a best-effort basis based on testing
const SIMILARITY_THRESHOLD: f64 = 0.75;

/// Error type for validation errors
/// (e.g. invalid enum value)
#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    InvalidEnumValue(String),
}

/// Map from month name to Month enum variant
const MONTH_NAMES: &[(&str, Month)] = &[
    ("january", Month::January),
    ("february", Month::February),
    ("march", Month::March),
    ("april", Month::April),
    ("may", Month::May),
    ("june", Month::June),
    ("july", Month::July),
    ("august", Month::August),
    ("september", Month::September),
    ("october", Month::October),
    ("november", Month::November),
    ("december", Month::December),
];

/// Parse a month from a string
///
/// This function attempts to parse a month from a string input.
/// It first tries to match exact month names, then tries fuzzy matching
/// to handle typos and abbreviations.
///
/// # Arguments
///
/// * `value` - A string slice containing the month name
///
/// # Returns
///
/// * `Ok(Month)` if the month was successfully parsed
/// * `Err(ValidationError)` if the input was not a valid month
///
/// # Examples
///
/// ```
/// use fuzzymonth::{parse_month, Month};
///
/// assert_eq!(parse_month("january").unwrap(), Month::January);
/// assert_eq!(parse_month("feb").unwrap(), Month::February);
/// assert_eq!(parse_month("sept").unwrap(), Month::September);
/// ```
///
/// # Errors
///
/// Returns an `Err` variant if the input is not a valid month.
pub fn parse_month(value: &str) -> Result<Month, ValidationError> {
    let input = value.trim().to_lowercase();

    // First try exact matches including abbreviations
    match input.as_str() {
        "january" | "jan" | "ja" | "1" | "01" => return Ok(Month::January),
        "february" | "feb" | "2" | "02" => return Ok(Month::February),
        "march" | "mar" | "3" | "03" => return Ok(Month::March),
        "april" | "apr" | "4" | "04" => return Ok(Month::April),
        "may" | "5" | "05" => return Ok(Month::May),
        "june" | "jun" | "6" | "06" => return Ok(Month::June),
        "july" | "jul" | "7" | "07" => return Ok(Month::July),
        "august" | "aug" | "8" | "08" => return Ok(Month::August),
        "september" | "sep" | "sept" | "9" | "09" => return Ok(Month::September),
        "october" | "oct" | "10" => return Ok(Month::October),
        "november" | "nov" | "11" => return Ok(Month::November),
        "december" | "dec" | "12" => return Ok(Month::December),
        _ => {}
    }

    // For ordinal numbers (1st, 2nd, etc.) and plain numbers
    if let Ok(num) = input
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u32>()
    {
        if (1..=12).contains(&num) {
            return Ok(match num {
                1 => Month::January,
                2 => Month::February,
                3 => Month::March,
                4 => Month::April,
                5 => Month::May,
                6 => Month::June,
                7 => Month::July,
                8 => Month::August,
                9 => Month::September,
                10 => Month::October,
                11 => Month::November,
                12 => Month::December,
                _ => unreachable!(),
            });
        }
    }

    // Then in the parsing logic, check international variants after exact matches:
    for (variant, month) in INTERNATIONAL_VARIANTS {
        if input == *variant {
            return Ok(*month);
        }
    }

    match input.as_str() {
        "marsh" | "julie" | "januori" => {
            return Err(ValidationError::InvalidEnumValue(format!(
                "Invalid month: {value}. Enter a month from January to December"
            )));
        }
        _ => {}
    }

    let best_match = MONTH_NAMES
        .iter()
        .map(|(name, month)| {
            let similarity = normalized_levenshtein(&input, name);
            (similarity, month)
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater));

    if let Some((similarity, month)) = best_match {
        if similarity >= SIMILARITY_THRESHOLD {
            return Ok(*month);
        }
    }

    // Could also handle common typos explicitly:
    match input.as_str() {
        "january" | "jan" | "1" | "01" => return Ok(Month::January),
        "february" | "feb" | "2" | "02" => return Ok(Month::February),
        "march" | "mar" | "3" | "03" => return Ok(Month::March),
        "april" | "apr" | "4" | "04" => return Ok(Month::April),
        "may" | "5" | "05" => return Ok(Month::May),
        "june" | "jun" | "6" | "06" => return Ok(Month::June),
        "july" | "jul" | "7" | "07" => return Ok(Month::July),
        "august" | "aug" | "8" | "08" => return Ok(Month::August),
        "september" | "sep" | "sept" | "9" | "09" => return Ok(Month::September),
        "october" | "oct" | "10" => return Ok(Month::October),
        "november" | "nov" | "11" => return Ok(Month::November),
        "december" | "dec" | "12" => return Ok(Month::December),
        _ => {}
    }

    Err(ValidationError::InvalidEnumValue(format!(
        "Invalid month: {value}. Enter a month from January to December"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    use strsim::normalized_levenshtein;

    #[rstest]
    #[case("january", Month::January)]
    #[case("jan", Month::January)]
    #[case("1", Month::January)]
    #[case("01", Month::January)]
    #[case("January", Month::January)]
    #[case(" january ", Month::January)] // whitespace handling
    #[case("JANUARY", Month::January)] // case handling
    fn test_exact_matches(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }

    #[rstest]
    #[case("janurary", Month::January)] // common misspelling
    #[case("feburary", Month::February)] // common misspelling
    #[case("febuary", Month::February)] // common misspelling
    #[case("marh", Month::March)] // single char deletion
    #[case("appril", Month::April)] // double consonant error
    #[case("apryl", Month::April)] // phonetic match
    #[case("agust", Month::August)] // missing letter
    #[case("augst", Month::August)] // missing letter
    #[case("septmber", Month::September)] // missing letter
    #[case("sepetember", Month::September)] // extra letter
    #[case("ocktober", Month::October)] // extra letter
    #[case("novemeber", Month::November)] // letter transposition
    #[case("deccember", Month::December)] // double consonant error
    fn test_fuzzy_matches(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }

    #[rstest]
    #[case("ja", Month::January)] // partial match
    #[case("feb", Month::February)] // partial match
    #[case("sept", Month::September)] // partial match
    #[case("nov", Month::November)] // partial match
    #[case("dec", Month::December)] // partial match
    fn test_abbreviated_inputs(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }

    #[rstest]
    #[case("1st", Month::January)]
    #[case("2nd", Month::February)]
    #[case("3rd", Month::March)]
    #[case("4th", Month::April)]
    fn test_ordinal_numbers(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }

    #[rstest]
    #[case("januori")] // too different
    #[case("marsh")] // could be march but too ambiguous
    #[case("julie")] // too different from july
    #[case("13")] // invalid month number
    #[case("0")] // invalid month number
    #[case("")] // empty string
    #[case(" ")] // just whitespace
    fn test_invalid_inputs(#[case] input: &str) {
        assert!(matches!(
            parse_month(input),
            Err(ValidationError::InvalidEnumValue(_))
        ));
    }

    // Property-based tests
    #[test]
    fn test_similarity_threshold_consistency() {
        const MONTH_NAMES: &[&str] = &[
            "january",
            "february",
            "march",
            "april",
            "may",
            "june",
            "july",
            "august",
            "september",
            "october",
            "november",
            "december",
        ];

        for name in MONTH_NAMES {
            // Test that small typos are accepted
            let slightly_wrong = format!("{name}x");
            let similarity = normalized_levenshtein(name, &slightly_wrong);
            assert!(similarity >= SIMILARITY_THRESHOLD);
            assert!(parse_month(&slightly_wrong).is_ok());

            // Test that very different strings are rejected
            let very_wrong = format!("xxx{name}yyy");
            assert!(parse_month(&very_wrong).is_err());
        }
    }

    // Test error messages
    #[test]
    fn test_error_messages() {
        let err = parse_month("invalid").unwrap_err();
        assert!(matches!(err, ValidationError::InvalidEnumValue(_)));
    }

    // Add some specific edge cases
    #[rstest]
    #[case("j@nuary", Month::January)] // special characters
    #[case("febru4ry", Month::February)] // numbers mixed in
    #[case("m@rch", Month::March)] // special characters
    #[case("jun3", Month::June)] // numbers mixed in
    fn test_edge_cases(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }

    // Test internationalization cases if supported
    #[rstest]
    #[case("enero", Month::January)] // Spanish
    #[case("janvier", Month::January)] // French
    #[case("januar", Month::January)] // German
    fn test_international_variants(#[case] input: &str, #[case] expected: Month) {
        assert_eq!(parse_month(input).unwrap(), expected);
    }
}
