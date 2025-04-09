use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
/// Error creating BSN
pub enum Error {
    /// The BSN has an invalid length
    InvalidLength,
    /// The BSN has an invalid character
    InvalidCharacter(usize, char),
    /// The BSN has an invalid checksum
    InvalidChecksum,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength => write!(f, "Invalid BSN length"),
            Error::InvalidCharacter(pos, char) => {
                write!(f, "Invalid BSN character at position {}: {}", pos, char)
            }
            Error::InvalidChecksum => write!(f, "Invalid BSN checksum"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    /// Try to create a new BSN. Returns `Err` if the passed string
    /// does not represent a valid BSN
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        let bsn = bsn.to_string();
        Bsn::validate(&bsn)?;
        Ok(Bsn { inner: bsn })
    }

    /// Check whether the passed string represents a valid BSN.
    //  Returns `Err` if the passed string does not represent a valid BSN
    pub fn validate(bsn: &str) -> Result<(), Error> {
        let len = bsn.len();
        if len != 9 {
            return Err(Error::InvalidLength);
        }
        let mut total = 0;
        for (i, c) in bsn.chars().enumerate() {
            if !c.is_digit(10) {
                return Err(Error::InvalidCharacter(i, c));
            }
            if i == len - 1 {
                continue;
            }
            let digit = c.to_digit(10).unwrap();
            let weight = 9 - (i as u32);
            total += digit * weight;
        }
        let last = bsn.chars().last().unwrap().to_digit(10).unwrap();
        if total % 11 != last {
            return Err(Error::InvalidChecksum);
        }
        Ok(())
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A vistitor for deserializing strings into `Bns`
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Bsn::try_from_string(v).map_err(serde::de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Bsn::try_from_string(v).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(BsnVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_validation() {
        let bsns = include_str!("../assets/valid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_ok(),
                "BSN {bsn} is valid, but did not pass validation"
            )
        });

        let bsns = include_str!("../assets/invalid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_err(),
                "BSN {bsn} invalid, but passed validation"
            )
        });
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}

fn main() {}
