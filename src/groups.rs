use words::{Word, Words};

use tens::*;
use hundreds::*;

pub struct Groups(Sign, Vec<Group>);

impl Groups
{
    pub fn new(val: i64) -> Groups
    {
        let sign = Sign::new(val);

        let val = i64::abs(val);

        let groups = if val < i64::pow(10, 6 * 3) // if it's greater than 10 ^ 18, the numerical approach won't work
        {
            (0..)
                .map(|step| (val / (i64::pow(10, step * 3))) % 1000)
                .take_while(|n| *n != 0)
                .map(|n| Group::new(n as usize))
                .collect::<Vec<_>>()
        }
        else
        {
            let mut val_string = val.to_string();

            while val_string.len() % 3 != 0
            {
                val_string = format!("0{}", val_string);
            }

            let mut parsed_chunks = val_string.chars().collect::<Vec<_>>().chunks(3)
                .map(|chunk| chunk.into_iter()
                    .map(|c| *c)
                    .collect::<String>())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            
            parsed_chunks.reverse();

            parsed_chunks.into_iter()
                .map(|n| Group::new(n))
                .collect::<Vec<_>>()
        };

        Groups(sign, groups)
    }

    pub fn build(&self) -> Words
    {
        let mut words = Words::new(vec![]);

        // i64::max is nine quintillion, so this should cover all the possible values
        let places = vec!["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

        for i in 0..self.1.len()
        {
            let mut group = self.1[i].build();

            if places[i] != ""
            {
                group.add(Words::new(vec![
                    Word::Space,
                    Word::Number(places[i].to_owned())]))
            }

            if i != 0
            {
                group.add(Words::new(vec![Word::Comma]));
            }

            group.add(words);

            words = group;
        }

        if self.0 == Sign::Negative
        {
            let mut neg = Words::new(vec![
                Word::Number("negative".to_owned()),
                Word::Space
            ]);

            neg.add(words);

            return neg;
        }

        words
    }
}

pub struct Group(Hundreds, Tens);

impl Group
{
    pub fn new(val: usize) -> Group
    {
        assert!(val < 1000);

        let hundreds = Hundreds::new(val / 100);
        let tens = Tens::new(val % 100);

        Group(hundreds, tens)
    }

    pub fn build(&self) -> Words
    {
        if self.0.is_zero()
        {
            return self.1.build()
        }

        let mut words = self.0.build();

        words.add(Words::new(vec![Word::And]));

        words.add(self.1.build());

        words
    }
}

#[derive(PartialEq)]
enum Sign
{
    Positive,
    Negative
}

impl Sign
{
    pub fn new(val: i64) -> Sign
    {
        match val >= 0
        {
            true => Sign::Positive,
            false => Sign::Negative
        }
    }
}