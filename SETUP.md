# rfd-api

Backend services and tools for processing and managing RFDs

## Setup

To start with there are few dependencies for running the RFD API. Notably the RFD API is broken up
in to two applications, the `rfd-api` and the `rfd-processor`. See [README.md] for more information
on the distinctions.

### Dependencies

General
 * Rust 1.77
 * Postgres 14

Processor
 * Asciidoctor 2.0.16
 * Node 20.11.0
 * NPM Packages:
 * Ruby 3.0.2
 * Ruby Gems:
   * asciidoctor-mermaid 0.4.1
   * asciidoctor-pdf 2.3.10
   * mmdc 10.6.1
   * rouge 4.2.0

Optional
 * Meilisearch (if search is to be supported)

### Build

Run `cargo build --release` to build all of the project binaries. Two binaries will be generated and
emitted at:

* `target/release/rfd-api`
* `target/release/rfd-processor`

### Installation

Once all of the dependencies have been installed, database migrations will need to be run to prepare
the database tables. These can be run using the `rfd-installer` tool:

```sh
cd rfd-model
V_ONLY=1 DATABASE_URL=<database-url> cargo run -p rfd-installer
DATABASE_URL=<database-url> diesel migration run
```

Replace `<database-url>` with the url to the Postgres instance that the API and processor will be
configured to run against.

### Running database migrations

After the initial migration described above, any future database migration can
be executed with the following commands:

```sh
cd rfd-model
DATABASE_URL=<database-url> diesel migration run
```

> [!NOTE]
>
> If the generated `schema.rs` includes additional tables in its diff, it means
> v-api added more tables of its own. You should exclude them in
> `rfd-model/diesel.toml` and re-run migrations. The extraneous tables should
> then disappear from `schema.rs`.

### Configuration

Each part (the api and processor) has its own configuration file, and the respective application
directories have example files called `config.example.toml`. Where possible the define either
default values, or disabled options.

#### API

The two sections in the API configuration to pay attention to are the `[[keys]]` and `[[authn]]`
configurations.

`[[keys]]` define the key pairs used to sign API keys and session tokens. Two sources are supported
for keys, either local or GCP Cloud KMS. Placeholder configurations are provided for both sources as
examples. During local development it is recommended to generate a new RSA key pair locally for use.

`[[authn]]` is a list of authentication providers that should be enabled. Google and GitHub are the
only authentication providers supported currently and placeholders are available in the API example
configuration.

For local development remote authentication can be bypassed by using the `local-dev` feature.
Instead of using a remote authentication provider, the API can be run via:

```sh
cargo run -p rfd-api --features local-dev
```

This will run the API with a [`POST /login/local`](rfd-api/src/endpoints/login/local/mod.rs) endpoint
exposed. This endpoint allows for logging in with an arbitrary email and user supplied unique
identifier. To use this with the CLI, the `local-dev` feature will need to be passed to the CLI
build.

```sh
cargo run -p rfd-cli --features local-dev
```

#### Processor

The processor has multiple jobs that are able to be run, and configuration is only required for
jobs that are going to be run. The `actions` key defines the jobs that should be run. By default
all jobs are disabled. In this this mode the processor will only construct a database of RFDs.

##### Static Asset Storage

The processor can copy images and other static assets extracted from RFDs to cloud storage. Both
Google Cloud Storage (GCS) and Amazon S3 (or S3-compatible services) are supported. Multiple
storage backends can be configured simultaneously, and assets will be pushed to all of them.

To enable this feature, add `CopyImagesToStorage` to the `actions` list and configure at least one
storage backend.

**Google Cloud Storage (GCS)**

```toml
[[gcs_storage]]
bucket = "your-bucket-name"
```

GCS uses GCP Application Default Credentials for authentication. Configure credentials using one of:
- `GOOGLE_APPLICATION_CREDENTIALS` environment variable pointing to a service account key file
- Instance metadata (when running on GCP Compute Engine, GKE, etc.)
- `gcloud auth application-default login` (for local development)

**Amazon S3**

```toml
[[s3_storage]]
bucket = "your-bucket-name"
region = "us-west-2"
# Optional: custom endpoint for S3-compatible services
# endpoint = "https://s3.example.com"
```

S3 uses the AWS SDK default credential chain for authentication:
- Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, optionally `AWS_SESSION_TOKEN`)
- Shared credentials file (`~/.aws/credentials`)
- IAM role (when running on AWS EC2, ECS, Lambda, etc.)

The optional `endpoint` field allows using S3-compatible services such as MinIO, Backblaze B2, or
Cloudflare R2.
