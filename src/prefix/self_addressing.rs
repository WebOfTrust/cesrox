use core::{fmt, str::FromStr};

use base64::decode_config;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::derivation::{DerivationCode, self_addressing::SelfAddressing};
use crate::error::Error;
use std::string::String;
use super::Prefix;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SelfAddressingPrefix {
    pub derivation: SelfAddressing,
    pub digest: Vec<u8>,
}

impl SelfAddressingPrefix {
    pub fn new(code: SelfAddressing, digest: Vec<u8>) -> Self {
        Self {
            derivation: code,
            digest,
        }
    }

    pub fn verify_binding(&self, sed: &[u8]) -> bool {
        self.derivation.digest(sed) == self.digest
    }
}

impl FromStr for SelfAddressingPrefix {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = SelfAddressing::from_str(s)?;
        let c_len = code.code_len();
        let p_len = code.prefix_b64_len();

        let mut tmp = String::from(s);
        let pad = String::from_utf8(vec![b'A'; c_len]).unwrap();

        tmp.replace_range(0..c_len, &pad);

        if s.len() == code.prefix_b64_len() {
            Ok(Self::new(
                code,
                decode_config(&tmp, base64::URL_SAFE)?,
            ))
        } else {
            Err(Error::SemanticError(format!(
                "Incorrect Prefix Length: {}",
                s
            )))
        }
    }
}

impl Prefix for SelfAddressingPrefix {
    fn derivative(&self) -> Vec<u8> {
        self.digest.to_owned()
    }
    fn derivation_code(&self) -> String {
        self.derivation.to_str()
    }
}

impl fmt::Display for SelfAddressingPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

/// Serde compatible Serialize
impl Serialize for SelfAddressingPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for SelfAddressingPrefix {
    fn deserialize<D>(deserializer: D) -> Result<SelfAddressingPrefix, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SelfAddressingPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for SelfAddressingPrefix {
    fn default() -> Self {
        Self {
            derivation: SelfAddressing::Blake3_256,
            digest: vec![],
        }
    }
}


#[cfg(test)]
mod prefix_self_addressing_tests {
    use std::str;
    use std::str::FromStr;

    use base64::{encode_config, URL_SAFE};

    use crate::derivation::basic::Basic;
    use crate::derivation::DerivationCode;
    use crate::derivation::self_addressing::SelfAddressing;
    use crate::prefix::{Prefix, SelfAddressingPrefix};

    #[test]
    fn test_self_addressing() {
        let pre = SelfAddressingPrefix::from_str("ELC5L3iBVD77d_MYbYGGCUQgqQBju1o4x1Ud-z2sL-ux").unwrap();
        assert_eq!(pre.derivation, SelfAddressing::Blake3_256);
        assert_eq!(encode_config(pre.digest, URL_SAFE), "ALC5L3iBVD77d_MYbYGGCUQgqQBju1o4x1Ud-z2sL-ux");
    }
}