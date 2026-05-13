use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::TokenHash;

/// Email verification token. Only the **hash** of the plaintext token is stored —
/// the plaintext is sent to the user via email and never persisted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: TokenHash,
    pub expires_at: OffsetDateTime,
    pub consumed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl EmailVerificationToken {
    #[must_use]
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        token_hash: TokenHash,
        expires_at: OffsetDateTime,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            expires_at,
            consumed_at: None,
            created_at: now,
        }
    }

    #[must_use]
    pub fn is_expired(&self, now: OffsetDateTime) -> bool {
        now >= self.expires_at
    }

    #[must_use]
    pub fn is_consumed(&self) -> bool {
        self.consumed_at.is_some()
    }

    #[must_use]
    pub fn is_usable(&self, now: OffsetDateTime) -> bool {
        !self.is_consumed() && !self.is_expired(now)
    }

    /// Marks the token consumed at the given instant. Idempotent.
    pub fn mark_consumed(&mut self, at: OffsetDateTime) {
        if self.consumed_at.is_none() {
            self.consumed_at = Some(at);
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use time::Duration;
    use uuid::Uuid;

    use super::*;

    fn fresh(now: OffsetDateTime) -> EmailVerificationToken {
        EmailVerificationToken::new(
            Uuid::nil(),
            Uuid::nil(),
            TokenHash::from_hash_string("hash").unwrap(),
            now + Duration::hours(24),
            now,
        )
    }

    #[test]
    fn fresh_token_is_usable() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let t = fresh(now);
        assert!(t.is_usable(now));
        assert!(!t.is_expired(now));
        assert!(!t.is_consumed());
    }

    #[test]
    fn expired_token_is_not_usable() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let t = fresh(now);
        let later = now + Duration::hours(25);
        assert!(t.is_expired(later));
        assert!(!t.is_usable(later));
    }

    #[test]
    fn consumed_token_is_not_usable() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let mut t = fresh(now);
        t.mark_consumed(now);
        assert!(t.is_consumed());
        assert!(!t.is_usable(now));
    }

    #[test]
    fn mark_consumed_is_idempotent() {
        let now = OffsetDateTime::UNIX_EPOCH;
        let mut t = fresh(now);
        let first = now + Duration::seconds(10);
        let second = now + Duration::seconds(20);
        t.mark_consumed(first);
        t.mark_consumed(second);
        assert_eq!(t.consumed_at, Some(first));
    }
}
