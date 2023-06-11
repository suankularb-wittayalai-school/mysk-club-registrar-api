use chrono::{Datelike, Utc};

pub fn get_current_academic_year() -> u32 {
    let now = Utc::now();
    let month = now.month();
    let year = now.year();

    if month <= 4 {
        year as u32 - 1
    } else {
        year as u32
    }
}
