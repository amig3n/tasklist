use chrono::{Local, DateTime, TimeDelta, Utc};
use std::fmt;

#[derive(Debug)]
pub enum DeadlineParseError {
    ValueTooShort,
    InvalidFormat,
    InvalidValue,
    InvalidUnit,
    GeneralError(String),
}

impl fmt::Display for DeadlineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeadlineParseError::ValueTooShort => write!(f, "Deadline string too short"),
            DeadlineParseError::InvalidFormat => write!(f, "Invalid deadline format"),
            DeadlineParseError::InvalidValue => write!(f, "Invalid deadline value"),
            DeadlineParseError::InvalidUnit => write!(f, "Invalid deadline unit"),
            DeadlineParseError::GeneralError(msg) => write!(f, "Deadline parser general error: {}", msg),
        }
    }
}

impl std::error::Error for DeadlineParseError {}

// NOTE: TEMPORARY implementation for error migration to enums
// FIXME: to be removed later
impl From<DeadlineParseError> for String {
    fn from(err: DeadlineParseError) -> Self {
        err.to_string()
    }
}

pub fn parse_deadline(deadline_str: &str) -> Result<DateTime<Utc>, DeadlineParseError> {
    // define some consts for period lengths
    const MONTH_LENGTH: i64 = 30;
    const YEAR_LENGTH:  i64 = 365;

    //obtain local time for calculating the offset
    let current_time = Local::now();
    if deadline_str.len() < 2 {
        return Err(DeadlineParseError::ValueTooShort);
    }

    // simple parsing - extract last char as letter, everything else as int
    let unit = deadline_str.chars().last().ok_or(DeadlineParseError::InvalidValue)?;
    let time_number_string = &deadline_str[..deadline_str.len()-1];

    // convert number_string to int
    let time_number: i64 = time_number_string
        .parse()
        .map_err(|_| DeadlineParseError::InvalidValue)?;

    if time_number < 1 {
        return Err(DeadlineParseError::InvalidValue);
    }

    let delta = match unit {
        'h' => { TimeDelta::try_hours(time_number).ok_or(DeadlineParseError::InvalidValue)? }
        'd' => { TimeDelta::try_days(time_number).ok_or(DeadlineParseError::InvalidValue)? }
        'w' => { TimeDelta::try_weeks(time_number).ok_or(DeadlineParseError::InvalidValue)? }
        'm' => { TimeDelta::try_days(time_number*MONTH_LENGTH).ok_or(DeadlineParseError::InvalidValue)? }
        'y' => { TimeDelta::try_days(time_number*YEAR_LENGTH).ok_or(DeadlineParseError::InvalidValue)? }
        _ => { return Err(DeadlineParseError::InvalidUnit); }
    };
    
    return Ok((current_time + delta).into());
    
}
