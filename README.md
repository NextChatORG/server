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
-   LLVM
-   [RustLang](https://www.rust-lang.org)
-   [PostgreSQL 9.6.20](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)

## Endpoints

Error response example (_400 Bad Request_):
```json
{
    "code": 0,
    "message": "Unknown"
}
```

### Version Checker
-   _PATCH_ `/version_checker?version={version}`

See [versions.md](./versions.md) for more information about the available versions.

Response examples:
```json
{
    "_url": "/version_checker?version=0.0.1",
    "required_update": true,
    "to": "1.0.0-alpha1"
}
```
```json
{
    "_url": "/version_checker?version=1.0.0-alpha1",
    "required_update": false,
    "to": null
}
```

### WebSockets
-   _WebSocket_ `/ws/?user_id={id}`

### Users
-   _GET_ `/users/all`

Error codes:
```
0 -> Cannot get the users from the table.
```

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

Error codes:
```
0 -> Username and id in the query: `/users/find?id={}&username={username}`
1 -> The user id does not exist.
```

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

Error codes:
```
0 -> Username and id in the query: `/users/find?id={}&username={username}`
1 -> The username does not exist.
```

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

Error codes:
```
0 -> Username is empty.
1 -> Username between 4 and 15 characteres.
2 -> Password is empty.
3 -> Password between 8 and 40 characteres.
4 -> Username already exists.
5 -> Unknown.
```

Body example:
```json
{
    "username": "danielsolartech",
    "password": "1234"
}
```

Response example:
```json
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
    "username": "danielsolartech",
    "profile_image": ""
}
```

-   _POST_ `/users/signin`

Error codes:
```
0 -> Username is empty.
1 -> Password is empty.
2 -> Username does not exist.
3 -> The password is incorrect.
```

Body example:
```json
{
    "username": "danielsolartech",
    "password": "1234"
}
```

Response example:
```json
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
    "username": "danielsolartech",
    "profile_image": ""
}
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
