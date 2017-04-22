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

pub fn convert_all_fmt(val: i64) -> String
{
    groups::Groups::new(val)
        .build()
        .build(Formatting::all())
}

pub fn convert_no_fmt(val: i64) -> String
{
    groups::Groups::new(val)
        .build()
        .build(Formatting::none())
}