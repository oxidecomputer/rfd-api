# rfd-api

Work in progress replacement for RFD processing and programmatic access.

## Authentication

Rough sketch of how users can authenticate to the RFD API

```
     ┌─────────┐                     ┌─────────┐
     │ Google  │                     │ GitHub  │
     └────┬────┘                     └──┬───┬──┘
          │                     ┌───────┘   └───────────┐
┌─────────┴─────────┐ ┌─────────┴─────────┐ ┌───────────┴───────────┐
│ OIDC Access Token │ │ App Access Token  │ │ Personal Access Token │
└─────────┬─────────┘ └─────────┬─────────┘ └───────────┬───────────┘
          │                     │                       │
┌─────────┴─────────┐       ┌───┴───────────────────────┴───┐
│ /login/jwt/google │       │  /login/access-token/github   │
└─────────┬─────────┘       └───────────────┬───────────────┘
          └─────────────┐   ┌───────────────┘
              ┌─────────┴───┴─────────┐
              │ Create/Fetch API User │
              └───────────┬───────────┘
                          │
             ┌────────────┴────────────┐
             │ Create New Access Token │
             └─────────────────────────┘
```