use chrono::prelude::*;

pub fn format_timestamp(ts: i64) -> String
{
    Utc.timestamp_opt(ts, 0).unwrap().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn parse_timestamp(str: &str) -> Option<i64>
{
    Utc.datetime_from_str(str, "%+").map(|dt| dt.timestamp()).ok()
}