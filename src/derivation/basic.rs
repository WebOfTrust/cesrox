use super::DerivationCode;
use crate::{error::Error, keys::PublicKey, prefix::BasicPrefix};
use core::str::FromStr;
use serde::{Deserialize, Serialize};

/// Basic Derivations
///
/// Basic prefix derivation is just a public key (2.3.1)
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum Basic {
    ECDSAsecp256k1NT,
    ECDSAsecp256k1,
    Ed25519NT,
    Ed25519,
    Ed448NT,
    Ed448,
    X25519,
    X448,
}

impl Basic {
    pub fn derive(&self, public_key: PublicKey) -> BasicPrefix {
        BasicPrefix::new(*self, public_key)
    }
}

impl DerivationCode for Basic {
    fn code_len(&self) -> usize {
        match self {
            Self::Ed25519NT | Self::Ed25519 | Self::X25519 | Self::X448 => 1,
            Self::ECDSAsecp256k1NT | Self::ECDSAsecp256k1 | Self::Ed448NT | Self::Ed448 => 4,
        }
    }

    fn derivative_b64_len(&self) -> usize {
        match self {
            Self::Ed25519NT | Self::Ed25519 | Self::X25519 => 43,
            Self::X448 => 75,
            Self::ECDSAsecp256k1NT | Self::ECDSAsecp256k1 => 47,
            Self::Ed448NT | Self::Ed448 => 76,
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Ed25519NT => "B",
            Self::X25519 => "C",
            Self::Ed25519 => "D",
            Self::X448 => "L",
            Self::ECDSAsecp256k1NT => "1AAA",
            Self::ECDSAsecp256k1 => "1AAB",
            Self::Ed448NT => "1AAC",
            Self::Ed448 => "1AAD",
        }
        .into()
    }
}

impl FromStr for Basic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .get(..1)
            .ok_or_else(|| Error::DeserializeError("Empty prefix".into()))?
        {
            "B" => Ok(Self::Ed25519NT),
            "C" => Ok(Self::X25519),
            "D" => Ok(Self::Ed25519),
            "L" => Ok(Self::X448),
            "1" => match &s[1..4] {
                "AAA" => Ok(Self::ECDSAsecp256k1NT),
                "AAB" => Ok(Self::ECDSAsecp256k1),
                "AAC" => Ok(Self::Ed448NT),
                "AAD" => Ok(Self::Ed448),
                _ => Err(Error::DeserializeError("Unknown signature code".into())),
            },
            _ => Err(Error::DeserializeError("Unknown prefix code".into())),
        }
    }
}


#[cfg(test)]
mod basic_tests {
    use crate::derivation::basic::Basic;
    use crate::derivation::basic::FromStr;
    use crate::derivation::DerivationCode;
    use crate::error::Error;
    use crate::error::Error::DeserializeError;
    use crate::keys::PublicKey;

    #[test]
    fn test_code_len() {
        assert_eq!(Basic::Ed25519.code_len(), 1);
        assert_eq!(Basic::Ed25519NT.code_len(), 1);
        assert_eq!(Basic::X25519.code_len(), 1);
        assert_eq!(Basic::X448.code_len(), 1);

        assert_eq!(Basic::ECDSAsecp256k1NT.code_len(), 4);
        assert_eq!(Basic::ECDSAsecp256k1.code_len(), 4);
        assert_eq!(Basic::Ed448NT.code_len(), 4);
        assert_eq!(Basic::Ed448.code_len(), 4);
    }

    #[test]
    fn test_derivative_b64_len() {
        assert_eq!(Basic::Ed25519NT.derivative_b64_len(), 43);
        assert_eq!(Basic::Ed25519.derivative_b64_len(), 43);
        assert_eq!(Basic::X25519.derivative_b64_len(), 43);

        assert_eq!(Basic::X448.derivative_b64_len(), 75);

        assert_eq!(Basic::ECDSAsecp256k1NT.derivative_b64_len(), 47);
        assert_eq!(Basic::ECDSAsecp256k1.derivative_b64_len(), 47);

        assert_eq!(Basic::Ed448NT.derivative_b64_len(), 76);
        assert_eq!(Basic::Ed448.derivative_b64_len(), 76);
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Basic::Ed25519NT.to_str(), "B");
        assert_eq!(Basic::X25519.to_str(), "C");
        assert_eq!(Basic::Ed25519.to_str(), "D");
        assert_eq!(Basic::X448.to_str(), "L");
        assert_eq!(Basic::ECDSAsecp256k1NT.to_str(), "1AAA");
        assert_eq!(Basic::ECDSAsecp256k1.to_str(), "1AAB");
        assert_eq!(Basic::Ed448NT.to_str(), "1AAC");
        assert_eq!(Basic::Ed448.to_str(), "1AAD");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Basic::from_str(&"B").unwrap(), Basic::Ed25519NT);
        assert_eq!(Basic::from_str(&"C").unwrap(), Basic::X25519);
        assert_eq!(Basic::from_str(&"D").unwrap(), Basic::Ed25519);
        assert_eq!(Basic::from_str(&"L").unwrap(), Basic::X448);

        assert_eq!(Basic::from_str(&"1AAA").unwrap(), Basic::ECDSAsecp256k1NT);
        assert_eq!(Basic::from_str(&"1AAB").unwrap(), Basic::ECDSAsecp256k1);
        assert_eq!(Basic::from_str(&"1AAC").unwrap(), Basic::Ed448NT);
        assert_eq!(Basic::from_str(&"1AAD").unwrap(), Basic::Ed448);
    }

    #[test]
    fn test_basic() {
        use crate::derivation::basic::Basic;
        use crate::prefix::Prefix;

        let der = Basic::ECDSAsecp256k1NT.derive(PublicKey::new([0; 33].to_vec()));
        assert_eq!(der.to_str(), ["1AAA".to_string(), "A".repeat(44)].join(""));

        let der = Basic::ECDSAsecp256k1.derive(PublicKey::new([0; 33].to_vec()));
        assert_eq!(der.to_str(), ["1AAB".to_string(), "A".repeat(44)].join(""));

        let der = Basic::Ed25519NT.derive(PublicKey::new([0; 32].to_vec()));
        assert_eq!(der.to_str(), ["B".to_string(), "A".repeat(43)].join(""));

        let der = Basic::Ed25519.derive(PublicKey::new([0; 32].to_vec()));
        assert_eq!(der.to_str(), ["D".to_string(), "A".repeat(43)].join(""));

        let der = Basic::Ed448NT.derive(PublicKey::new([0; 57].to_vec()));
        assert_eq!(der.to_str(), ["1AAC".to_string(), "A".repeat(76)].join(""));

        let der = Basic::Ed448.derive(PublicKey::new([0; 57].to_vec()));
        assert_eq!(der.to_str(), ["1AAD".to_string(), "A".repeat(76)].join(""));

        let der = Basic::X25519.derive(PublicKey::new([0; 32].to_vec()));
        assert_eq!(der.to_str(), ["C".to_string(), "A".repeat(43)].join(""));

        let der = Basic::X448.derive(PublicKey::new([0; 56].to_vec()));
        assert_eq!(der.to_str(), ["L".to_string(), "A".repeat(75)].join(""));
    }

}
