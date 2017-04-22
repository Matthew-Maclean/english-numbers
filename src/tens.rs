use words::{Words, Word};

pub struct Tens(u8, u8);

impl Tens
{
    pub fn new(val: usize) -> Tens
    {
        assert!(val < 100);

        let val = val as u8;

        let tens_place = val / 10;
        let ones_place = val % 10;

        Tens(tens_place, ones_place)
    }

    pub fn build(&self) -> Words
    {
        if self.0 * 10 + self.1 < 20
        {
            return Words::new(vec![Word::Number(match self.0 * 10 + self.1
            {
                0 => "zero",
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                10 => "ten",
                11 => "eleven",
                12 => "twelve",
                13 => "thirteen",
                14 => "fourteen",
                15 => "fifteen",
                16 => "sixteen",
                17 => "seventeen",
                18 => "eighteen",
                19 => "nineteen",
                _ => unreachable!()
            }.to_owned())])
        }
        else
        {
            let tens = match self.0
            {
                2 => "twenty",
                3 => "thirty",
                4 => "fourty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!()
            };

            if self.1 == 0
            {
                return Words::new(vec![Word::Number(tens.to_owned())])
            }
            else
            {
                let ones = match self.1
                {
                    0 => "zero",
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
                };

                return Words::new(vec![
                    Word::Number(tens.to_owned()),
                    Word::Dash,
                    Word::Number(ones.to_owned())
                ])
            }
        }
    }
}