#![allow(dead_code)]

mod groups;
mod hundreds;
mod tens;

mod words;
mod formatting;

#[cfg(test)]
mod test;

pub use formatting::Formatting;

pub fn convert(val: i64, fmt: Formatting) -> String
{
    groups::Groups::new(val)
        .build()
        .build(fmt)
}