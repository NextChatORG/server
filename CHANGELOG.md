# Changelog

## Unreleased (0.1.0-alpha1)

### 23/03/2021
-   Add unit tests.
-   Add `async-trait` for incoming events (NextChat Communication).

### 22/03/2021
-   Add NextChat Communication module.
-   Add a new modular structure.

### 20/03/2021
-   Code refactor to a scalable structure.
-   Add code documentation.
-   Add `colored` and `anyhow` crates.

### 10/03/2021
-   Add `/friends/get-of/:id` endpoint.
-   Fix `length` bug in `API_HOST`.
-   Add `friends` sql and `/friends/:id/:id` endpoint.
-   Create `endpoints.md` file.

### 07/03/2021
-   Add `/search` endpoint.
-   Add `take` and `skip` query params to `/users/all` route.
-   Add `/version_checker` endpoint.
-   Change `ResponseBody` struct.
-   Remove password from `UserModel` JSON serialize.
-   Remove `/v1/` from routes.

### 04/03/2021
-   Update `online` and `last_online` when a user is connected or disconnected.
-   Add `Storage` struct.
-   Change everything to Tokio and Warp.

### 12/02/2021
-   (WebSockets) Add close connection message.

### 07/02/2021
-   Add error codes to user endpoints.
-   Change response of `/users/signin` and `/users/signup`.

### 05/02/2021
-   Fix `/users/all` endpoint.

### 04/02/2021
-   Add `profile_image` column to users table.
-   Fix `signup` user data from body.
-   Change `/users/create` to `/users/signup`.
-   Add `/users/signin` endpoint.
-   Add password encryption with argon2.

### 02/02/2021
-   Add simple websocket server.
-   Add `Connection` struct.
-   Change find user routes to query url.
-   Add users endpoints and structs.
-   Add database connection with SQLx and PostgreSQL.
-   Add HTTP server with actix-web.
