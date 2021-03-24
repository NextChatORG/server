# NextChat Server (API)
This repository contains the source code of the server application for NextChat written with Rustlang.

-   [Pre-requisites](#Pre-requisites)
-   [Endpoints](#Endpoints)
-   [Authors](#Authors)
-   [Changelog](#Changelog)
-   [Copyright](#Copyright)

## Pre-requisites
-   C++
-   [LLVM](https://llvm.org/)
-   [RustLang](https://www.rust-lang.org)
-   [PostgreSQL 9.6.20](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

## Endpoints
Read file [endpoints.md](./endpoints.md) for mroe information.

## Testing

### All features
```sh
cargo test --workspace --all-features
```

### Without panic tests
```sh
cargo test --workspace
```

## Authors
-   [@danielsolartech](https://github.com/danielsolartech) - Initial project
-   [@JheysonDev](https://github.com/JheysonDev) - Icon Design
-   [@TeoDev1611](https://github.com/TeoDev1611) - Icon Design

## Changelog
View the lastest repository changes in the [CHANGELOG.md](./CHANGELOG.md) file.

## Copyright
License: GPL-2.0

Read file [LICENSE](./LICENSE) for more information.
