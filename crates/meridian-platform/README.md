# meridian-platform

Defines the common contract between platform integrations and local orchestration.

`PlatformClient` discovers typed games using studio-controlled credentials. `Platform` remains an
explicit enum so support for each new platform is deliberate and reviewable. Concrete API behavior
belongs in platform submodules such as `steam`.
