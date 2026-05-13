use std::fmt;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RoleError {
    #[error("unknown role: {0}")]
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Role {
    Admin,
    Manager,
    Operator,
}

impl Role {
    /// String representation matching the DB CHECK constraint.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Manager => "manager",
            Self::Operator => "operator",
        }
    }
}

impl FromStr for Role {
    type Err = RoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "operator" => Ok(Self::Operator),
            other => Err(RoleError::Unknown(other.to_string())),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn as_str_matches_schema() {
        assert_eq!(Role::Admin.as_str(), "admin");
        assert_eq!(Role::Manager.as_str(), "manager");
        assert_eq!(Role::Operator.as_str(), "operator");
    }

    #[test]
    fn display_matches_as_str() {
        assert_eq!(format!("{}", Role::Admin), "admin");
        assert_eq!(format!("{}", Role::Manager), "manager");
        assert_eq!(format!("{}", Role::Operator), "operator");
    }

    #[test]
    fn from_str_known_values() {
        assert_eq!("admin".parse::<Role>().unwrap(), Role::Admin);
        assert_eq!("manager".parse::<Role>().unwrap(), Role::Manager);
        assert_eq!("operator".parse::<Role>().unwrap(), Role::Operator);
    }

    #[test]
    fn from_str_rejects_unknown() {
        let err = "owner".parse::<Role>().unwrap_err();
        assert_eq!(err, RoleError::Unknown("owner".to_string()));
    }

    #[test]
    fn from_str_is_case_sensitive() {
        // The DB schema and serde representation are lowercase only.
        assert!("Admin".parse::<Role>().is_err());
        assert!("ADMIN".parse::<Role>().is_err());
    }

    #[test]
    fn roundtrips_through_string() {
        for role in [Role::Admin, Role::Manager, Role::Operator] {
            let s = role.as_str();
            let parsed: Role = s.parse().unwrap();
            assert_eq!(parsed, role);
        }
    }
}
