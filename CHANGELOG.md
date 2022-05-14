<a name="unreleased"></a>
## [Unreleased]


<a name="v0.10.0"></a>
## [v0.10.0] - 2022-05-14
### Feat
- adding version in initial output. refactor: cleanup main function


<a name="v0.9.0"></a>
## [v0.9.0] - 2022-05-14
### Feat
- configurable duration chore: updated dependencies


<a name="v0.8.0"></a>
## [v0.8.0] - 2022-05-14
### Chore
- cache attempt

### Feat
- look for a settings file if there


<a name="v0.7.0"></a>
## [v0.7.0] - 2022-05-10
### Refactor
- improved config [#6](https://github.com/Exodus/vsts/issues/6) refactor: simplified logging [#4](https://github.com/Exodus/vsts/issues/4) chore: cargo fmt


<a name="v0.6.0"></a>
## [v0.6.0] - 2022-05-08
### Docs
- updating README.md, no openapi but it's something.

### Refactor
- rewrite using axum


<a name="v0.5.0"></a>
## [v0.5.0] - 2022-05-03
### Refactor
- use authorization headers refactor: use thiserror for derived errors docs: cleanup README


<a name="v0.4.1"></a>
## [v0.4.1] - 2022-05-03
### Chore
- adding tini for better Docker compatibility


<a name="v0.4.0"></a>
## [v0.4.0] - 2022-04-18
### Feat
- adding basic logging


<a name="v0.3.0"></a>
## [v0.3.0] - 2022-04-18
### Feat
- very basic health check response (200)


<a name="v0.2.1"></a>
## [v0.2.1] - 2022-04-18
### Fix
- initial path trimming chore: adding git-chglog and more conservative k6 values


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


[Unreleased]: https://github.com/Exodus/vsts/compare/v0.10.0...HEAD
[v0.10.0]: https://github.com/Exodus/vsts/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/Exodus/vsts/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/Exodus/vsts/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/Exodus/vsts/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/Exodus/vsts/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/Exodus/vsts/compare/v0.4.1...v0.5.0
[v0.4.1]: https://github.com/Exodus/vsts/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/Exodus/vsts/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/Exodus/vsts/compare/v0.2.1...v0.3.0
[v0.2.1]: https://github.com/Exodus/vsts/compare/v0.2.0...v0.2.1
