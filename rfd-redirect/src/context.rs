use regex::Regex;

pub struct Context {
    pub host_regex: Regex,
    pub github_template: String,
}

impl Context {
    pub fn is_rfd_number_valid(&self, rfd_number: u16) -> bool {
        0 < rfd_number
    }
}
