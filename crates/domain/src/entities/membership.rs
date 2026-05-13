use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::Role;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: Role,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Membership {
    #[must_use]
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        organization_id: Uuid,
        role: Role,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            organization_id,
            role,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn new_membership_stores_role() {
        let now = time::OffsetDateTime::UNIX_EPOCH;
        let m = Membership::new(Uuid::nil(), Uuid::nil(), Uuid::nil(), Role::Admin, now);
        assert_eq!(m.role, Role::Admin);
        assert_eq!(m.created_at, m.updated_at);
    }
}
