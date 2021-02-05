# NextChat Server (API)
This repository contains the source code of the server application for NextChat written with Rustlang.

-   [Pre-requisites](#Pre-requisites)
-   [Endpoints](#Endpoints)
    -   [Users](#Users)
-   [Authors](#Authors)
-   [Changelog](#Changelog)
-   [Copyright](#Copyright)

## Pre-requisites
-   C++
-   [RustLang](https://www.rust-lang.org)
-   [PostgreSQL 9.6.20](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

## Endpoints

Error response example (_400 Bad Request_):
```json
{
    "message": "The message is here"
}
```

### WebSockets
-   _WebSocket_ `/ws/?user_id={id}`

### Users
-   _GET_ `/users/all`

Response example:
```json
[
    {
        "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
        "username": "danielsolartech",
        "password": null,
        "online": false,
        "last_online": "2021-02-02T18:27:08",
        "created_at": "2021-02-02T18:27:08"
    }
]
```

-   _GET_ `/users/find?id={id}`

Response example:
```json
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
    "username": "danielsolartech",
    "password": null,
    "online": false,
    "last_online": "2021-02-02T18:27:08",
    "created_at": "2021-02-02T18:27:08"
}
```

-   _GET_ `/users/find?username={username}`

Response example:
```json
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
    "username": "danielsolartech",
    "password": null,
    "online": false,
    "last_online": "2021-02-02T18:27:08",
    "created_at": "2021-02-02T18:27:08"
}
```

-   _POST_ `/users/signup`

Body example:
```json
{
    "username": "danielsolartech",
    "password": "1234"
}
```

Response example:
```
5959ad9c-598e-4deb-bcbe-053c1f73b400
```

-   _POST_ `/users/signin`

Body example:
```json
{
    "username": "danielsolartech",
    "password": "1234"
}
```

Response example:
```
5959ad9c-598e-4deb-bcbe-053c1f73b400
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
