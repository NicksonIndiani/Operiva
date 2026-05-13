use std::fmt;

use thiserror::Error;

const MAX_LEN: usize = 100;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DisplayNameError {
    #[error("name is empty")]
    Empty,
    #[error("name must be at most {0} characters")]
    TooLong(usize),
}

fn validate(raw: &str) -> Result<String, DisplayNameError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(DisplayNameError::Empty);
    }
    if trimmed.chars().count() > MAX_LEN {
        return Err(DisplayNameError::TooLong(MAX_LEN));
    }
    Ok(trimmed.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UserName(String);

impl UserName {
    pub fn parse(raw: &str) -> Result<Self, DisplayNameError> {
        validate(raw).map(Self)
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

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UserName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OrganizationName(String);

impl OrganizationName {
    pub fn parse(raw: &str) -> Result<Self, DisplayNameError> {
        validate(raw).map(Self)
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

impl fmt::Display for OrganizationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OrganizationName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    // --- UserName ---

    #[test]
    fn user_name_parses_valid() {
        let n = UserName::parse("Alice").unwrap();
        assert_eq!(n.as_str(), "Alice");
    }

    #[test]
    fn user_name_trims_whitespace() {
        let n = UserName::parse("  Alice  ").unwrap();
        assert_eq!(n.as_str(), "Alice");
    }

    #[test]
    fn user_name_rejects_empty() {
        assert_eq!(UserName::parse(""), Err(DisplayNameError::Empty));
        assert_eq!(UserName::parse("   "), Err(DisplayNameError::Empty));
    }

    #[test]
    fn user_name_rejects_too_long() {
        let huge = "a".repeat(101);
        assert_eq!(UserName::parse(&huge), Err(DisplayNameError::TooLong(100)));
    }

    #[test]
    fn user_name_accepts_at_max_length() {
        let s = "a".repeat(100);
        let n = UserName::parse(&s).unwrap();
        assert_eq!(n.as_str().chars().count(), 100);
    }

    #[test]
    fn user_name_counts_chars_not_bytes() {
        // 100 multibyte chars (each emoji is 4 bytes UTF-8) — should be accepted
        let s = "🦀".repeat(100);
        let n = UserName::parse(&s).unwrap();
        assert_eq!(n.as_str().chars().count(), 100);
    }

    #[test]
    fn user_name_rejects_over_max_chars() {
        // 101 multibyte chars — should be rejected
        let s = "🦀".repeat(101);
        assert_eq!(UserName::parse(&s), Err(DisplayNameError::TooLong(100)));
    }

    #[test]
    fn user_name_display_outputs_inner() {
        let n = UserName::parse("Bob").unwrap();
        assert_eq!(format!("{n}"), "Bob");
    }

    // --- OrganizationName ---

    #[test]
    fn organization_name_parses_valid() {
        let n = OrganizationName::parse(" Acme Corp ").unwrap();
        assert_eq!(n.as_str(), "Acme Corp");
    }

    #[test]
    fn organization_name_rejects_empty() {
        assert_eq!(OrganizationName::parse(""), Err(DisplayNameError::Empty));
    }

    #[test]
    fn organization_name_rejects_too_long() {
        let huge = "a".repeat(101);
        assert_eq!(
            OrganizationName::parse(&huge),
            Err(DisplayNameError::TooLong(100))
        );
    }
}
