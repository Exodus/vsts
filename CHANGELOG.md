<a name="unreleased"></a>
## [Unreleased]


<a name="v0.2.0"></a>
## v0.2.0 - 2022-04-18
### Chore
- arm64 build slows down amd64 build
- adding github action, k6 parameters
- remove Cargo.lock from gitignore

### Feat
- handle errors with malformed X-FORWARDED-Uri feat: added auth endpoint for traefik ForwardAuth chore: cleanup code chore: add k6 test
- pickup secret from config file/env
- better error handling feat: initial config file support

### Fix
- serve 0.0.0.0 & workaround for docker config file & k6 test
- validate fn uses CONFIG as well.

### Refactor
- json messages, cleaner with modules


[Unreleased]: https://github.com/Exodus/vsts/compare/v0.2.0...HEAD
