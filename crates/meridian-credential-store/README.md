# meridian-credential-store

Defines credential access within studio-controlled infrastructure.

Credential access remains independent of platforms and storage backends. Secret values are
redacted from ordinary output, explicitly accessed, and never serializable through credential
configuration types.

The crate re-exports `SecretString` and `ExposeSecret` from `secrecy`, and `JsonFileError` from
`meridian-utils`, under their original names. `FileCredentialStore::load_from_file` returns that
shared file error; `CredentialStore::get` borrows an already-loaded secret.
