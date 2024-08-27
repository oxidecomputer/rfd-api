// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{
    ApiDescription, ConfigDropshot, EndpointTagPolicy, HttpServerStarter, TagConfig, TagDetails,
};
use serde::Deserialize;
use slog::Drain;
use std::{collections::HashMap, error::Error, fs::File, net::SocketAddr, path::PathBuf};
use tracing_slog::TracingSlogDrain;

use crate::{
    context::RfdContext,
    endpoints::{
        rfd::{
            discuss_rfd, get_rfd, get_rfd_attr, get_rfds, publish_rfd, reserve_rfd, search_rfds,
            set_rfd_attr, set_rfd_content, set_rfd_document, update_rfd_visibility,
        },
        webhook::github_webhook,
    },
};

#[derive(Clone, Debug, Deserialize)]
pub struct SpecConfig {
    pub title: String,
    pub description: String,
    pub contact_url: String,
    pub contact_email: String,
    pub output_path: PathBuf,
}

pub struct ServerConfig {
    pub context: RfdContext,
    pub server_address: SocketAddr,
    pub spec_output: Option<SpecConfig>,
}

pub fn server(
    config: ServerConfig,
) -> Result<HttpServerStarter<RfdContext>, Box<dyn Error + Send + Sync>> {
    let mut config_dropshot = ConfigDropshot::default();
    config_dropshot.bind_address = config.server_address;
    config_dropshot.request_body_max_bytes = 1024 * 1024;

    // Construct a shim to pipe dropshot logs into the global tracing logger
    let dropshot_logger = {
        let level_drain = slog::LevelFilter(TracingSlogDrain, slog::Level::Debug).fuse();
        let async_drain = slog_async::Async::new(level_drain).build().fuse();
        slog::Logger::root(async_drain, slog::o!())
    };

    let mut tag_definitions = HashMap::new();
    tag_definitions.insert(
        "hidden".to_string(),
        TagDetails {
            description: Some("Internal endpoints".to_string()),
            external_docs: None,
        },
    );

    let mut api = ApiDescription::new().tag_config(TagConfig {
        allow_other_tags: false,
        endpoint_tag_policy: EndpointTagPolicy::Any,
        tag_definitions,
    });

    // RFDs
    api.register(get_rfds).expect("Failed to register endpoint");
    api.register(get_rfd).expect("Failed to register endpoint");
    api.register(reserve_rfd)
        .expect("Failed to register endpoint");
    api.register(set_rfd_document)
        .expect("Failed to register endpoint");
    api.register(set_rfd_content)
        .expect("Failed to register endpoint");
    api.register(get_rfd_attr)
        .expect("Failed to register endpoint");
    api.register(set_rfd_attr)
        .expect("Failed to register endpoint");
    api.register(discuss_rfd)
        .expect("Failed to register endpoint");
    api.register(publish_rfd)
        .expect("Failed to register endpoint");
    api.register(update_rfd_visibility)
        .expect("Failed to register endpoint");
    api.register(search_rfds)
        .expect("Failed to register endpoint");

    // Webhooks
    api.register(github_webhook)
        .expect("Failed to register endpoint");

    if let Some(spec) = config.spec_output {
        // TODO: How do we validate that spec_output can be read or written? Currently File::create
        // panics if the file path is not a valid path

        // Create the API schema.
        let mut api_definition = &mut api.openapi(spec.title, &"");
        api_definition = api_definition
            .description(spec.description)
            .contact_url(spec.contact_url)
            .contact_email(spec.contact_email);

        let mut buffer = File::create(spec.output_path).unwrap();
        api_definition.write(&mut buffer).unwrap();
    }

    HttpServerStarter::new(&config_dropshot, api, config.context, &dropshot_logger)
}
