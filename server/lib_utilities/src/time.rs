use chrono::Duration;

pub fn timestamp(days: i64) -> u32 {
    if days != 0 {
        (chrono::offset::Local::now() - Duration::days(days)).timestamp() as u32
    } else {
        chrono::offset::Local::now().timestamp() as u32
    }
}

pub fn date(days: i64) -> String {
    if days != 0 {
        (chrono::offset::Local::now() - Duration::days(days))
            .format("%Y-%m-%d")
            .to_string()
    } else {
        chrono::offset::Local::now().format("%Y-%m-%d").to_string()
    }
}
