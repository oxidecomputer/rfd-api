# rfd-api

Work in progress replacement for RFD processing and programmatic access.

## Granting access to external users

## RFD Model

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
              ┌─┤ sha             │ │ sha             │
              │ └────────┬────────┘ └────────┬────────┘
              │          │                   │
              │          └───────────────────┤
┌─────────┐   │                              │
│         │   └─────────────────┐            │
│ Scanner │                     │   ┌────────┴────────┐
├─────────┤     ┌────────────┐  │   │                 │
│ sha     ├───┐ │            │  │   │  RFD 123        │
├─────────┤   │ │ Job 1      │  │   │                 │
│ branch* ├─┐ │ ├────────────┤  │   ├─────────────────┤
└─────────┘ │ ├─┤ sha        ├──┘   │ id              │
            │ │ ├────────────┤      ├─────────────────┤
┌─────────┐ ├─┼─┤ rfd_number ├──────┤ rfd_number      │
│         │ │ │ └────────────┘      └─────────────────┘
│ Webhook │ │ │
├─────────┤ │ │
│ sha     ├─┼─┘
├─────────┤ │
│ branch* ├─┘
└─────────┘
```
* Scanner and Webhook operations that occur on the default branch do not use the branch name for
determining the RFD number to update. Instead they use the numeric portion of the `rfd/{number}/README.adoc` path.

### Revisions

## RFD Processing

### Triggers

### Periodic Schedule

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
providers. Currently two providers are supported (both of which are OAuth2 providers themselves): GitHub and Google. Authenticating against the RFD API will return an access token that is valid for TBD. Refresh tokens are not supported by the RFD API. For application contexts where longer term access is required, fine-grained API tokens should be used instead.

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

The RFD API supports the OAuth2 authorization code flow via two remote providers: GitHub and Google. A caller can choose which remote provider to send a user to by using the corresponding endpoint:

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
   │                    │    on verified emails     │                             │
   │                    │    into the RFD API.      │                             │
   │                    │    Return RFD API token   │                             │
   │                    │                           │                             │
   │                    │                           │                             │
```

## Authorization

### Permissions

### Groups

### Mappers

#### Supported Mappers

Email Address - 
Email Domain - 
GitHub Username - 
