//! # english-numbers
//!
//! A library for converting integers to their written-english formats.  
//! Supports both American "short" and European "long" number formats.

mod groups;
mod hundreds;
mod tens;

mod words;
mod formatting;

#[cfg(test)]
mod test;

pub use formatting::Formatting;

/// Converts a number to its written format, using "short" format.
///
/// Arguments:
/// * `val`: the `i64` to convert  
/// * `fmt`: the formatting options to use
pub fn convert(val: i64, fmt: Formatting) -> String
{
    groups::Groups::new(val)
        .build(false)
        .build(fmt)
}

/// Converts a number to its written format, using "short" format with all formatting options enabled.
///
/// # Arguments:
/// * `val` - the `i64` to convert
pub fn convert_all_fmt(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false)
        .build(Formatting::all())
}

/// Converts a number to its written format, using "short" format with no formatting options enabled.
///
/// # Arguments:
/// * `val` - the `i64` to convert
pub fn convert_no_fmt(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false)
        .build(Formatting::none())
}

/// Converts a number to its written format, using "long" format.
///
/// # Arguments:
/// * `val` - the `i64` to convert  
/// * `fmt` - the formatting options to use
pub fn convert_long(val: i64, fmt: Formatting) -> String
{
    groups::Groups::new(val)
        .build(true)
        .build(fmt)
}
