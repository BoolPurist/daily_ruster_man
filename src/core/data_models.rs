#[derive(Debug)]
pub enum PastFuture {
    Past(u32),
    Future(u32),
}
impl PastFuture {
    pub fn new(past_or_future: i32) -> Self {
        if past_or_future < 0 {
            Self::Past(past_or_future.unsigned_abs())
        } else {
            Self::Future(past_or_future.unsigned_abs())
        }
    }
}
#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct DayOfYear {
    year: u32,
    day_of_year: u32,
}
#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct DayMonthYear {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct FilterParamsYmD {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}
