# VSTS - Very Simple Token Service

Very simple JWT token generator / validator. Created to use with Traefik ForwardAuth middleware since it hides behind Traefik (can be located internally). This service is of no use publicly exposed since the `/gen` endpoint would be accessable.

## API

---

### GET /gen

Generate a token in the response body.

---

### GET /auth

The /auth endpoint works expecting the following header:
`AUTHORIZATION: Bearer <token>` where <token> is a valid generated token from the `/gen` endpoint.

On a valid token, will respond with a status code 200 making it useful for something like Traefik ForwardAuth.

---

### GET /validate

Validates a token in the URL path after /validate: `/validate/<token>`

---

### GET /healthz

Very simple health check endpoint at: /healthz

Config file and Environment variable overrides

---

The server may be configured with a configuration file with a basename of `settings` and any of the following supported formats {json, yaml, toml, INI, RON, JSON5}

All of which can be overriden by environment variables:

VSTS_SERVER_PORT
VSTS_JWT_SECRET

Examples:

Run with secret set up by environment variable:

`VSTS_JWT_SECRET="test" VSTS_SERVER_PORT=3030 ./vsts`
