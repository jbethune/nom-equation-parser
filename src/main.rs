use std::{env::args, process::exit};

use equation_parser::calculation::Calculation;

const ANSI_RED_BOLD_ON: &str = "\u{1B}[38;5;9m\u{1B}[1m";
const ANSI_OFF: &str = "\u{1B}[0m";

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() != 2 {
        eprintln!("Usage: {} 'math-expression'", argv[0]);
        exit(1);
    }

    let calculation: Calculation = match argv[1].parse() {
        Ok(calc) => calc,
        Err(e) => {
            eprintln_failed_expr(&argv[1], &e.failed_tail);
            exit(2);
        }
    };

    let result = calculation.evaluate();
    println!("{} = {}", result, argv[1]);
}

fn eprintln_failed_expr(full_expr: &str, tail: &str) {
    if !tail.is_empty() {
        eprintln!("Failed to parse full expression below:\n{}", full_expr);
        let good_len = full_expr.len() - tail.len();
        eprintln!(
            "{}{}{} cannot parse{}",
            " ".repeat(good_len),
            ANSI_RED_BOLD_ON,
            "^".repeat(tail.len()),
            ANSI_OFF
        );
    } else {
        eprintln!("The expression below is incomplete:\n{}", full_expr);
    }
}
