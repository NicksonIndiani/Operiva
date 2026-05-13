use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TokenHashError {
    #[error("token hash must not be empty")]
    Empty,
}

/// An opaque token hash stored in the database.
///
/// Only the **hash** of a plaintext token is ever persisted; the plaintext is
/// sent to the user (via email, SMS, etc.) and never stored. This type wraps
/// the hash string to prevent bare `String` values from bypassing validation.
///
/// # Format constraint
///
/// This constructor only enforces **non-empty**. The exact encoding (SHA-256
/// hex, base64url-no-padding, etc.) is the responsibility of the infrastructure
/// crypto layer introduced in plan 1b. Constraining the format here would
/// couple the domain to a specific hashing algorithm and would need to change
/// every time the algorithm changes — a domain-layer anti-pattern.
///
/// # Debug output
///
/// Unlike `HashedPassword`, `TokenHash` deliberately derives `Debug` without
/// redaction. The hash itself is not sensitive: an attacker who already holds
/// the hash gains nothing, because it is computationally infeasible to reverse
/// it to the plaintext token. The plaintext token (delivered via email) is what
/// must be kept secret. Contrast with `HashedPassword`, which has a custom
/// redacting `Debug` to avoid leaking the PHC string in logs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TokenHash(String);

impl TokenHash {
    /// Constructs a `TokenHash` from a pre-computed hash string.
    ///
    /// Returns `Err(TokenHashError::Empty)` if `s` is empty. See the type-level
    /// documentation for why no further format validation is applied here.
    pub fn from_hash_string(s: impl Into<String>) -> Result<Self, TokenHashError> {
        let s = s.into();
        if s.is_empty() {
            return Err(TokenHashError::Empty);
        }
        Ok(Self(s))
    }

    /// Returns a reference to the inner hash string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes `self` and returns the inner `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TokenHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_hash_string(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_empty_hex_string() {
        let t = TokenHash::from_hash_string("abc123def").unwrap();
        assert_eq!(t.as_str(), "abc123def");
    }

    #[test]
    fn accepts_typical_sha256_hex() {
        // A 64-char hex string (typical SHA-256 hex representation)
        let hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let t = TokenHash::from_hash_string(hash).unwrap();
        assert_eq!(t.as_str(), hash);
    }

    #[test]
    fn accepts_typical_base64url() {
        // A 43-char base64url-no-pad string (typical SHA-256 base64url)
        let hash = "47DEQpj8HBSa-_TImW-5JCeuQeRkm5NMpJWZG3hSuFU";
        let t = TokenHash::from_hash_string(hash).unwrap();
        assert_eq!(t.as_str(), hash);
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(TokenHash::from_hash_string(""), Err(TokenHashError::Empty));
    }

    #[test]
    fn debug_does_not_redact() {
        // Token hashes are opaque to attackers (the plaintext token is the secret,
        // not the hash). Debug output is allowed to show the value, unlike HashedPassword.
        let t = TokenHash::from_hash_string("abc").unwrap();
        let dbg = format!("{t:?}");
        assert!(dbg.contains("abc"));
    }
}
