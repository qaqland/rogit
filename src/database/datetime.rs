use anyhow::Result;
use git2::Time;
use time::{OffsetDateTime, UtcOffset};

pub fn get_time(gt: Time) -> Result<OffsetDateTime> {
    let offset = UtcOffset::from_whole_seconds(gt.offset_minutes() * 60)?;
    let time = OffsetDateTime::from_unix_timestamp(gt.seconds())?.to_offset(offset);
    Ok(time)
}
