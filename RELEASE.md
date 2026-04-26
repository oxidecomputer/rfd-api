# Releasing

Releases are triggered by pushing a version tag. The tag format determines what
gets released:

- `rfd-cli/1.2.3` — releases only `rfd-cli` at version `1.2.3`
- `1.2.3` — releases all dist-able packages at that version

The release workflow uses the [Oxide fork of cargo-dist][cargo-dist] to build
and publish artifacts. macOS binaries are code-signed and notarized as part of
the build.

## Required GitHub Actions secrets

### macOS code signing and notarization

| Secret                           | Description                                                                                          |
| -------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `CODESIGN_CERTIFICATE`           | Base64-encoded `.p12` certificate file (Developer ID Application certificate exported from Keychain) |
| `CODESIGN_CERTIFICATE_PASSWORD`  | Password used when exporting the `.p12` certificate                                                  |
| `CODESIGN_IDENTITY`              | Signing identity string, e.g. `Developer ID Application: Oxide Computer Company (XXXXXXXXXX)`        |
| `CODESIGN_NOTARIZATION_APPLE_ID` | Apple ID used to submit binaries for notarization                                                    |
| `CODESIGN_NOTARIZATION_PASSWORD` | App-specific password for the Apple ID (generated at appleid.apple.com)                              |

To base64-encode the certificate for `CODESIGN_CERTIFICATE`:

```sh
base64 -i certificate.p12 | pbcopy
```

[cargo-dist]: https://github.com/oxidecomputer/cargo-dist
