use std::fmt;
use std::sync::OnceLock;

use regex::Regex;
use thiserror::Error;

const MAX_LEN: usize = 254;

/// Static regex — `expect` is safe (compile-time-checked literal) but the
/// workspace lint `clippy::expect_used = "deny"` requires an explicit allow.
#[allow(clippy::expect_used)]
fn email_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // Local part: one or more chunks of allowed chars (letters/digits/_%+-),
        // separated by single dots (no leading/trailing/consecutive dots).
        // Domain: labels separated by dots, TLD must be 2+ letters.
        Regex::new(r"^[A-Za-z0-9_%+\-]+(?:\.[A-Za-z0-9_%+\-]+)*@[A-Za-z0-9](?:[A-Za-z0-9\-]*[A-Za-z0-9])?(?:\.[A-Za-z0-9](?:[A-Za-z0-9\-]*[A-Za-z0-9])?)*\.[A-Za-z]{2,}$")
            .expect("static email regex is valid")
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmailError {
    #[error("email is empty")]
    Empty,
    #[error("email is longer than 254 characters")]
    TooLong,
    #[error("email format is invalid")]
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Email(String);

impl Email {
    pub fn parse(raw: &str) -> Result<Self, EmailError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(EmailError::Empty);
        }
        if trimmed.len() > MAX_LEN {
            return Err(EmailError::TooLong);
        }
        if !email_regex().is_match(trimmed) {
            return Err(EmailError::InvalidFormat);
        }
        Ok(Self(trimmed.to_ascii_lowercase()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_email() {
        let email = Email::parse("alice@example.com").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn trims_whitespace() {
        let email = Email::parse("  alice@example.com  ").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn lowercases_input() {
        let email = Email::parse("Alice@Example.COM").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(Email::parse(""), Err(EmailError::Empty));
        assert_eq!(Email::parse("   "), Err(EmailError::Empty));
    }

    #[test]
    fn rejects_too_long() {
        let local = "a".repeat(250);
        let huge = format!("{local}@b.co");
        assert_eq!(Email::parse(&huge), Err(EmailError::TooLong));
    }

    #[test]
    fn rejects_no_at_sign() {
        assert_eq!(
            Email::parse("aliceexample.com"),
            Err(EmailError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_multiple_at_signs() {
        assert_eq!(Email::parse("a@b@c.com"), Err(EmailError::InvalidFormat));
    }

    #[test]
    fn rejects_missing_local_part() {
        assert_eq!(Email::parse("@example.com"), Err(EmailError::InvalidFormat));
    }

    #[test]
    fn rejects_missing_domain() {
        assert_eq!(Email::parse("alice@"), Err(EmailError::InvalidFormat));
    }

    #[test]
    fn rejects_domain_without_tld() {
        assert_eq!(
            Email::parse("alice@example"),
            Err(EmailError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_leading_dot_in_local_part() {
        assert_eq!(
            Email::parse(".alice@example.com"),
            Err(EmailError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_trailing_dot_in_local_part() {
        assert_eq!(
            Email::parse("alice.@example.com"),
            Err(EmailError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_consecutive_dots_in_local_part() {
        assert_eq!(
            Email::parse("alice..bob@example.com"),
            Err(EmailError::InvalidFormat)
        );
    }

    #[test]
    fn accepts_max_length_email() {
        // 254 chars total: local part 249 chars + "@b.co" (5) = 254
        let local = "a".repeat(249);
        let s = format!("{local}@b.co");
        assert_eq!(s.len(), 254);
        let email = Email::parse(&s).unwrap();
        assert_eq!(email.as_str().len(), 254);
    }

    #[test]
    fn parses_plus_addressing() {
        let email = Email::parse("alice+filter@example.com").unwrap();
        assert_eq!(email.as_str(), "alice+filter@example.com");
    }
}
