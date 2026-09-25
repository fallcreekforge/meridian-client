# meridian-credential-store

Defines credential access within studio-controlled infrastructure.

`CredentialStore` retrieves named credentials without assuming a platform or backend. `SecretString`
redacts its `Debug` output, zeroizes its allocation when dropped, and requires explicit access to
the underlying value. `FileCredentialStore` deserializes credentials from customer-controlled
configuration without supporting serialization of those secret values.
