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
    // TODO eliminate this unwraps here
    let time_number: i64 = time_number_string.parse().unwrap();

    let delta = match unit {
        'd' => { TimeDelta::try_days(time_number).unwrap() }
        'm' => { TimeDelta::try_days(time_number*MONTH_LENGTH).unwrap() }
        'y' => { TimeDelta::try_days(time_number*YEAR_LENGTH).unwrap() }
        'w' => { TimeDelta::try_weeks(time_number).unwrap() }
        _ => { return Err("Not-recognized time unit") }
    };
    
    return Ok((current_time + delta).into());
    
}
