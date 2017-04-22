#[derive(Copy, Clone)]
pub struct Formatting
{
    pub title_case: bool,
    pub spaces: bool,
    pub conjunctions: bool,
    pub commas: bool,
    pub dashes: bool,
}

impl Formatting
{
    pub fn all() -> Formatting
    {
        Formatting
        {
            title_case: true,
            spaces: true,
            conjunctions: true,
            commas: true,
            dashes: true
        }
    }

    pub fn none() -> Formatting
    {
        Formatting
        {
            title_case: false,
            spaces: false,
            conjunctions: false,
            commas: false,
            dashes: false
        }
    }
}

impl Default for Formatting
{
    fn default() -> Formatting
    {
        Formatting::none()
    }
}