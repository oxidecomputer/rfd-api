// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod content;

#[derive(Debug, Copy, Clone)]
pub struct RfdNumber(i32);

impl RfdNumber {
    /// Get the path to where the source contents of this RFD exists in the RFD repo.
    pub fn repo_path(&self) -> String {
        format!("/rfd/{}", self.as_number_string())
    }

    /// Get an RFD number in its expanded form with leading 0s
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
