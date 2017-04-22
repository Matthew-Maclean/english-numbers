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
}