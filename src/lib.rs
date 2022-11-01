//! Parse mathematical expressions and exaluate them.
//!
//! Usage:
//! ```rust
//! use equation_parser::parse_equation;
//!
//! let (tail, equation) = parse_equation("2 + 3 * 4").expect("doc-test");
//! assert_eq!(equation.evaluate(), 14);
//! ```
//! or alternatively:
//! ```rust
//! use equation_parser::calculation::Calculation;
//! let formula = "2 + 3 * 4";
//! let equation: Calculation = formula.parse().expect("doc-test");
//! assert_eq!(equation.evaluate(), 14);
//! ````

pub mod calculation;
pub mod error;
pub mod parse;

pub use parse::parse_equation;
