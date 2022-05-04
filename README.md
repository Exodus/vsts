# VSTS - Very Simple Token Service

Very simple JWT token generator / validator

## Endpoints

### /gen

Will generate a token in the response body.

### /auth
The /auth endpoint works expecting the following header:
`AUTHORIZATION: Bearer <token>` where <token> is a valid generated token from the `/gen` endpoint.

On a valid token, will respond with a status code 200 making it useful for something like Traefik ForwardAuth.

### /validate

Validates a token in the URL path after /validate: `/validate/<token>`

### /healthz
Very simple health check endpoint at: /healthz

Config file and Environment variable overrides

Env Example:

`VSTS_JWT__SECRET="test123" ./vsts`
