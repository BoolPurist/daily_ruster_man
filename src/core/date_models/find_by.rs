use crate::prelude::*;
use super::units_validated::{ValidatedYear, ValidatedMonth, ValidatedDay};

#[derive(CopyGetters, Debug)]
#[getset(get_copy = "pub")]
pub struct FindByYearMonthDay {
    year: Option<ValidatedYear>,
    month: Option<ValidatedMonth>,
    day: Option<ValidatedDay>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FindByMonthInYear {
    All,
    InCurrentYear(ValidatedYear),
    MonthYear {
        month: ValidatedMonth,
        year: ValidatedYear,
    },
}

impl FindByYearMonthDay {
    pub fn new(y_opt: Option<u32>, m_opt: Option<u32>, d_opt: Option<u32>) -> AppResult<Self> {
        let mut valid_d_opt = None;
        let mut valid_m_opt = None;
        let mut valid_y_opt = None;

        if let Some(year) = y_opt {
            let valid_year: ValidatedYear = year.try_into()?;
            valid_y_opt = Some(valid_year);
        }
        if let Some(month) = m_opt {
            let valid_month: ValidatedMonth = month.try_into()?;
            valid_m_opt = Some(valid_month);
        }
        if let Some(day) = d_opt {
            let valid_day: ValidatedDay = day.try_into()?;
            valid_d_opt = Some(valid_day);
        }

        Ok(Self {
            year: valid_y_opt,
            month: valid_m_opt,
            day: valid_d_opt,
        })
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    mod find_by_year_month_day {
        use super::*;
        use test_case::test_case;
        type TestOpt = Option<u32>;

        #[test_case(Some(2000), Some(13), Some(4))]
        #[test_case(Some(2000), Some(11), Some(0))]
        #[test_case(Some(2000), Some(11), Some(32))]
        #[test_case(Some(2222222), Some(11), Some(32))]
        fn should_deny_invalid_input(y_opt: TestOpt, m_opt: TestOpt, d_opt: TestOpt) {
            let actual = FindByYearMonthDay::new(y_opt, m_opt, d_opt);
            assert!(actual.is_err(), "Does not deny invalid date");
        }
    }
}
