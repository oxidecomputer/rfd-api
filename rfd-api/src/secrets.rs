use schemars::{JsonSchema, schema::{SchemaObject, InstanceType}};
use secrecy::{SecretString, ExposeSecret};
use serde::{Deserialize, Serializer, Serialize};
use std::borrow::Cow;

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
        }.into()
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