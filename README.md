# rfd-api

Backend services and tools for processing and managing RFDs

## RFD CLI

### Getting Started

1. Download the latest release of `rfd-cli` or run `cargo run -p rfd-cli`
2. Configure the API host with `rfd-cli config set host https://rfd-api.shared.oxide.computer`

Choose an authentication mode based on the kind of session you want, either a short-term session
token (id) or a long-term api token (token).

3. Authenticate against the API with `rfd-cli auth login google` via a session

**OR**

3. Authenticate against the API with `rfd-cli auth login google -m token` via a token

## Backend

The RFD API backend is made up of two services:
* `rfd-api` - API for accessing RFDs and handling GitHub webhooks
* `rfd-processor` - Scans for RFDs to update, handles RFD state transitions, manages RFD assets

The RFD API backend services expect to run against a Postgres database.

### API

Running the API requires setting up a configuration file as outlined in `config.example.toml`.

### Processor

Dependencies

* asciidoctor
* Node
  * @mermaid-js/mermaid-cli
* Ruby
  * rouge
  * asciidoctor-pdf
  * asciidoctor-mermaid

## Background

Objects reference:
```
                                    ┌─────────────────┐ ┌─────────────────┐
                                    │                 │ │                 │
                ┌─────────────────┐ │ PDF cccccc Src1 │ │ PDF cccccc Src2 │
                │                 │ │                 │ │                 │
                │ PDF aaaaaa Src1 │ └────────┬────────┘ └────────┬────────┘
                │                 │          │                   │
                └────────┬────────┘          ├───────────────────┘
                         │                   │
                ┌────────┴────────┐ ┌────────┴────────┐
                │                 │ │                 │
                │ Revision aaaaaa │ │ Revision cccccc │
                ├─────────────────┤ ├─────────────────┤
              ┌─┤ commit_sha      │ │ commit_sha      │
              │ └────────┬────────┘ └────────┬────────┘
              │          │                   │
              │          └───────────────────┤
┌─────────┐   │                              │
│         │   └─────────────────┐            │
│ Scanner │                     │            |
├─────────┤     ┌────────────┐  │   ┌────────┴────────┐
│ sha     ├───┐ │            │  │   |                 |
├─────────┤   │ │ Job 1      │  │   │  RFD 123        │
│ branch* ├─┐ │ ├────────────┤  │   ├─────────────────┤
└─────────┘ │ ├─┤ sha        ├──┘   │ id              │
            │ │ ├────────────┤      ├─────────────────┤
┌─────────┐ ├─┼─┤ rfd        ├──────┤ rfd_number      │
│         │ │ │ └────────────┘      └─────────────────┘
│ Webhook │ │ │
├─────────┤ │ │
│ sha     ├─┼─┘
├─────────┤ │
│ branch* ├─┘
└─────────┘
```
Note: Scanner and Webhook operations that occur on the default branch do not use the branch name for
determining the RFD number to update. Instead they use the numeric portion of the 
`rfd/{number}/README.adoc` path.

### Revisions

Every revision is tied to a commit* against an RFD readme file. There is no guarantee though that
there exists a revision though for every commit. While the RFD API will attempt to create a revision
for every commit, outages, missing webhooks, or internal errors can result in missing revisions.
Currently the background periodic processor does not attempt to backfill missing revisions, it only
ensures that there is a revision for the latest commit it sees during its run.

Note: Force pushes may result in the removal of the commit that triggered a revision.

## RFD Processing

The RFD processors primary purpose is the implement and maintain the specifications defined in RFD 1.
As internal needs have grown though, so has the processor. Each step of the processor is implemented
as a separate action. Currently the supported actions are:

| Action                 | Purpose |
|------------------------|------------
| copy_images_to_storage | Copies images and static files associated with an RFD to cloud storage
| create_pull_request    | Create a PR for the RFD if it does not have one and the RFD is in discussion
| ensure_default_state   | Checks that RFDs on the default branch have appropriate states
| ensure_pr_state        | Updates the state attribute for RFDs not on the default branch as needed
| update_discussion_url  | Updates the discussion url attribute in the RFD contents
| update_pdfs            | Create and upload a PDF version of the RFD revision
| update_pull_request    | Update pull request titles and labels so they align with the RFD content
| update_search_index    | Update the RFD search index with the new RFD contents

### Content Updates

RFD processing manipulates both internally stored state as well as the source content document of
the RFD it is processing. The two cases where the processor will update the contents of an RFD are:

1. An RFD has an incorrect discussion url
2. An RFD is in an incorrect state

The first update is the easier of the two. For any RFD that has an open discussion PR, the processor
will check that the `discussion` attribute in the RFD document matches the url of the discussion PR.
Note though that there is a bug here currently related to the order in which revisions may be processed.

State checking is a bit more complex. For an RFD that has an open discussion PR, the processor will
ensure that the RFD state is set to `discussion`. For RFDs that are merged to the default branch
though, there is not a good determination as to which of the final states to assign them. Instead
the processor will emit a warning when it encounters such a case.

### Update Runners

RFD updates occur via two mechanism. The first of which is in response to GitHub webhook calls for
pushes against the RFD repo. The RFDs that are updated in response to a webhook depend on the branch
that was updated and the contents of the commit. RFDs are also updated via a periodic processor so
that the processor can account for webhook calls that were either missed, dropped, or failed due to
some internal error.

### Webhooks

Webhook calls are accepted by the `rfd-api` server which validates the call and determines the RFDs
to update. Pushes to the default branch will allow for updates to occur to any RFD number. So if a
commit contains an update to RFD 1, RFD 2, and RFD 3, then three update jobs will be scheduled. In
contrast to this, if the commit is made against a specific branch (i.e. 0123) then a job will only
be scheduled if a change is made to RFD 123.

Note that the `rfd-api` server does not perform RFD updates. It is responsible only for validating
calls and scheduling update jobs. Once scheduled, the job will be processed by the `rfd-processor`.

### Scanner

The scanner can be run at a configurable interval which is largely dependent on the size of the RFD
repo itself, and GitHub rate limits. Currently we run the scanner on a 15 minute interval.

## Authentication

### Accounts and Providers

```
              ┌───────────────────┐
              │                   │
              │ api_user_provider │
              ├───────────────────┤
              │ id                │
┌────────┐    ├───────────────────┤
│ Google ├────┤ remote_id         │   ┌─────────────────┐
└────────┘    ├───────────────────┤   │                 │
              │ api_user          ├─┐ │ api_user        │
              └───────────────────┘ │ ├─────────────────┤
                                    ├─┤ id              │
              ┌───────────────────┐ │ └─────────────────┘
              │                   │ │
              │ api_user_provider │ │
              ├───────────────────┤ │
              │ id                │ │
┌────────┐    ├───────────────────┤ │
│ GitHub ├────┤ remote_id         │ │
└────────┘    ├───────────────────┤ │
              │ api_user          ├─┘
              └───────────────────┘
```

### Account Provider Linking

(Note: not yet implemented)

The RFD API does not perform any kind of automatic account linking. Every new remote provider id that
is seen results in a new account being generated. This is problematic though if you want to be able to
log in to your account via multiple remote accounts. To support this, providers can be moved between
accounts.

Transfers are performed manually and require access to both the source account that currently owns the
provider and the target account. To initiate a transfer, the source account will make a call to
`/api-user-provider/{identifier}/link-token` where `identifier` is the id of the provider to transfer.
This endpoint will return a token that can be used to move the provider to a new account.

The target account then needs to call `/api-user/{identifier}/link` with the generated token to link
the provider to their account, where `identifier` is the id of the account to link the provider to.
Note that only the owner of an account can link new providers to it. As such it is only valid to call
this endpoint with an `identifier == caller_identifier`.

### OAuth2

Users can authenticate to the RFD API via OAuth2. The RFD APIs OAuth implementation is backed by remote
providers. Currently two providers are supported (both of which are OAuth2 providers themselves): GitHub
and Google. Authenticating against the RFD API will return an access token that is valid for TBD.
Refresh tokens are not supported by the RFD API. For application contexts where longer term access is
required, fine-grained API tokens should be used instead.

### OAuth2 Scopes

|Scope               | Description                                      |
|--------------------|--------------------------------------------------|
| user:info:r        | Read information about users and their providers |
| user:info:w        | Update information about users                   |
| user:provider:w    | Update user providers                            |
| user:token:r       | Read API token information for users             |
| user:token:w       | Create API tokens for users                      |
| group:r            | Read group information                           |
| group:w            | Create, update, and delete groups                |
| group:membership:w | Add and remove users from groups                 |
| rfd:content:r      | List and fetch RFDs                              |
| rfd:discussion:r   | Fetch RFD discussions                            |
| search             | Search for RFDs                                  |
| oauth:client:r     | List OAuth clients                               |
| oauth:client:w     | Create and update OAuth clients                  |

### OAuth2 Authorization Code

The RFD API supports the OAuth2 authorization code flow via two remote providers: GitHub and Google.
A caller can choose which remote provider to send a user to by using the corresponding endpoint:

`/login/oauth/github/code/authorize` - Authenticate with GitHub

`/login/oauth/google/code/authorize` - Authenticate with Google

### OAuth2 Device Code

To an OAuth2 client, the device flow appears as a spec compliant device flow. Internally the RFD API
defers to device and user code creation and validation to the remote provider. It then exposes a
custom token endpoint that proxies requests from the client to the appropriate remote provider.
Once a remote provider responds successfully with an access token, the RFD API will perform its
internal account lookup / creation logic to find a matching RFD API user account. The remote access
token is then thrown away and an access token for the RFD API is returned.

As with the authorization code flow, the RFD API does not provide refresh tokens.

Example of device flow with the Google provider

```
Browser                Client                    RFD API                        Google
   │                    │                           │                             │
   │                    │    Request oauth config   │                             │
   │                    ├──────────────────────────►│                             │
   │                    │◄──────────────────────────┤                             │
   │                    │     Return with custom    │                             │
   │                    │       token endpoint      │                             │
   │                    │                           │                             │
   │                    │    Device authz request   │                             │
   │                    ├───────────────────────────┼────────────────────────────►│
   │  Authenticate with │◄──────────────────────────┼─────────────────────────────┤
   │  Google and enter  │    Return device_code,    │                             │
   │      user_code     │      user_code, etc       │                             │
   │◄───────────────────┤                           │                             │
   │                    │                           │                             │
   │                    │    Poll token endpoint    │                             │
   │                    ├──────────────────────────►│                             │
   │                    │        device_code        │      Proxied token call     │
   │                    │             .             ├────────────────────────────►│
   │                    │             .             │◄────────────────────────────┤
   │                    │             .             │     Return access token     │
   │                    │◄──────────────────────────┤                             │
   │                    │     Failure response:     │                             │
   │                    │     Authn not complete    │                             │
   ├───────────────────►│                           │                             │
   │   Complete authn   │    Poll token endpoint    │                             │
   │                    ├──────────────────────────►│                             │
   │                    │        device_code        │      Proxied token call     │
   │                    │                           ├────────────────────────────►│
   │                    │                           │◄────────────────────────────┤
   │                    │                           │     Return access token     │
   │                    │◄──────────────────────────┤                             │
   │                    │    Use access token to    │                             │
   │                    │    fetch user info and    │                             │
   │                    │    perform authn based    │                             │
   │                    │    on remote user id      │                             │
   │                    │    into the RFD API.      │                             │
   │                    │    Return RFD API token   │                             │
   │                    │                           │                             │
   │                    │                           │                             │
```

## Authorization

### Permissions

Permissions can be assigned to both users and groups (see below). Permissions are always additive,
and a callers full permissions are the combined set of their directly assigned permissions and their
group permissions.

[Api Permissions](rfd-api/src/permissions.rs)

### Groups

Groups are a way to manage sets of permissions and assigned them to one or more users. Permissions
from multiple groups are always additive. Users can be assigned to any number of groups, and group
assignments are stored on user records. Sub-groups are not supported.

```
                  ┌──────────────┐
                  │              │
                  │ access_group │
                  ├──────────────┤
┌─────────────┐ ┌─┤ id           │
│             │ │ ├──────────────┤
│ api_user    │ │ │ permissions  │
├─────────────┤ │ └──────────────┘
│ permissions │ │
├─────────────┤ │ ┌──────────────┐
│ groups      ├─┤ │              │
└─────────────┘ │ │ access_group │
                │ ├──────────────┤
                └─┤ id           │
                  ├──────────────┤
                  │ permissions  │
                  └──────────────┘
```

### Mappers

By default, new accounts do not have any permissions. The only thing they can do is login. Mappers
can be used to assign default permissions to accounts immediately upon login. Mappers apply to both
existing and new accounts. Mappers are currently only additive. They can assign permissions and
groups, but they can not remove them. Mappers that remove assignments may be supported in the future.

A mapper contains a `condition` and a set of values to apply. The `condition` is tested against the
user information returned from a remote provider, if it returns true then the values are applied to
the user account associated with the user provider.

Notably this means that mappers explicitly only run when a user authenticates via a remote provider.

A `mappers.toml` file can be used to configure mappers that should be installed during startup of
the RFD API.

#### Supported Mappers

**Email Address** - Maps from a fully specified email address to a list of permissions and/or list
of groups. This mapper can be used with GitHub or Google.

```toml
[[mappers]]
name = "Initial admin"
rule = "email_address"
email = "user@domain.com"
groups = [
  "admin"
]
```

```bash
cargo run -p rfd-cli mapper create --json-body /dev/stdin <<EOM
{
  "name": "add_email_address",
  "max_activations": 1,
  "rule": {
    "rule": "email_address",
    "email": "user@domain.com",
    "groups": [
      "admin"
    ]
  }
}
EOM
```

**Email Domain** - Maps from a email domain to a list of permissions and/or list of groups. This
mapper can be
used with GitHub or Google.

```toml
[[mappers]]
name = "Employees"
rule = "email_domain"
domain = "domain.com"
groups = [
  "company-employee"
]
```

```bash
cargo run -p rfd-cli mapper create --json-body /dev/stdin <<EOM
{
  "name": "add_email_domain",
  "max_activations": 5,
  "rule": {
    "rule": "email_domain",
    "domain": "domain.com",
    "groups": [
      "company-employee"
    ]
  }
}
EOM
```

**GitHub Username** - Maps from a GitHub username to a list of permissions and/or list of groups.
As expected, this mapper can only succeed with a GitHub provider.

```toml
[[mappers]]
name = "Friend"
rule = "github_username"
domain = "githubuser"
groups = [
  "friend-of-company"
]
```

```bash
cargo run -p rfd-cli mapper create --json-body /dev/stdin <<EOM
{
  "name": "add_github_user",
  "max_activations": 1,
  "rule": {
    "rule": "github_username",
    "github_username": "githubuser",
    "groups": [
      "friend-of-company"
    ]
  }
}
EOM
```

## Contributing

This repo is public because others are interested in the RFD process and the tooling we've
built around it. In its present state, it's the code we're using as the backend to
[our RFD frontend](https://rfd.shared.oxide.computer/). We're open to PRs that
improve these services, especially if they make the repo easier for others to use and contribute
to. However, we are a small company, and the primary goal of this repo is as an internal
tool for Oxide, so we can't guarantee that PRs will be integrated.

## License

Unless otherwise noted, all components are licensed under the
[Mozilla Public License Version 2.0](LICENSE).
