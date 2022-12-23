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
                let timestamp = s[4..]
                    .replace('c', ":")
                    .replace('d', ".")
                    .replace('p', "+")
                    .parse::<TimeStamp>()
                    .unwrap();
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
        self.datetime
            .to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
            .replace(':', "c")
            .replace('.', "d")
            .replace('+', "p")
            .into_bytes()
    }
    fn derivation_code(&self) -> String {
        "1AAG".to_owned()
    }
    fn to_str(&self) -> String {
        let derivative = std::str::from_utf8(&self.derivative())
            .expect("Invalid UTF8 string")
            .to_owned();
        [self.derivation_code(), derivative].join("")
    }
}

#[cfg(test)]
mod datetime_tests {
    use super::*;

    #[test]
    fn test_datetime() {
        let timestamp_str = "1AAG2020-08-22T17c50c09d988921p00c00";
        let timestamp = timestamp_str.parse::<DateTimePrefix>();
        assert!(timestamp.is_ok());
        assert_eq!(timestamp.unwrap().to_str(), timestamp_str);
    }
}
