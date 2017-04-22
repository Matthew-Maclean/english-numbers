use words::{Words, Word};
use formatting::Formatting;

use tens::Tens;
use hundreds::Hundreds;
use groups::{Groups, Group};

use super::convert_all_fmt;

#[test]
fn test_words()
{
    let words = Words::new(vec![
        Word::Number("one".to_owned()),
        Word::Dash,
        Word::Comma,
        Word::Number("two".to_owned()),
        Word::And,
        Word::Number("three".to_owned())
    ]);

    assert_eq!(words.build(Formatting::all()), "One-, Two and Three");
    assert_eq!(words.build(Formatting::none()), "onetwothree");
}

#[test]
fn test_tens()
{
    let nineteen = Tens::new(19);

    let twenty = Tens::new(20);
    let ninetynine = Tens::new(99);

    let all = Formatting::all();
    let none = Formatting::none();
    
    assert_eq!(nineteen.build().unwrap().build(all), "Nineteen");
    assert_eq!(nineteen.build().unwrap().build(none), "nineteen");

    assert_eq!(twenty.build().unwrap().build(all), "Twenty");
    assert_eq!(twenty.build().unwrap().build(none), "twenty");

    assert_eq!(ninetynine.build().unwrap().build(all), "Ninety-Nine");
    assert_eq!(ninetynine.build().unwrap().build(none), "ninetynine");
}

#[test]
fn test_hundreds()
{
    let one = Hundreds::new(1);
    let seven = Hundreds::new(7);

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(one.build().unwrap().build(all), "One Hundred");
    assert_eq!(one.build().unwrap().build(none), "onehundred");

    assert_eq!(seven.build().unwrap().build(all), "Seven Hundred");
    assert_eq!(seven.build().unwrap().build(none), "sevenhundred");
}

#[test]
fn test_group()
{
    let g87 = Group::new(87);
    let g834 = Group::new(834);

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(g87.build().unwrap().build(all), "Eighty-Seven");
    assert_eq!(g87.build().unwrap().build(none), "eightyseven");

    assert_eq!(g834.build().unwrap().build(all), "Eight Hundred and Thirty-Four");
    assert_eq!(g834.build().unwrap().build(none), "eighthundredthirtyfour");
}

#[test]
fn test_groups()
{
    let g1 = Groups::new(1);
    let gneg1 = Groups::new(-1);

    let g1000 = Groups::new(1000);

    let g123456789 = Groups::new(123_456_789);

    let g9223372036854775807 = Groups::new(9223372036854775807); // i64::MAX

    let all = Formatting::all();
    let none = Formatting::none();
    
    assert_eq!(g1.build(false).build(all), "One");
    assert_eq!(g1.build(false).build(none), "one");

    assert_eq!(gneg1.build(false).build(all), "Negative One");
    assert_eq!(gneg1.build(false).build(none), "negativeone");

    assert_eq!(g1000.build(false).build(all), "One Thousand");

    assert_eq!(g123456789.build(false).build(all), "One Hundred and Twenty-Three Million, Four Hundred and Fifty-Six Thousand, Seven Hundred and Eighty-Nine");
    assert_eq!(g123456789.build(false).build(none), "onehundredtwentythreemillionfourhundredfiftysixthousandsevenhundredeightynine");

    assert_eq!(g9223372036854775807.build(false).build(all), "Nine Quintillion, Two Hundred and Twenty-Three Quadrillion, Three Hundred and Seventy-Two Trillion, Thirty-Six Billion, Eight Hundred and Fifty-Four Million, Seven Hundred and Seventy-Five Thousand, Eight Hundred and Seven");
    assert_eq!(g9223372036854775807.build(false).build(none), "ninequintilliontwohundredtwentythreequadrillionthreehundredseventytwotrillionthirtysixbillioneighthundredfiftyfourmillionsevenhundredseventyfivethousandeighthundredseven");
}

#[test]
fn test_long()
{
    assert_eq!(Groups::new(1000000).build(true).build(Formatting::all()), "One Milliard");
    assert_eq!(Groups::new(1000000000).build(true).build(Formatting::all()), "One Million");
    assert_eq!(Groups::new(1000000000000).build(true).build(Formatting::all()), "One Thousand Million");
    assert_eq!(Groups::new(1000000000000000).build(true).build(Formatting::all()), "One Billion");
    assert_eq!(Groups::new(1000000000000000000).build(true).build(Formatting::all()), "One Thousand Billion");
}

#[test]
fn test_convert()
{
    assert_eq!(convert_all_fmt(0), "Zero");
    assert_eq!(convert_all_fmt(1), "One");
    assert_eq!(convert_all_fmt(-10), "Negative Ten");
    assert_eq!(convert_all_fmt(1000), "One Thousand");
    assert_eq!(convert_all_fmt(100000), "One Hundred Thousand");
    assert_eq!(convert_all_fmt(1000000), "One Million");
}