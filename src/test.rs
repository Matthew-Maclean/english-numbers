use words::{Words, Word};
use formatting::Formatting;

use tens::Tens;
use hundreds::Hundreds;
use groups::{Groups, Group};

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
    let zero = Tens::new(0);
    let nineteen = Tens::new(19);

    let twenty = Tens::new(20);
    let ninetynine = Tens::new(99);

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(zero.build().build(all), "Zero");
    assert_eq!(zero.build().build(none), "zero");

    assert_eq!(nineteen.build().build(all), "Nineteen");
    assert_eq!(nineteen.build().build(none), "nineteen");

    assert_eq!(twenty.build().build(all), "Twenty");
    assert_eq!(twenty.build().build(none), "twenty");

    assert_eq!(ninetynine.build().build(all), "Ninety-Nine");
    assert_eq!(ninetynine.build().build(none), "ninetynine");
}

#[test]
fn test_hundreds()
{
    let one = Hundreds::new(1);
    let seven = Hundreds::new(7);

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(one.build().build(all), "One Hundred");
    assert_eq!(one.build().build(none), "onehundred");

    assert_eq!(seven.build().build(all), "Seven Hundred");
    assert_eq!(seven.build().build(none), "sevenhundred");
}

#[test]
fn test_group()
{
    let g87 = Group::new(87);
    let g834 = Group::new(834);

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(g87.build().build(all), "Eighty-Seven");
    assert_eq!(g87.build().build(none), "eightyseven");

    assert_eq!(g834.build().build(all), "Eight Hundred and Thirty-Four");
    assert_eq!(g834.build().build(none), "eighthundredthirtyfour");
}

#[test]
fn test_groups()
{
    let g1 = Groups::new(1);
    let gneg1 = Groups::new(-1);

    let g123456789 = Groups::new(123_456_789);

    let g9223372036854775807 = Groups::new(9223372036854775807); // i64::MAX

    let all = Formatting::all();
    let none = Formatting::none();

    assert_eq!(g1.build().build(all), "One");
    assert_eq!(g1.build().build(none), "one");

    assert_eq!(gneg1.build().build(all), "Negative One");
    assert_eq!(gneg1.build().build(none), "negativeone");

    assert_eq!(g123456789.build().build(all), "One Hundred and Twenty-Three Million, Four Hundred and Fifty-Six Thousand, Seven Hundred and Eighty-Nine");
    assert_eq!(g123456789.build().build(none), "onehundredtwentythreemillionfourhundredfiftysixthousandsevenhundredeightynine");

    assert_eq!(g9223372036854775807.build().build(all), "Nine Quintillion, Two Hundred and Twenty-Three Quadrillion, Three Hundred and Seventy-Two Trillion, Thirty-Six Billion, Eight Hundred and Fifty-Four Million, Seven Hundred and Seventy-Five Thousand, Eight Hundred and Seven");
    assert_eq!(g9223372036854775807.build().build(none), "ninequintilliontwohundredtwentythreequadrillionthreehundredseventytwotrillionthirtysixbillioneighthundredfiftyfourmillionsevenhundredseventyfivethousandeighthundredseven");
}