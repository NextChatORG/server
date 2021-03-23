//! NextChat Server users service module.
//!
//! This module contains the handlers of the users controller routes:
//!
//! `/users/all`                        -> get_all_handler
//! `/users/search/:text_to_search`     -> search_handler
//! `/users/find?id={user_id}`          -> find_handler
//! `/users/find?username={username}`   -> find_handler
//! `/users/signup`                     -> signup_handler
//! `/users/signin`                     -> signin_handler

use std::convert::Infallible;

use nextchat_database::{models::users::*, Client, Row, Uuid};
use nextchat_security::{encrypt_password, verify_password};
use warp::Reply;

use crate::response::{Error, Response};

/// `/users/all` handler
///
/// # Request query
/// - `?take={number}` _Default_ 10
/// - `?skip={number}` _Default_ 0
///
/// ## Example
/// - `?skip=10&take=10` Get all users from 10 to 20.
///
/// # Response
/// ```json
/// {
///     "id": "86df7b6c-2377-4cd6-ac1c-badfef243f3b",
///     "username": "NextChat",
///     "profile_image": "url.png"
/// }
/// ```
///
/// ## Status codes
/// - `204` - When the search is successful but does not return anything.
/// - `200` - When the search is successful and returns one or more results.
///
/// ## Errors
/// 1. Cannot get the users.
pub async fn get_all_handler(query: GetAllQuery, client: Client) -> Result<impl Reply, Infallible> {
    match nextchat_database::query(
        "SELECT id, username, profile_image FROM users LIMIT $1 OFFSET $2",
    )
    .bind(query.take.unwrap_or(10))
    .bind(query.skip.unwrap_or(0))
    .fetch_all(&client)
    .await
    {
        Err(_) => Ok(Error::from_str("Cannot get the users.")
            .to_response(400)
            .to_reply()),
        Ok(users) => {
            let users: Vec<UserDataResponse> = users
                .iter()
                .map(|user| UserDataResponse::from_row(user))
                .collect();
            Ok(Response::new(if users.is_empty() { 204 } else { 200 }, users).to_reply())
        }
    }
}

/// `/users/search/:text_to_search` handler.
///
/// # Request query
/// - `?take={number}` _Default_ 10
/// - `?skip={number}` _Default_ 0
///
/// ## Example
/// - `?skip=10&take=10` Get the results from 10 to 20.
///
/// # Response
/// ```json
/// {
///     "id": "86df7b6c-2377-4cd6-ac1c-badfef243f3b",
///     "username": "NextChat",
///     "profile_image": "url.png"
/// }
/// ```
///
/// ## Status codes
/// - `204` - When the search is successful but does not return anything.
/// - `200` - When the search is successful and returns one or more results.
///
/// ## Errors
/// 1. Cannot get the users.
pub async fn search_handler(
    text_to_search: String,
    query: SearchQuery,
    client: Client,
) -> Result<impl Reply, Infallible> {
    // Get the search query.
    let sql = format!("SELECT id, username, profile_image FROM users WHERE username LIKE '{}%' LIMIT $1 OFFSET $2", text_to_search);
    match nextchat_database::query(&sql)
        .bind(query.take.unwrap_or(10))
        .bind(query.skip.unwrap_or(0))
        .fetch_all(&client)
        .await
    {
        Err(_) => Ok(Error::from_str("Cannot get the users.")
            .to_response(400)
            .to_reply()),
        Ok(users) => {
            let users: Vec<UserDataResponse> = users
                .iter()
                .map(|user| UserDataResponse::from_row(user))
                .collect();
            Ok(Response::new(if users.is_empty() { 204 } else { 200 }, users).to_reply())
        }
    }
}

/// `/users/find` handler.
///
/// # Request query
/// 1. Get the user by id: `?id={user_id}`
/// 2. Get the user by username: `?username={username}`
///
/// # Response
/// ```json
/// {
///     "id": "86df7b6c-2377-4cd6-ac1c-badfef243f3b",
///     "username": "NextChat",
///     "profile_image": "url.png"
/// }
/// ```
///
/// ## Errors
/// 1. Cannot find a user by id and username at the same time.
/// 2. Cannot find the user #{user_id}.
/// 3. Cannot find the user by its name: {username}.
/// 4. Invalid query.
pub async fn find_handler(query: FindQuery, client: Client) -> Result<impl Reply, Infallible> {
    if query.id.is_some() && query.username.is_some() {
        Ok(
            Error::from_str("Cannot find a user by id and username at the same time.")
                .to_response(400)
                .to_reply(),
        )
    } else if let Some(id) = query.id {
        // Get the user by id.
        match nextchat_database::query(
            "SELECT id, username, profile_image FROM users WHERE id = $1",
        )
        .bind(&id)
        .fetch_one(&client)
        .await
        {
            Err(_) => Ok(Error::new(format!("Cannot find the user #{}.", id))
                .to_response(400)
                .to_reply()),
            Ok(user) => Ok(Response::new_success(UserDataResponse::from_row(&user)).to_reply()),
        }
    } else if let Some(username) = query.username {
        // Get the user by username.
        match nextchat_database::query(
            "SELECT id, username, profile_image FROM users username = $1",
        )
        .bind(&username)
        .fetch_one(&client)
        .await
        {
            Err(_) => Ok(
                Error::new(format!("Cannot find the user by its name: {}.", username))
                    .to_response(400)
                    .to_reply(),
            ),
            Ok(user) => Ok(Response::new_success(UserDataResponse::from_row(&user)).to_reply()),
        }
    } else {
        Ok(Error::from_str("Invalid query.")
            .to_response(400)
            .to_reply())
    }
}

/// `/users/signup` handler.
///
/// # Request body
/// ```json
/// {
///     "username": "NextChat",
///     "password": "1234"
/// }
/// ```
///
/// ## Requeriments
/// - `username` **Required** - Min length: 4 - Max length: 15
/// - `password` **Required** - Min length: 8 - Max length: 40
///
/// # Response
/// ```json
/// {
///     "id": "86df7b6c-2377-4cd6-ac1c-badfef243f3b",
///     "username": "NextChat",
///     "profile_image": "url.png"
/// }
/// ```
///
/// ## Errors
/// 1. You must enter the username.
/// 2. The username must be between 4 and 15 characteres.
/// 3. You must enter the password.
/// 4. The password must be between 8 and 40 characteres.
/// 5. The username already exists.
/// 6. Cannot encrypt the password.
/// 7. Cannot create the user.
/// 8. Rows not affected.
/// 9. Unknown.
pub async fn signup_handler(
    body: SignUpAndSignInBody,
    client: Client,
) -> Result<impl Reply, Infallible> {
    if body.username.is_empty() {
        return Ok(Error::from_str("You must enter the username.")
            .to_response(400)
            .to_reply());
    } else if body.username.len() < 4 || body.username.len() > 15 {
        return Ok(
            Error::from_str("The username must be between 4 and 15 characteres.")
                .to_response(400)
                .to_reply(),
        );
    } else if body.password.is_empty() {
        return Ok(Error::from_str("You must enter the password.")
            .to_response(400)
            .to_reply());
    } else if body.password.len() < 8 || body.password.len() > 40 {
        return Ok(
            Error::from_str("The password must be between 8 and 40 characteres.")
                .to_response(400)
                .to_reply(),
        );
    }

    // Check if the username is already in use.
    match nextchat_database::query("SELECT COUNT(id) AS count FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(&client)
        .await
    {
        Err(_) => Ok(Error::from_str("Unknown.").to_response(400).to_reply()),
        Ok(with_username) => {
            let count: i64 = with_username.get(0);
            if count > 0 {
                return Ok(Error::from_str("The username already exists.")
                    .to_response(400)
                    .to_reply());
            }

            // Encrypt and get the password.
            let password: String = match encrypt_password(&body.password) {
                Ok(hash) => hash,
                Err(_) => {
                    return Ok(Error::from_str("Cannot encrypt the password.")
                        .to_response(400)
                        .to_reply());
                }
            };

            // Generate a new UUID.
            let user_id: Uuid = Uuid::new_v4();

            // Add the user to the database.
            match nextchat_database::query(
                "INSERT INTO users(id, username, password, profile_image) VALUES ($1, $2, $3, $4)",
            )
            .bind(&user_id)
            .bind(&body.username)
            .bind(&password)
            .execute(&client)
            .await
            {
                Err(_) => Ok(Error::from_str("Cannot create the user.")
                    .to_response(400)
                    .to_reply()),
                Ok(result) => {
                    // Check if the user was added successfully.
                    if result.rows_affected() == 1 {
                        Ok(Response::new_success(UserDataResponse {
                            id: user_id,
                            username: body.username,
                            profile_image: String::new(),
                        })
                        .to_reply())
                    } else {
                        Ok(Error::from_str("Rows not affected.")
                            .to_response(400)
                            .to_reply())
                    }
                }
            }
        }
    }
}

/// `/users/signin` handler.
///
/// # Request body
/// ```json
/// {
///     "username": "NextChat",
///     "password": "1234"
/// }
/// ```
///
/// ## Requeriments
/// - `username` **Required**
/// - `password` **Required**
///
/// # Response
/// ```json
/// {
///     "id": "86df7b6c-2377-4cd6-ac1c-badfef243f3b",
///     "username": "NextChat",
///     "profile_image": "url.png"
/// }
/// ```
///
/// ## Errors
/// 1. You must enter the username.
/// 2. You must enter the password.
/// 3. The username does not exist.
/// 4. The password is incorrect.
pub async fn signin_handler(
    body: SignUpAndSignInBody,
    client: Client,
) -> Result<impl Reply, Infallible> {
    if body.username.is_empty() {
        return Ok(Error::from_str("You must enter the username.")
            .to_response(400)
            .to_reply());
    } else if body.password.is_empty() {
        return Ok(Error::from_str("You must enter the password.")
            .to_response(400)
            .to_reply());
    }

    match nextchat_database::query(
        "SELECT id, password, username, profile_image FROM users WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_one(&client)
    .await
    {
        Err(_) => Ok(Error::from_str("The username does not exist.")
            .to_response(400)
            .to_reply()),
        Ok(user) => {
            let is_password: bool = match verify_password(
                &body.password,
                user.try_get("password")
                    .expect("Cannot parse the user password."),
            ) {
                Ok(result) => result,
                Err(e) => {
                    return Ok(Error::new(format!("{}", e)).to_response(400).to_reply());
                }
            };

            if !is_password {
                Ok(Error::from_str("The password is incorrect.")
                    .to_response(400)
                    .to_reply())
            } else {
                Ok(Response::new_success(UserDataResponse::from_row(&user)).to_reply())
            }
        }
    }
}
