use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

const MIN_LEN: usize = 12;
const MAX_LEN: usize = 256;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PlaintextPasswordError {
    #[error("password must be at least {0} characters")]
    TooShort(usize),
    #[error("password must be at most {0} characters")]
    TooLong(usize),
}

/// Plaintext password. Never persisted; used only for hashing/verification.
///
/// Intentionally does NOT implement `Debug`, `Display`, `Serialize`, or
/// `Deserialize`: we never want this to appear in logs, panics, JSON, or
/// any external representation. The inner value is zeroized when dropped.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PlaintextPassword(String);

impl PlaintextPassword {
    pub fn parse(raw: impl Into<String>) -> Result<Self, PlaintextPasswordError> {
        let s = raw.into();
        let len = s.chars().count();
        if len < MIN_LEN {
            return Err(PlaintextPasswordError::TooShort(MIN_LEN));
        }
        if len > MAX_LEN {
            return Err(PlaintextPasswordError::TooLong(MAX_LEN));
        }
        Ok(Self(s))
    }

    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HashedPasswordError {
    #[error("hashed password string is empty")]
    Empty,
}

/// A password hash in PHC string format (e.g., `$argon2id$...`).
///
/// Constructed from a string already produced by the `PasswordHasher` port
/// (plan 1b) or read back from the database. We do not validate the PHC
/// format here — that's the hasher's responsibility — we only ensure the
/// string is non-empty.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_phc_string(s: impl Into<String>) -> Result<Self, HashedPasswordError> {
        let s = s.into();
        if s.is_empty() {
            return Err(HashedPasswordError::Empty);
        }
        Ok(Self(s))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashedPassword")
            .field("value", &"<redacted>")
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for HashedPassword {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_phc_string(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn plaintext_accepts_min_length() {
        let p = PlaintextPassword::parse("aaaaaaaaaaaa").unwrap();
        assert_eq!(p.expose(), "aaaaaaaaaaaa");
    }

    #[test]
    fn plaintext_rejects_short() {
        let err = PlaintextPassword::parse("short").err().unwrap();
        assert_eq!(err, PlaintextPasswordError::TooShort(12));
    }

    #[test]
    fn plaintext_rejects_too_long() {
        let huge = "a".repeat(257);
        let err = PlaintextPassword::parse(huge).err().unwrap();
        assert_eq!(err, PlaintextPasswordError::TooLong(256));
    }

    #[test]
    fn plaintext_counts_chars_not_bytes() {
        // 11 multibyte chars (crab emojis): 11 chars < 12 chars → TooShort
        let s = "🦀".repeat(11);
        let err = PlaintextPassword::parse(s).err().unwrap();
        assert_eq!(err, PlaintextPasswordError::TooShort(12));
    }

    #[test]
    fn plaintext_accepts_at_max_length() {
        let s = "a".repeat(256);
        let p = PlaintextPassword::parse(s).unwrap();
        assert_eq!(p.expose().chars().count(), 256);
    }

    #[test]
    fn hashed_password_accepts_non_empty() {
        let h = HashedPassword::from_phc_string("$argon2id$...").unwrap();
        assert_eq!(h.as_str(), "$argon2id$...");
    }

    #[test]
    fn hashed_password_rejects_empty() {
        assert_eq!(
            HashedPassword::from_phc_string(""),
            Err(HashedPasswordError::Empty)
        );
    }

    #[test]
    fn hashed_password_debug_does_not_leak() {
        let h = HashedPassword::from_phc_string("$argon2id$secret-stuff").unwrap();
        let dbg = format!("{h:?}");
        assert!(dbg.contains("redacted"));
        assert!(!dbg.contains("secret-stuff"));
    }
}
