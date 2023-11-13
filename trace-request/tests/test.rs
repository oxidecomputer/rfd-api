// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use trace_request::trace_request;

#[trace_request]
#[endpoint {
    method = GET,
    path = "/test"
}]
async fn _trace_entry_exit(rqctx: RequestContext<()>) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}
