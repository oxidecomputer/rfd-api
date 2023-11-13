// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{ApiDescription, ConfigDropshot, EndpointTagPolicy, HttpServerStarter, TagConfig};
use slog::Drain;
use std::{error::Error, net::SocketAddr};
use tracing_slog::TracingSlogDrain;

use crate::{
    context::Context,
    endpoints::{redirect_host, redirect_path},
};

pub struct ServerConfig {
    pub context: Context,
    pub server_address: SocketAddr,
}

pub fn server(
    config: ServerConfig,
) -> Result<HttpServerStarter<Context>, Box<dyn Error + Send + Sync>> {
    let mut config_dropshot = ConfigDropshot::default();
    config_dropshot.bind_address = config.server_address;

    // Construct a shim to pipe dropshot logs into the global tracing logger
    let dropshot_logger = {
        let level_drain = slog::LevelFilter(TracingSlogDrain, slog::Level::Debug).fuse();
        let async_drain = slog_async::Async::new(level_drain).build().fuse();
        slog::Logger::root(async_drain, slog::o!())
    };

    let mut api = ApiDescription::new().tag_config(TagConfig {
        allow_other_tags: false,
        endpoint_tag_policy: EndpointTagPolicy::Any,
        tag_definitions: vec![].into_iter().collect(),
    });

    // Handle {rfd_number}.hostname redirects
    api.register(redirect_host).unwrap();

    // Handle hostname/{rfd_number} redirects
    api.register(redirect_path).unwrap();

    HttpServerStarter::new(&config_dropshot, api, config.context, &dropshot_logger)
}
