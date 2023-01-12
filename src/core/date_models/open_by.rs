#[derive(Debug)]
pub enum OpenByDaysInTime {
    Past(u32),
    Future(u32),
}

impl OpenByDaysInTime {
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
pub struct OpenByDayOfYear {
    year: u32,
    day_of_year: u32,
}
#[derive(new, CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct OpenByDayMonthYear {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug)]
pub enum OpenByMonthInYear {
    CurrentMonth,
    InCurrentYear(u32),
    WithYear { month: u32, year: u32 },
}
