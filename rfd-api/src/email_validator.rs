// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait EmailValidator {
    fn validate(&self, email: &str) -> bool;
}

pub struct DomainValidator {
    valid_domains: Vec<String>,
}

impl DomainValidator {
    pub fn new(valid_domains: Vec<String>) -> Self {
        Self { valid_domains }
    }
}

impl EmailValidator for DomainValidator {
    fn validate(&self, email: &str) -> bool {
        self.valid_domains
            .iter()
            .any(|domain| email.ends_with(domain))
    }
}
