use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::{Email, HashedPassword, UserName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub name: UserName,
    pub password_hash: HashedPassword,
    pub email_verified_at: Option<OffsetDateTime>,
    pub failed_login_attempts: u32,
    pub locked_until: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    /// Creates a freshly registered user: email NOT verified, no failed attempts.
    #[must_use]
    pub fn new(
        id: Uuid,
        email: Email,
        name: UserName,
        password_hash: HashedPassword,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            email,
            name,
            password_hash,
            email_verified_at: None,
            failed_login_attempts: 0,
            locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn is_email_verified(&self) -> bool {
        self.email_verified_at.is_some()
    }

    /// Marks the email as verified at the given instant. Idempotent: subsequent
    /// calls do not overwrite the original verification timestamp.
    pub fn mark_email_verified(&mut self, at: OffsetDateTime) {
        if self.email_verified_at.is_none() {
            self.email_verified_at = Some(at);
            self.updated_at = at;
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::value_objects::{Email, HashedPassword, UserName};

    fn sample(now: OffsetDateTime) -> User {
        User::new(
            Uuid::nil(),
            Email::parse("alice@example.com").unwrap(),
            UserName::parse("Alice").unwrap(),
            HashedPassword::from_phc_string("$argon2id$...").unwrap(),
            now,
        )
    }

    #[test]
    fn new_user_is_not_verified() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let u = sample(now);
        assert!(!u.is_email_verified());
        assert_eq!(u.failed_login_attempts, 0);
        assert!(u.locked_until.is_none());
        assert_eq!(u.created_at, u.updated_at);
    }

    #[test]
    fn mark_email_verified_sets_timestamp() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let mut u = sample(now);
        let later = now + time::Duration::seconds(60);
        u.mark_email_verified(later);
        assert!(u.is_email_verified());
        assert_eq!(u.email_verified_at, Some(later));
        assert_eq!(u.updated_at, later);
    }

    #[test]
    fn mark_email_verified_is_idempotent() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let mut u = sample(now);
        let first = now + time::Duration::seconds(60);
        let second = now + time::Duration::seconds(120);
        u.mark_email_verified(first);
        u.mark_email_verified(second);
        assert_eq!(u.email_verified_at, Some(first));
    }
}
