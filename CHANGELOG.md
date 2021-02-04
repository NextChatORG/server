# Changelog

## Unreleased

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
