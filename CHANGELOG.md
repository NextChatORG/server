# Changelog

## Unreleased (MVP)

## 07/03/2021
-   Add `version_checker` route.
-   Change `ResponseBody` struct.
-   Remove password from `UserModel` JSON serialize.
-   Remove `/v1/` from routes.

## 04/03/2021
-   Update `online` and `last_online` when a user is connected or disconnected.
-   Add `Storage` struct.
-   Change everything to Tokio and Warp.

## 12/02/2021
-   (WebSockets) Add close connection message.

## 07/02/2021
-   Add error codes to user endpoints.
-   Change response of `/users/signin` and `/users/signup`.

## 05/02/2021
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
