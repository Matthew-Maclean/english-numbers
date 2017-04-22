use words::{Words, Word};
use formatting::Formatting;

use tens::Tens;
use hundreds::Hundreds;

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