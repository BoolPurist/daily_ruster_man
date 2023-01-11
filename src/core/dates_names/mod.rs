pub mod daily_names;
pub mod monthly_name;
#[cfg(test)]
pub mod test_daily_names;

pub use daily_names::DailyName;
pub use monthly_name::MonthlyName;

const DIGIT_SEP: &str = "_";

pub trait HasYear {
    fn year(&self) -> u32;

    fn is_in_year(&self, other_year: u32) -> bool {
        self.year() == other_year
    }
}
pub trait HasMonth {
    fn month(&self) -> u32;

    fn is_in_month(&self, other_month: u32) -> bool {
        self.month() == other_month
    }
}

pub trait ToDateTuple {
    fn to_date_tuple(&self) -> String;
}
