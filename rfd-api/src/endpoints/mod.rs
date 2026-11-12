// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub static UNLIMITED: i64 = 9999999;

/// Default request body limit. Endpoints that accept larger bodies opt in explicitly.
pub const DEFAULT_REQUEST_BODY_MAX_BYTES: usize = 16 * 1024;

/// Request body limit for endpoints that accept full RFD documents
pub const LARGE_REQUEST_BODY_MAX_BYTES: usize = 1024 * 1024;

/// GitHub caps webhook payloads at 25 MB, but observed push payloads are far smaller. The
/// signature can only be checked after the body is read, so keep this limit conservative.
/// https://docs.github.com/en/webhooks/webhook-events-and-payloads#payload-cap
pub const WEBHOOK_REQUEST_BODY_MAX_BYTES: usize = 1024 * 1024;

pub mod job;
pub mod rfd;
pub mod webhook;
