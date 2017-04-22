use words::{Words, Word};

pub struct Hundreds(u8);

impl Hundreds
{
    pub fn new(val: usize) -> Hundreds
    {
        assert!(val < 10);

        Hundreds(val as u8)
    }

    pub fn build(&self) -> Option<Words>
    {
        if self.0 == 0
        {
            return None
        }

        return Some(Words::new(vec![Word::Number(match self.0
        {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => unreachable!()
        }.to_owned()),
        Word::Space,
        Word::Number("hundred".to_owned())]))
    }
}