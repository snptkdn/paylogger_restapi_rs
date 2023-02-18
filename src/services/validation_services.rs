pub fn is_valid_date(year: Option<usize>, month: Option<usize>, day: Option<usize>) -> bool {
    let is_valid_year = if let Some(year) = year {
        year > 2020
    } else {
        true
    };

    let is_valid_month = if let Some(month) = month {
        month < 13
    } else {
        true
    };

    let is_valid_day = if let Some(day) = day { day < 32 } else { true };

    is_valid_year && is_valid_month && is_valid_day
}
