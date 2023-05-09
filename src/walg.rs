use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BackupList {
    pub backups: Vec<BackupDetail>,
}

#[derive(Deserialize, Debug)]
pub struct BackupDetail {
    pub backup_name: String,
    #[serde(with = "rfc3339_nano_format")]
    pub time: DateTime<Utc>,
    pub wal_file_name: String,
    #[serde(with = "rfc3339_nano_format")]
    pub start_time: DateTime<Utc>,
    #[serde(with = "rfc3339_nano_format")]
    pub finish_time: DateTime<Utc>,
    pub date_fmt: String,
    pub hostname: String,
    pub data_dir: String,
    pub pg_version: i64,
    pub start_lsn: i64,
    pub finish_lsn: i64,
    pub is_permanent: bool,
    pub system_identifier: i64,
    pub uncompressed_size: i32,
    pub compressed_size: i32,
}

pub mod rfc3339_nano_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.%fZ";

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
