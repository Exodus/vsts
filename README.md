# VSTS - Very Simple Token Service

Very simple JWT token generator / validator. Created to use with Traefik ForwardAuth middleware since it hides behind Traefik (can be located internally). This service is of no use publicly exposed since the `/gen` endpoint would be accessable.

## API

---

> ### GET /gen

Generate a token in the response body.

---

> ### GET /auth

The /auth endpoint works in 3 ways.

Expecting the following header:
`TOKEN: <token>` where `<token>` is a valid generated token from the `/gen` endpoint.

Expecting a query param: `/auth?token=<token>`.

Expecting a token in the path: `/auth/<token>`.

On a valid token, the service will respond with a status code 200 making it useful for something like Traefik ForwardAuth.

---

> ### GET /healthz

Very simple health check endpoint at: `/healthz`.

Config file and Environment variable overrides.

---

The server may be configured with a configuration file with a basename of `settings` and any of the following supported formats {json, yaml, toml, INI, RON, JSON5}.

All of which can be overriden by environment variables:

- VSTS_SERVER_PORT
- VSTS_JWT_SECRET
- VSTS_JWT_DURATION

Examples:

Run with secret set up by environment variable:

`VSTS_JWT_SECRET="test" VSTS_SERVER_PORT=3030 VSTS_JWT_DURATION="3d 4h 10s" ./vsts`
