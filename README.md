# NextChat Server (API)
-   [Pre-requisites](#Pre-requisites)
-   [Endpoints](#Endpoints)
    - [Users](#Users)
-   [Changelog](#Changelog)
-   [Copyright](#Copyright)

## Pre-requisites
-   [RustLang](https://www.rust-lang.org)

## Endpoints

Error response example (_400 Bad Request_):
```json
{
    "message": "The message is here"
}
```

### Users
-   _GET_ `/users/all`

Response example:
```json
[
    {
        "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
        "username": "danielsolartech",
        "password": "1234",
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
    "password": "1234",
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
    "password": "1234",
    "online": false,
    "last_online": "2021-02-02T18:27:08",
    "created_at": "2021-02-02T18:27:08"
}
```

-   _POST_ `/users/create`

Body example:
```
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400", // Optional
    "username": "danielsolartech", // Required
    "password": "1234" // Required
}
```

Response example:
```json
{
    "id": "5959ad9c-598e-4deb-bcbe-053c1f73b400",
    "username": "danielsolartech",
    "password": "1234",
    "online": false,
    "last_online": "2021-02-02T18:27:08",
    "created_at": "2021-02-02T18:27:08"
}
```

## Changelog
View the lastest repository changes in the [CHANGELOG.md](./CHANGELOG.md) file.

## Copyright
License: GPL-2.0

Read file [LICENSE](./LICENSE) for more information.
