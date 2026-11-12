// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use schemars::{
    schema::{InstanceType, Schema, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

pub mod content;

#[derive(Debug, Copy, Clone)]
pub struct RfdNumber(i32);

impl RfdNumber {
    /// Get the path to where the source contents of this RFD exists in the RFD repo.
    pub fn repo_path(&self) -> String {
        format!("rfd/{}", self.as_number_string())
    }

    /// Get a RFD number in its expanded form with leading 0s
    pub fn as_number_string(&self) -> String {
        let mut number_string = self.0.to_string();
        while number_string.len() < 4 {
            number_string = format!("0{}", number_string);
        }

        number_string
    }
}

impl Display for RfdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for RfdNumber {
    fn from(num: i32) -> Self {
        Self(num)
    }
}

impl From<&i32> for RfdNumber {
    fn from(num: &i32) -> Self {
        Self(*num)
    }
}

impl From<RfdNumber> for i32 {
    fn from(num: RfdNumber) -> Self {
        num.0
    }
}

impl From<&RfdNumber> for i32 {
    fn from(num: &RfdNumber) -> Self {
        num.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RfdState {
    Abandoned,
    Committed,
    Discussion,
    Ideation,
    Prediscussion,
    Published,
}

impl Display for RfdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RfdState::Abandoned => write!(f, "abandoned"),
            RfdState::Committed => write!(f, "committed"),
            RfdState::Discussion => write!(f, "discussion"),
            RfdState::Ideation => write!(f, "ideation"),
            RfdState::Prediscussion => write!(f, "prediscussion"),
            RfdState::Published => write!(f, "published"),
        }
    }
}

#[derive(Debug)]
pub struct InvalidRfdState<'a>(pub &'a str);

impl<'a> TryFrom<&'a str> for RfdState {
    type Error = InvalidRfdState<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "abandoned" => Ok(RfdState::Abandoned),
            "committed" => Ok(RfdState::Committed),
            "discussion" => Ok(RfdState::Discussion),
            "ideation" => Ok(RfdState::Ideation),
            "prediscussion" => Ok(RfdState::Prediscussion),
            "published" => Ok(RfdState::Published),
            _ => Err(InvalidRfdState(value)),
        }
    }
}

/// A string that contains no line breaks (`\r` or `\n`). Fails to deserialize otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct SingleLineString(String);

/// Inlined as a string with a pattern so that generated clients validate it too
impl JsonSchema for SingleLineString {
    fn schema_name() -> String {
        "SingleLineString".to_string()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(_: &mut schemars::r#gen::SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation {
                pattern: Some(r"^[^\r\n]*$".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl SingleLineString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SingleLineString {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains(['\r', '\n']) {
            Err("value must not contain line breaks")
        } else {
            Ok(Self(value))
        }
    }
}

impl From<SingleLineString> for String {
    fn from(value: SingleLineString) -> Self {
        value.0
    }
}

impl From<RfdState> for SingleLineString {
    fn from(state: RfdState) -> Self {
        Self(state.to_string())
    }
}

impl std::ops::Deref for SingleLineString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SingleLineString;

    #[test]
    fn test_single_line_string_rejects_line_breaks() {
        for value in ["a\nb", "a\r\nb", "a\rb", "\n"] {
            assert!(SingleLineString::try_from(value.to_string()).is_err());
            assert!(serde_json::from_value::<SingleLineString>(serde_json::json!(value)).is_err());
        }

        let parsed: SingleLineString = serde_json::from_value(serde_json::json!("ok")).unwrap();
        assert_eq!(parsed.as_str(), "ok");
    }
}
