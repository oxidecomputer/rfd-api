use std::{collections::HashSet, fmt::Debug, hash::Hash};

use diesel::{sql_types::Jsonb, AsExpression, FromSqlRow};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

pub trait Permission:
    Clone + Debug + Eq + PartialEq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
}
impl<T> Permission for T where
    T: Clone + Debug + Eq + PartialEq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
}

#[derive(Debug)]
pub struct Caller<T> {
    pub id: Uuid,
    pub permissions: Permissions<T>,
}

impl<T> Caller<T>
where
    T: Permission,
{
    pub fn is(&self, id: &Uuid) -> bool {
        &self.id == id
    }

    pub fn all(&self, permissions: &[&T]) -> bool {
        self.permissions.all(permissions)
    }

    pub fn any(&self, permissions: &[&T]) -> bool {
        self.permissions.any(permissions)
    }

    pub fn can(&self, permission: &T) -> bool {
        let result = self.permissions.can(permission);
        tracing::debug!(?permission, ?result, "Test if caller can");
        result
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, FromSqlRow, AsExpression, JsonSchema)]
#[diesel(sql_type = Jsonb)]
pub struct Permissions<T>(Vec<T>);

impl<T> Default for Permissions<T>
where
    T: Permission,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Permissions<T>
where
    T: Permission,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn inner(&self) -> &[T] {
        &self.0
    }

    pub fn all(&self, permissions: &[&T]) -> bool {
        permissions.iter().all(|p| self.can(p))
    }

    pub fn any(&self, permissions: &[&T]) -> bool {
        permissions.iter().any(|p| self.can(p))
    }

    pub fn can(&self, permission: &T) -> bool {
        let res = self.0.contains(permission);
        tracing::trace!(available = ?self.0, requested = ?permission, result = ?res, "Permissions existence check");
        res
    }

    pub fn intersect(&self, other: &Permissions<T>) -> Permissions<T> {
        let left: HashSet<&T> = self.0.iter().collect();
        let right: HashSet<&T> = other.0.iter().collect();
        let intersection: Vec<T> = left
            .intersection(&right)
            .into_iter()
            .map(|p| *p)
            .cloned()
            .collect();
        intersection.into()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> From<Vec<T>> for Permissions<T>
where
    T: Permission,
{
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<'a, T> From<&'a Permissions<T>> for &'a [T]
where
    T: Permission,
{
    fn from(value: &'a Permissions<T>) -> Self {
        value.inner()
    }
}
