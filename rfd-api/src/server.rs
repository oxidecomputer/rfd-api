use dropshot::{ApiDescription, ConfigDropshot, EndpointTagPolicy, HttpServerStarter, TagConfig};
use serde::Deserialize;
use slog::Drain;
use slog_tracing_bridge::BridgeDrain;
use std::{error::Error, fs::File, net::SocketAddr, path::PathBuf};

use crate::{
    context::ApiContext,
    endpoints::{
        api_user::{
            create_api_user, create_api_user_token, delete_api_user_token, get_api_user,
            get_api_user_token, get_self, list_api_user_tokens, update_api_user,
        },
        login::{
            access_token::access_token_login,
            jwt::jwt_login,
            oauth::{exchange_device_token, get_device_provider},
        },
        rfd::get_rfd,
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
    pub context: ApiContext,
    pub server_address: SocketAddr,
    pub spec_output: Option<SpecConfig>,
}

pub fn server(
    config: ServerConfig,
) -> Result<HttpServerStarter<ApiContext>, Box<dyn Error + Send + Sync>> {
    let mut config_dropshot = ConfigDropshot::default();
    config_dropshot.bind_address = config.server_address;

    // Construct a shim to pipe dropshot logs into the global tracing logger
    let dropshot_logger = {
        let level_drain = slog::LevelFilter(BridgeDrain, slog::Level::Debug).fuse();
        let async_drain = slog_async::Async::new(level_drain).build().fuse();
        slog::Logger::root(async_drain, slog::o!())
    };

    let mut api = ApiDescription::new().tag_config(TagConfig {
        allow_other_tags: false,
        endpoint_tag_policy: EndpointTagPolicy::Any,
        tag_definitions: vec![].into_iter().collect(),
    });

    // RFDs
    api.register(get_rfd).unwrap();

    // Webhooks
    api.register(github_webhook).unwrap();

    // User Management
    api.register(get_self).unwrap();
    api.register(get_api_user).unwrap();
    api.register(create_api_user).unwrap();
    api.register(update_api_user).unwrap();
    api.register(list_api_user_tokens).unwrap();
    api.register(get_api_user_token).unwrap();
    api.register(create_api_user_token).unwrap();
    api.register(delete_api_user_token).unwrap();

    // Access Token Login
    api.register(access_token_login).unwrap();

    // JWT Login
    api.register(jwt_login).unwrap();

    // OAuth Device Login
    api.register(get_device_provider).unwrap();
    api.register(exchange_device_token).unwrap();

    if let Some(spec) = config.spec_output {
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
