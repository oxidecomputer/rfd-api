# rfd-api

Work in progress replacement for RFD processing and programmatic access.

## Granting access to external users

## RFD Model

### Revisions

## RFD Processing

### Triggers

### Periodic Schedule

## Authentication

### Accounts and Providers

#### Supported Providers

GitHub -
Google -

### Account Linking

### OAuth2 Authorization Code

### OAuth2 Device Code

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
