use chrono::{Local, DateTime, TimeDelta, Utc};

pub fn parse_deadline(deadline_str: &str) -> Result<DateTime<Utc>,&str> {
    // define some consts for period lengths
    const MONTH_LENGTH: i64 = 30;
    const YEAR_LENGTH:  i64 = 365;

    //obtain local time for calculating the offset
    let current_time = Local::now();
    if deadline_str.len() < 2 {
        return Err("Invalid time indicator");
    }

    // simple parsing - extract last char as letter, everything else as int
    let unit = deadline_str.chars().last().ok_or("Invalid unit char")?;
    let time_number_string = &deadline_str[..deadline_str.len()-1];

    // convert number_string to int
    let time_number: i64 = time_number_string
        .parse()
        .map_err(|_| "Invalid time valued provided. You should use integer value, e.g. 4d/12w etc.")?;

    if time_number < 1 {
        return Err("Time value cannot be less than 1")
    }

    let delta = match unit {
        'h' => { TimeDelta::try_hours(time_number).ok_or("Invalid hours value")? }
        'd' => { TimeDelta::try_days(time_number).ok_or("Invalid days value")? }
        'w' => { TimeDelta::try_weeks(time_number).ok_or("Invalid weeks value")? }
        'm' => { TimeDelta::try_days(time_number*MONTH_LENGTH).ok_or("Invalid months value")? }
        'y' => { TimeDelta::try_days(time_number*YEAR_LENGTH).ok_or("Invalid years value")? }
        _ => { return Err("Time unit not recognized") }
    };
    
    return Ok((current_time + delta).into());
    
}
