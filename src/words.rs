use formatting::Formatting;

pub struct Words(Vec<Word>);

impl Words
{
    pub fn new(val: Vec<Word>) -> Words
    {
        Words(val)
    }

    pub fn add(&mut self, mut other: Words)
    {
        self.0.append(&mut other.0);
    }

    pub fn build(&self, fmt: Formatting) -> String
    {
        self.0.iter()
            .fold(String::new(), |mut acc, item| { acc.push_str(&item.build(fmt)); acc })
    }
}

pub enum Word
{
    Number(String),
    And,
    Dash,
    Comma
}

impl Word
{
    pub fn build(&self, fmt: Formatting) -> String
    {
        match self
        {
            &Word::Number(ref n) => if fmt.title_case && n.len() > 0
            {
                let first = n.chars().nth(0).unwrap().to_string().to_uppercase();
                let last = &n[1..n.len()];

                format!("{}{}", first, last)
            }
            else
            {
                n.to_owned()
            },
            &Word::And => if fmt.conjunctions && fmt.spaces
            {
                String::from(" and ")
            }
            else if fmt.conjunctions
            {
                String::from("and")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            },
            &Word::Dash => if fmt.dashes
            {
                String::from("-")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            },
            &Word::Comma => if fmt.commas && fmt.spaces
            {
                String::from(", ")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            }
        }
    }
}