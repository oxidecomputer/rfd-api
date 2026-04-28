// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Verifies that every JSON response type the API can produce has a matching
// arm in the dispatch table in `src/main.rs`.
//
// The expected set of schema names is generated at build time by `build.rs`,
// which reads `rfd-api-spec.json` and emits Rust source that references real
// types from `rfd_sdk::types`. The compiler verifies each referenced type
// still exists, so SDK rename/removal breaks the build instead of silently
// passing the test.

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/expected_response_types.rs"));

#[test]
fn dispatch_table_covers_all_api_response_types() {
    let expected: HashSet<String> = expected_response_schema_names().into_iter().collect();

    let main_src =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs"))
            .expect("read rfd-cli/src/main.rs");

    // Names handled by the dispatch table in main.rs: any string literal
    // followed by `=>`. Over-collects on purpose (catches non-dispatch arms
    // too) — we only assert `expected ⊆ handled`.
    let arm_re = Regex::new(r#""([A-Za-z_][A-Za-z_0-9<>]*)"\s*=>"#).unwrap();
    let handled: HashSet<String> = arm_re
        .captures_iter(&main_src)
        .map(|c| c[1].to_string())
        .collect();

    let mut missing: Vec<&String> = expected.difference(&handled).collect();
    missing.sort();
    assert!(
        missing.is_empty(),
        "API responses with no dispatch arm in rfd-cli/src/main.rs:\n  - {}",
        missing
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  - ")
    );
}
