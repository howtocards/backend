use chrono::NaiveDateTime;

pub fn now() -> NaiveDateTime {
    use chrono::DateTime;
    use chrono::Utc;

    let datetime: DateTime<Utc> = Utc::now();

    NaiveDateTime::from_timestamp(datetime.timestamp(), 0)
}
