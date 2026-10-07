# meridian-platform

Defines the common contract between platform integrations and local orchestration.

Platform support is explicit, typed, and limited to capabilities required by local orchestration.
Adapters keep credentials inside studio-controlled infrastructure, and concrete API behavior stays
isolated in platform-specific submodules.
