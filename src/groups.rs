use words::{Word, Words};

use tens::*;
use hundreds::*;

pub struct Groups(Sign, Vec<Group>);

impl Groups
{
    pub fn new(val: i64) -> Groups
    {
        let sign = Sign::new(val);

        let groups = (0..)
            .map(|step| (val / (i64::pow(10, step * 3))) % 1000)
            .take_while(|n| *n != 0)
            .map(|n| Group::new(n as usize))
            .collect::<Vec<_>>();

        Groups(sign, groups)
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