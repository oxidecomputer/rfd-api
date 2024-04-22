// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template is missing some of the required values")]
    MissingRequiredFields {
        template: RfdTemplate,
        values: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RfdTemplate {
    template: String,
    #[serde(default)]
    values: HashMap<String, String>,
    required_fields: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RenderableRfdTemplate(RfdTemplate);

impl RfdTemplate {
    pub fn field(mut self, field: String, value: String) -> Self {
        self.values.insert(field, value);
        self
    }

    pub fn build(self) -> Result<RenderableRfdTemplate, TemplateError> {
        let set_fields = self.values.keys().collect::<Vec<_>>();
        let missing_fields = self
            .required_fields
            .iter()
            .filter(|field| !set_fields.contains(field))
            .cloned()
            .collect::<Vec<_>>();

        if missing_fields.len() == 0 {
            Ok(RenderableRfdTemplate(self))
        } else {
            Err(TemplateError::MissingRequiredFields {
                template: self,
                values: missing_fields,
            })
        }
    }
}

impl RenderableRfdTemplate {
    pub fn render(self) -> String {
        let mut rendered = self.0.template;

        for field in self.0.required_fields {
            rendered = rendered.replace(&format!("{{{}}}", field), self.0.values.get(&field).expect("Renderable template is missing a required field. This is a bug, please report."));
        }

        rendered
    }
}
