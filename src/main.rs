//! A friendly CLI tool for testing the `fuzzymonth` crate.
//!
//! Provides an interactive prompt for testing month parsing with
//! colorized output and helpful messages.

use fuzzymonth::{parse_month, Month};
use std::io::{self, Write};

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

mod display {
    use super::*;

    pub(crate) fn print_welcome() {
        println!(
            "{BLUE}
🗓 Fuzzy Month Parser
Type a month name (any format) and press Enter
Press Ctrl+C or Enter an empty line to exit
{RESET}"
        );
    }

    pub(crate) fn format_month(month: Month) -> String {
        match month {
            Month::January => "January (1)",
            Month::February => "February (2)",
            Month::March => "March (3)",
            Month::April => "April (4)",
            Month::May => "May (5)",
            Month::June => "June (6)",
            Month::July => "July (7)",
            Month::August => "August (8)",
            Month::September => "September (9)",
            Month::October => "October (10)",
            Month::November => "November (11)",
            Month::December => "December (12)",
        }
        .to_string()
    }

    pub(crate) fn print_prompt() -> io::Result<()> {
        print!("{CYAN}→ {RESET}");
        io::stdout().flush()
    }

    pub(crate) fn print_error(input: &str) {
        println!("{RED}✗ Invalid input: {input}{RESET}");
    }

    pub(crate) fn print_success(month: Month) {
        println!("{GREEN}✓ {}{RESET}", format_month(month));
    }

    pub(crate) fn print_goodbye() {
        println!("👋 Goodbye!");
    }
}

fn main() -> io::Result<()> {
    display::print_welcome();

    loop {
        display::print_prompt()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            display::print_goodbye();
            break;
        }

        match parse_month(input) {
            Ok(month) => display::print_success(month),
            Err(_) => display::print_error(input),
        }
    }

    Ok(())
}
