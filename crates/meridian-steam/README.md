# meridian-steam

Defines the Steam capabilities needed by Meridian Client.

`SteamClient` exposes typed game discovery rather than arbitrary endpoint access. `SteamSource`
retrieves its configured credential and adapts those results to the platform-neutral contract.
HTTP transport and authentication are not implemented yet.
