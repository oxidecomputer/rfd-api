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
