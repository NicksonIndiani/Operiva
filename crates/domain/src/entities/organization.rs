use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::{OrganizationName, OrganizationSlug};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    pub id: Uuid,
    pub slug: OrganizationSlug,
    pub name: OrganizationName,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Organization {
    #[must_use]
    pub fn new(
        id: Uuid,
        slug: OrganizationSlug,
        name: OrganizationName,
        now: OffsetDateTime,
    ) -> Self {
        Self { id, slug, name, created_at: now, updated_at: now }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn new_organization_has_matching_timestamps() {
        let now = time::OffsetDateTime::UNIX_EPOCH;
        let org = Organization::new(
            Uuid::nil(),
            OrganizationSlug::parse("acme").unwrap(),
            OrganizationName::parse("Acme").unwrap(),
            now,
        );
        assert_eq!(org.created_at, org.updated_at);
        assert_eq!(org.slug.as_str(), "acme");
        assert_eq!(org.name.as_str(), "Acme");
    }
}
