use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    pg::Pg,
    query_builder::QueryId,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Jsonb,
    AsExpression, FromSqlRow,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    io::Write,
};

use crate::{
    permissions::Permissions,
    schema::sql_types::{AttemptState, RfdContentFormat, RfdPdfSource, RfdVisibility},
};

macro_rules! sql_conversion {
    (
        $sql_t:ident => $model_t:ident,
        $($to_matcher:tt => $to_result:tt),*,
    ) => {
        impl ToSql<$sql_t, Pg> for $model_t {
            fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
                match *self {
                    $($model_t::$to_matcher => out.write_all($to_result)?),*
                };

                Ok(IsNull::No)
            }
        }

        impl FromSql<$sql_t, Pg> for $model_t {
            fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                match bytes.as_bytes() {
                    $($to_result => Ok($model_t::$to_matcher)),*,
                    x => Err(format!("Unrecognized {} variant {:?}", stringify!($sql_t), x).into()),
                }
            }
        }

        impl QueryId for $sql_t {
            type QueryId = $sql_t;
            const HAS_STATIC_QUERY_ID: bool = true;
        }
    };
}

#[derive(Debug, PartialEq, Clone, FromSqlRow, AsExpression, Serialize, Deserialize, JsonSchema)]
#[diesel(sql_type = RfdContentFormat)]
#[serde(rename_all = "lowercase")]
pub enum ContentFormat {
    Asciidoc,
    Markdown,
}

sql_conversion! {
    RfdContentFormat => ContentFormat,
    Asciidoc => b"asciidoc",
    Markdown => b"markdown",
}

impl Display for ContentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentFormat::Asciidoc => write!(f, "asciidoc"),
            ContentFormat::Markdown => write!(f, "markdown"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, FromSqlRow, AsExpression, Serialize, Deserialize, JsonSchema)]
#[diesel(sql_type = RfdPdfSource)]
#[serde(rename_all = "lowercase")]
pub enum PdfSource {
    GitHub,
    Google,
}

sql_conversion! {
    RfdPdfSource => PdfSource,
    GitHub => b"github",
    Google => b"google",
}

impl Display for PdfSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfSource::GitHub => write!(f, "github"),
            PdfSource::Google => write!(f, "google"),
        }
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

#[derive(Debug, PartialEq, Clone, FromSqlRow, AsExpression, Serialize, Deserialize, JsonSchema)]
#[diesel(sql_type = AttemptState)]
#[serde(rename_all = "lowercase")]
pub enum LoginAttemptState {
    Complete,
    Failed,
    New,
    RemoteAuthenticated,
}

sql_conversion! {
    AttemptState => LoginAttemptState,
    Complete => b"complete",
    Failed => b"failed",
    New => b"new",
    RemoteAuthenticated => b"remote_authenticated",
}

impl Display for LoginAttemptState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginAttemptState::Complete => write!(f, "complete"),
            LoginAttemptState::Failed => write!(f, "failed"),
            LoginAttemptState::New => write!(f, "new"),
            LoginAttemptState::RemoteAuthenticated => write!(f, "remote_authenticated"),
        }
    }
}

impl Default for LoginAttemptState {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Debug, PartialEq, Clone, FromSqlRow, AsExpression, Serialize, Deserialize, JsonSchema)]
#[diesel(sql_type = RfdVisibility)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

sql_conversion! {
    RfdVisibility => Visibility,
    Public => b"public",
    Private => b"private",
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Private => write!(f, "private"),
        }
    }
}
