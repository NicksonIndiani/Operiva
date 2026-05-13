use std::fmt;
use std::sync::OnceLock;

use regex::Regex;
use thiserror::Error;

const RESERVED: &[&str] = &[
    "api", "admin", "www", "auth", "app", "static", "assets", "health", "healthz", "readyz",
];

/// Static regex — `expect` is safe (compile-time-checked literal) but the
/// workspace lint `clippy::expect_used = "deny"` requires an explicit allow.
#[allow(clippy::expect_used)]
fn slug_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[a-z0-9][a-z0-9-]{1,62}[a-z0-9]$").expect("static slug regex is valid")
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OrganizationSlugError {
    #[error("slug is empty")]
    Empty,
    #[error("slug must be 3 to 64 lowercase alphanumeric characters and hyphens")]
    InvalidFormat,
    #[error("slug cannot contain consecutive hyphens")]
    ConsecutiveHyphens,
    #[error("slug \"{0}\" is reserved")]
    Reserved(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OrganizationSlug(String);

impl OrganizationSlug {
    pub fn parse(raw: &str) -> Result<Self, OrganizationSlugError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(OrganizationSlugError::Empty);
        }
        let lower = trimmed.to_ascii_lowercase();
        if !slug_regex().is_match(&lower) {
            return Err(OrganizationSlugError::InvalidFormat);
        }
        if lower.contains("--") {
            return Err(OrganizationSlugError::ConsecutiveHyphens);
        }
        if RESERVED.contains(&lower.as_str()) {
            return Err(OrganizationSlugError::Reserved(lower));
        }
        Ok(Self(lower))
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

impl fmt::Display for OrganizationSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrganizationSlug {
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
    fn parses_valid_slug() {
        let s = OrganizationSlug::parse("acme-corp").unwrap();
        assert_eq!(s.as_str(), "acme-corp");
    }

    #[test]
    fn lowercases() {
        let s = OrganizationSlug::parse("Acme-Corp").unwrap();
        assert_eq!(s.as_str(), "acme-corp");
    }

    #[test]
    fn trims_whitespace() {
        let s = OrganizationSlug::parse("  acme-corp  ").unwrap();
        assert_eq!(s.as_str(), "acme-corp");
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(
            OrganizationSlug::parse(""),
            Err(OrganizationSlugError::Empty)
        );
        assert_eq!(
            OrganizationSlug::parse("   "),
            Err(OrganizationSlugError::Empty)
        );
    }

    #[test]
    fn rejects_too_short() {
        assert_eq!(
            OrganizationSlug::parse("ab"),
            Err(OrganizationSlugError::InvalidFormat)
        );
    }

    #[test]
    fn accepts_min_length() {
        // 3 chars is the minimum per regex
        let s = OrganizationSlug::parse("abc").unwrap();
        assert_eq!(s.as_str(), "abc");
    }

    #[test]
    fn rejects_too_long() {
        let huge = "a".repeat(65);
        assert_eq!(
            OrganizationSlug::parse(&huge),
            Err(OrganizationSlugError::InvalidFormat)
        );
    }

    #[test]
    fn accepts_max_length() {
        let s = "a".repeat(64);
        let parsed = OrganizationSlug::parse(&s).unwrap();
        assert_eq!(parsed.as_str().len(), 64);
    }

    #[test]
    fn rejects_starting_with_hyphen() {
        assert_eq!(
            OrganizationSlug::parse("-acme"),
            Err(OrganizationSlugError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_ending_with_hyphen() {
        assert_eq!(
            OrganizationSlug::parse("acme-"),
            Err(OrganizationSlugError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_invalid_chars() {
        assert_eq!(
            OrganizationSlug::parse("acme_corp"),
            Err(OrganizationSlugError::InvalidFormat)
        );
        assert_eq!(
            OrganizationSlug::parse("acme.corp"),
            Err(OrganizationSlugError::InvalidFormat)
        );
        assert_eq!(
            OrganizationSlug::parse("acme corp"),
            Err(OrganizationSlugError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_consecutive_hyphens() {
        assert_eq!(
            OrganizationSlug::parse("acme--corp"),
            Err(OrganizationSlugError::ConsecutiveHyphens)
        );
    }

    #[test]
    fn rejects_reserved() {
        assert_eq!(
            OrganizationSlug::parse("api"),
            Err(OrganizationSlugError::Reserved("api".to_string()))
        );
        assert_eq!(
            OrganizationSlug::parse("Admin"),
            Err(OrganizationSlugError::Reserved("admin".to_string()))
        );
        assert_eq!(
            OrganizationSlug::parse("WWW"),
            Err(OrganizationSlugError::Reserved("www".to_string()))
        );
    }
}
