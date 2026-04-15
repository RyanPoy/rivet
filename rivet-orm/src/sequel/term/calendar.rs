use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, thiserror::Error)]
#[error("ValueError: {msg}, but got {value}")]
pub struct ValueError {
    msg: String,
    value: String,
}
impl ValueError {
    pub fn new<T: Into<String>, V: std::fmt::Display>(msg: T, value: V) -> Self {
        Self {
            msg: msg.into(),
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    microsecond: u32,
}
impl Time {
    pub fn new(hour: u8, minute: u8, second: u8, microsecond: u32) -> Result<Self, ValueError> {
        check_hour_minute_second_microsecond(hour, minute, second, microsecond)?;
        Ok(Self {
            hour,
            minute,
            second,
            microsecond,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:06}",
            self.hour, self.minute, self.second, self.microsecond
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}
impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, ValueError> {
        check_year_month_day(year, month, day)?;
        Ok(Self { year, month, day })
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub fn from(d: Date, t: Time) -> Result<Self, ValueError> {
        Self::new(d.year, d.month, d.day, t.hour, t.minute, t.second, t.microsecond)
    }

    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<Self, ValueError> {
        Ok(Self {
            date: Date::new(year, month, day)?,
            time: Time::new(hour, minute, second, microsecond)?,
        })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date.to_string(), self.time.to_string())
    }
}

fn check_year_month_day(year: i32, month: u8, day: u8) -> Result<(), ValueError> {
    if !(1..=12).contains(&month) {
        return Err(ValueError::new("month must be in 1..=12", month));
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) => 29,
        2 => 28,
        _ => unreachable!(), // 月份已检查在1..12
    };
    if day > max_day || day == 0 {
        return Err(ValueError::new(format!("day must be in 1..{}", max_day), day));
    }
    Ok(())
}

fn check_hour_minute_second_microsecond(hour: u8, minute: u8, second: u8, microsecond: u32) -> Result<(), ValueError> {
    if hour > 23 {
        return Err(ValueError::new("hour must be in 0..=23", hour));
    }
    if minute > 59 {
        return Err(ValueError::new("minute must be in 0..=59", minute));
    }
    if second > 59 {
        return Err(ValueError::new("second must be in 0..=59", second));
    }
    if microsecond > 999_999 {
        return Err(ValueError::new("microsecond must be in 0..=999999", microsecond));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_valid() {
        assert!(Date::new(2024, 2, 29).is_ok()); // 闰年
        assert!(Date::new(2023, 2, 29).is_err()); // 非闰年
        assert!(Date::new(2024, 4, 31).is_err()); // 4月只有30天
        assert!(Date::new(2024, 0, 1).is_err()); // 月份0
    }

    #[test]
    fn test_time_valid() {
        assert!(Time::new(23, 59, 59, 999_999).is_ok()); // 最大值
        assert!(Time::new(0, 0, 0, 0).is_ok()); // 最小值
        assert!(Time::new(24, 0, 0, 0).is_err()); // 小时越界
        assert!(Time::new(0, 60, 0, 0).is_err()); // 分钟越界
        assert!(Time::new(0, 0, 60, 0).is_err()); // 秒越界
        assert!(Time::new(0, 0, 0, 1_000_000).is_err()); // 微秒越界
    }

    #[test]
    fn test_datetime_valid() {
        assert!(DateTime::new(2024, 2, 29, 23, 59, 59, 999_999).is_ok()); // 有效组合
        assert!(DateTime::new(2024, 2, 30, 12, 0, 0, 0).is_err()); // 日期无效
        assert!(DateTime::new(2024, 2, 28, 24, 0, 0, 0).is_err()); // 时间无效
    }
}
