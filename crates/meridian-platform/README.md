# meridian-platform

Defines the common contract between platform integrations and local orchestration.

Platform support is explicit, typed, and limited to capabilities required by local orchestration.
Adapters consume the foundation types owned by `meridian-store`, keep credentials inside
studio-controlled infrastructure, and isolate concrete API behavior in platform-specific
submodules.
