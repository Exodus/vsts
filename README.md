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

Examples:

Run with secret set up by environment variable:

`VSTS_JWT__SECRET="test123" ./vsts`
