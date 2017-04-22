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

        let groups =
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

    pub fn build(&self, long: bool) -> Words
    {
        let places: [Words; 7] = if long
        {
            [Words::new(vec![]),
                Words::new(vec![Word::Number("thousand".to_owned())]),
                Words::new(vec![Word::Number("milliard".to_owned())]),
                Words::new(vec![Word::Number("million".to_owned())]),
                Words::new(vec![
                    Word::Number("thousand".to_owned()),
                    Word::Space,
                    Word::Number("million".to_owned())]),
                Words::new(vec![Word::Number("billion".to_owned())]),
                Words::new(vec![
                    Word::Number("thousand".to_owned()),
                    Word::Space,
                    Word::Number("billion".to_owned())])]
        }
        else
        {
            [Words::new(vec![]),
                Words::new(vec![Word::Number("thousand".to_owned())]),
                Words::new(vec![Word::Number("million".to_owned())]),
                Words::new(vec![Word::Number("billion".to_owned())]),
                Words::new(vec![Word::Number("trillion".to_owned())]),
                Words::new(vec![Word::Number("quadrillion".to_owned())]),
                Words::new(vec![Word::Number("quintillion".to_owned())])]
        };

        let built = self.1.iter()
            .enumerate()
            .filter_map(|tup|
            {
                let index = tup.0;
                let group = tup.1;

                match group.build()
                {
                    None => None,
                    Some(mut words) =>
                    {
                        if !places[index].is_empty()
                        {
                            words.add(Words::new(vec![Word::Space]));
                            words.add(places[index].clone());
                        }

                        Some(words)
                    }
                }
            })
            .collect::<Vec<_>>();

        if built.len() == 0
        {
            return Words::new(vec![
                Word::Number("zero".to_owned())
            ]);
        }

        let last = built.len() - 1;

        let mut words = Words::new(vec![]);

        for tup in built.into_iter().rev()
            .enumerate()
        {
            let index = tup.0;
            let group_words = tup.1;

            words.add(group_words);

            if index != last
            {
                words.add(Words::new(vec![
                    Word::Comma
                ]));
            }
        }

        if self.0 == Sign::Negative
        {
            let mut temp = Words::new(vec![
                Word::Number("negative".to_owned()),
                Word::Space
            ]);

            temp.add(words);

            return temp;
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

    pub fn build(&self) -> Option<Words>
    {
        match (self.0.build(), self.1.build())
        {
            (None, None) => return None,
            (Some(hun), None) => return Some(hun),
            (None, Some(ten)) =>
            {
                return Some(ten)
            },
            (Some(mut hun), Some(ten)) =>
            {
                hun.add(Words::new(vec![
                    Word::And]));
                
                hun.add(ten);

                return Some(hun)
            }
        }
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