// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{collections::BTreeSet, fmt::Debug, hash::Hash};

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::Jsonb,
    AsExpression, FromSqlRow,
};
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

#[derive(Clone, Debug)]
pub struct Caller<T: Ord> {
    pub id: Uuid,
    pub permissions: Permissions<T>,
}

impl<T> Caller<T>
where
    T: Permission + Ord,
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
        tracing::trace!(?permission, ?result, "Test if caller can");
        result
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, FromSqlRow, AsExpression, JsonSchema)]
#[diesel(sql_type = Jsonb)]
pub struct Permissions<T: Ord>(BTreeSet<T>);

impl<T> Default for Permissions<T>
where
    T: Permission + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Permissions<T>
where
    T: Permission + Ord,
{
    pub fn new() -> Self {
        Self(BTreeSet::new())
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
        self.0
            .intersection(&other.0)
            .into_iter()
            .map(|p| p.clone())
            .collect::<BTreeSet<_>>()
            .into()
    }

    pub fn insert(&mut self, item: T) -> bool {
        self.0.insert(item)
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    pub fn remove(&mut self, item: &T) -> bool {
        self.0.remove(item)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> From<BTreeSet<T>> for Permissions<T>
where
    T: Permission + Ord,
{
    fn from(value: BTreeSet<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Vec<T>> for Permissions<T>
where
    T: Permission + Ord,
{
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value)
    }
}

impl<T> FromIterator<T> for Permissions<T>
where
    T: Permission + Ord,
{
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let mut set = BTreeSet::new();
        set.extend::<Iter>(iter);
        Self(set)
    }
}

impl<T> ToSql<Jsonb, Pg> for Permissions<T>
where
    T: Serialize + Debug + Ord,
{
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

impl<T> FromSql<Jsonb, Pg> for Permissions<T>
where
    T: DeserializeOwned + Debug + Ord,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
