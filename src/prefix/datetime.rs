use super::Prefix;
use crate::error::Error;
use chrono::{DateTime, FixedOffset};
use core::str::FromStr;
use serde::{Deserialize, Serialize};

pub type TimeStamp = DateTime<FixedOffset>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateTimePrefix {
    datetime: TimeStamp,
}

impl FromStr for DateTimePrefix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "1AAG" => {
                let timestamp = s[4..].parse::<TimeStamp>().unwrap();
                let datetimeprefix = DateTimePrefix {
                    datetime: timestamp,
                };
                Ok(datetimeprefix)
            }
            _ => Err(Error::DeserializeError(format!(
                "Unknown datetime prefix code: {}",
                s
            ))),
        }
    }
}

impl Prefix for DateTimePrefix {
    fn derivative(&self) -> Vec<u8> {
        self.datetime.to_string().into_bytes()
    }
    fn derivation_code(&self) -> String {
        "1AAG".to_owned()
    }
}

#[cfg(test)]
mod datetime_tests {
    use super::*;

    #[test]
    fn test_datetime() -> Result<(), Error> {
        let timestamp_str = "1AAG2020-08-22T17:50:09.988921+00:00";
        let timestamp: DateTimePrefix = timestamp_str.parse().unwrap();
        println!("{:?}", timestamp);
        Ok(())
    }
}
