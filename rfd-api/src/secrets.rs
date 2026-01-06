// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use schemars::{
    schema::{InstanceType, SchemaObject},
    JsonSchema,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OpenApiSecretString(pub SecretString);

impl From<SecretString> for OpenApiSecretString {
    fn from(value: SecretString) -> Self {
        Self(value)
    }
}

impl JsonSchema for OpenApiSecretString {
    fn is_referenceable() -> bool {
        true
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        Cow::Borrowed("secret-string")
    }

    fn schema_name() -> String {
        "SecretString".to_string()
    }
}

impl Serialize for OpenApiSecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.expose_secret())
    }
}
