pub struct Hundreds(u8);

impl Hundreds
{
    pub fn new(val: usize) -> Hundreds
    {
        assert!(val < 10);

        Hundreds(val as u8)
    }
}