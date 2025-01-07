# fuzzymonth 📅

A robust, user-friendly Rust library for parsing month names with built-in fuzzy matching. It handles various formats, typos, and international month names.

## Features

- ✨ Fuzzy matching for typos and misspellings
- 🌍 International support (Spanish, French, German, Italian, Polish, Russian, Arabic, Chinese)
- 📝 Multiple input formats:
  - Full names ("January", "February")
  - Common abbreviations ("Jan", "Feb", "Sept")
  - Numbers ("1", "01")
  - Ordinal numbers ("1st", "2nd", "3rd")
- 🧹 Automatic cleanup of input (whitespace trimming, case-insensitive)
- 💪 Extensively tested with property-based tests and fuzzing

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
fuzzymonth = "0.1.0"
```

Basic usage:
```rust
use fuzzymonth::{parse_month, Month};

fn main() {
    // Standard formats
    assert_eq!(parse_month("January").unwrap(), Month::January);
    assert_eq!(parse_month("feb").unwrap(), Month::February);
    
    // International support
    assert_eq!(parse_month("enero").unwrap(), Month::January);    // Spanish
    assert_eq!(parse_month("février").unwrap(), Month::February); // French
    
    // Handles typos
    assert_eq!(parse_month("septemer").unwrap(), Month::September);
    assert_eq!(parse_month("agust").unwrap(), Month::August);
    
    // Various formats
    assert_eq!(parse_month("1st").unwrap(), Month::January);
    assert_eq!(parse_month("03").unwrap(), Month::March);
    
    // Even handles messy input
    assert_eq!(parse_month(" JANUARY ").unwrap(), Month::January);
    assert_eq!(parse_month("j@nuary").unwrap(), Month::January);
}
```

## Command Line Interface

`fuzzymonth` comes with a friendly interactive CLI tool for testing month parsing:

```bash
# Install from crates.io
cargo install fuzzymonth

# Or run directly from source
cargo run
```

You'll get an interactive prompt where you can test various month formats:

```bash
🗓  Fuzzy Month Parser
Type a month name (any format) and press Enter
Press Ctrl+C or Enter an empty line to exit

→ january
✓ January (1)

→ feb
✓ February (2)

→ septembr
✓ September (9)

→ enero
✓ January (1)

→ xyz
✗ Invalid input: xyz

→ [Enter]
👋 Goodbye!
```

## How It Works

The library uses a multi-step approach to parse month names:
1. Exact matching against known formats
2. Number parsing (including ordinal numbers)
3. International variant matching
4. Fuzzy matching using Levenshtein distance for typo tolerance

## Testing

The library is extensively tested with:
- Comprehensive unit tests covering all supported formats
- Property-based tests for fuzzy matching consistency
- Edge case testing for special characters and mixed inputs
- International format validation
- Extensive fuzzing to ensure robustness

## Contributing

Contributions are welcome! Here are some ways you can help:
- Add support for more languages and regional formats
- Contribute common misspellings or abbreviations from your locale
- Add new test cases for edge cases you've encountered
- Improve fuzzy matching accuracy
- Add new input formats

Please ensure tests pass and add new tests for any added functionality.

## License

[MIT License](LICENSE)

---

Built with 💝 for real-world month parsing. Because dates are hard, and users are creative.