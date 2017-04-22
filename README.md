# english-numbers

Convert your boring old `i64`'s to shiny new `String`'s!

Use this libary to spell out numbers, as you would when reading them. Choose from a variety of formatting options,
from title-case with spaces, commas, and 'and''s where they should be, to nothing but lowercase letters, and everything in between.

[crates](https://crates.io/crates/english-numbers)

## Important Functions

    fn convert(val: i64, fmt: Formatting) -> String

This is the base function, set your `val`, and then play around with the options in the `Formatting` struct to get the precise output
you desire.

    fn convert_all_fmt(val: i64) -> String

Use this function to get an output with all the bells and whistles, example:

    123456789 -> "One Hundred and Twenty-Three Million, Four Hundred and Fifty-Six Thousand, Seven Hundred and Eighty-Nine"

Much better, right?

    fn convert_no_fmt(val: i64) -> String

Use this function to get an output as bare-bones as possible, no spaces, no hyphens, no nothing! Example:

    9223372036854775807 -> "ninequintilliontwohundredtwentythreequadrillionthreehundredseventytwotrillionthirtysixbillioneighthundredfiftyfourmillionsevenhundredseventyfivethousandeighthundredseven"
    
If that isn't easy to read, I don't know what is!

## Important Structs

    struct Formatting
    {
        pub title_case: bool,
        pub spaces: bool,
        pub conjunctions: bool,
        pub commas: bool,
        pub dashes: bool,
    }
    
This struct handles all the formatting options that you can specify, feel free to mix-and-match to *your* needs!

Use the functions `Formatting::all()` and `Formatting::none()` to get the pre-build formatting values you expect.
