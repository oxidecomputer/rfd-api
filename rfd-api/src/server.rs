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
    context::ApiContext,
    endpoints::{
        api_user::{
            add_api_user_to_group, create_api_user, create_api_user_token, delete_api_user_token,
            get_api_user, get_api_user_token, get_self, list_api_user_tokens,
            remove_api_user_from_group, update_api_user,
        },
        group::{create_group, delete_group, get_groups, update_group},
        login::{
            local::local_login,
            oauth::{
                client::{
                    create_oauth_client, create_oauth_client_redirect_uri,
                    create_oauth_client_secret, delete_oauth_client_redirect_uri,
                    delete_oauth_client_secret, get_oauth_client, list_oauth_clients,
                },
                code::{authz_code_callback, authz_code_exchange, authz_code_redirect},
                device_token::{exchange_device_token, get_device_provider},
            },
        },
        mappers::{create_mapper, delete_mapper, get_mappers},
        rfd::{
            discuss_rfd, get_rfd, get_rfd_attr, get_rfds, publish_rfd, reserve_rfd, search_rfds,
            set_rfd_attr, set_rfd_content, set_rfd_document, update_rfd_visibility,
        },
        webhook::github_webhook,
        well_known::{jwks_json, openid_configuration},
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
    pub context: ApiContext,
    pub server_address: SocketAddr,
    pub spec_output: Option<SpecConfig>,
}

pub fn server(
    config: ServerConfig,
) -> Result<HttpServerStarter<ApiContext>, Box<dyn Error + Send + Sync>> {
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

    // .well-known
    api.register(openid_configuration)
        .expect("Failed to register endpoint");
    api.register(jwks_json)
        .expect("Failed to register endpoint");

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

    // User Management
    api.register(get_self).expect("Failed to register endpoint");
    api.register(get_api_user)
        .expect("Failed to register endpoint");
    api.register(create_api_user)
        .expect("Failed to register endpoint");
    api.register(update_api_user)
        .expect("Failed to register endpoint");
    api.register(list_api_user_tokens)
        .expect("Failed to register endpoint");
    api.register(get_api_user_token)
        .expect("Failed to register endpoint");
    api.register(create_api_user_token)
        .expect("Failed to register endpoint");
    api.register(delete_api_user_token)
        .expect("Failed to register endpoint");
    api.register(add_api_user_to_group)
        .expect("Failed to register endpoint");
    api.register(remove_api_user_from_group)
        .expect("Failed to register endpoint");

    // Group Management
    api.register(get_groups)
        .expect("Failed to register endpoint");
    api.register(create_group)
        .expect("Failed to register endpoint");
    api.register(update_group)
        .expect("Failed to register endpoint");
    api.register(delete_group)
        .expect("Failed to register endpoint");

    // Mapper Management
    api.register(get_mappers)
        .expect("Failed to register endpoint");
    api.register(create_mapper)
        .expect("Failed to register endpoint");
    api.register(delete_mapper)
        .expect("Failed to register endpoint");

    // OAuth Client Management
    api.register(list_oauth_clients)
        .expect("Failed to register endpoint");
    api.register(create_oauth_client)
        .expect("Failed to register endpoint");
    api.register(get_oauth_client)
        .expect("Failed to register endpoint");
    api.register(create_oauth_client_secret)
        .expect("Failed to register endpoint");
    api.register(delete_oauth_client_secret)
        .expect("Failed to register endpoint");
    api.register(create_oauth_client_redirect_uri)
        .expect("Failed to register endpoint");
    api.register(delete_oauth_client_redirect_uri)
        .expect("Failed to register endpoint");

    // OAuth Authorization Login
    api.register(authz_code_redirect)
        .expect("Failed to register endpoint");
    api.register(authz_code_callback)
        .expect("Failed to register endpoint");
    api.register(authz_code_exchange)
        .expect("Failed to register endpoint");

    // OAuth Device Login
    api.register(get_device_provider)
        .expect("Failed to register endpoint");
    api.register(exchange_device_token)
        .expect("Failed to register endpoint");

    // Development
    api.register(local_login)
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
