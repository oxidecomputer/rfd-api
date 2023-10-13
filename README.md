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
