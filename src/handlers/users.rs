use super::{with_client, Error, ResponseBody};
use crate::database::models::UserModel;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::convert::Infallible;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

fn users_path_prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

// Error codes:
// 0 -> Cannot get the users.
fn list(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(client: PgPool) -> Result<impl Reply, Infallible> {
        match sqlx::query(
            "SELECT id, username, profile_image, online, last_online, created_at FROM users",
        )
        .fetch_all(&client)
        .await
        {
            Err(_) => Ok(ResponseBody::<Error>::new_error(Error::new_str(
                0,
                "Cannot get the users.",
            ))
            .to_reply()),
            Ok(rows) => {
                let users: Vec<UserModel> = rows
                    .iter()
                    .map(|row| UserModel::from_row(row, false))
                    .collect();
                Ok(ResponseBody::new_success(users).to_reply())
            }
        }
    }

    warp::get()
        .and(users_path_prefix())
        .and(warp::path("all"))
        .and(with_client(client.clone()))
        .and_then(handler)
}

// Error codes:
// 0 -> Username and id in the query.
// 1 -> User id or username does not exist.
// 2 -> Invalid query.
fn find(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[derive(Deserialize)]
    struct FindQuery {
        pub id: Option<Uuid>,
        pub username: Option<String>,
    }

    async fn handler(query: FindQuery, client: PgPool) -> Result<impl Reply, Infallible> {
        if query.id.is_some() && query.username.is_some() {
            Ok(ResponseBody::<Error>::new_error(Error::new_str(
                0,
                "Cannot find a user by id and username at the same time.",
            ))
            .to_reply())
        } else if let Some(id) = query.id {
            match UserModel::from_id(&client, &id, false).await {
                Err(e) => {
                    eprintln!("User by id: Error: {:?}", e);
                    Ok(ResponseBody::<Error>::new_error(Error::new(
                        1,
                        format!("Cannot find the user #{}", id),
                    ))
                    .to_reply())
                }
                Ok(user) => Ok(ResponseBody::new_success(user).to_reply()),
            }
        } else if let Some(username) = query.username {
            match UserModel::from_username(&client, &username, false).await {
                Err(e) => {
                    eprintln!("User by username: Error: {:?}", e);
                    Ok(ResponseBody::<Error>::new_error(Error::new(
                        1,
                        format!("Cannot find the user by its name: {}", username),
                    ))
                    .to_reply())
                }
                Ok(user) => Ok(ResponseBody::new_success(user).to_reply()),
            }
        } else {
            Ok(ResponseBody::<Error>::new_error(Error::new_str(2, "Invalid query.")).to_reply())
        }
    }

    warp::get()
        .and(users_path_prefix())
        .and(warp::path("find"))
        .and(warp::query::<FindQuery>())
        .and(with_client(client.clone()))
        .and_then(handler)
}

#[derive(Deserialize)]
struct SignUpAndSigInBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct SignUpAndSignInResponse {
    pub id: String,
    pub username: String,
    pub profile_image: String,
}

// Error codes:
// 0 -> Username is empty.
// 1 -> Username between 4 and 15 characteres.
// 2 -> Password is empty.
// 3 -> Password between 8 and 40 characteres.
// 4 -> Username already exists.
// 5 -> Unknown.
fn signup(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(body: SignUpAndSigInBody, client: PgPool) -> Result<impl Reply, Infallible> {
        if body.username.is_empty() {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                0,
                "You must enter the username.",
            ))
            .to_reply());
        } else if body.username.len() < 4 || body.username.len() > 15 {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                1,
                "The username must be between 4 and 15 characteres.",
            ))
            .to_reply());
        } else if body.password.is_empty() {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                2,
                "You must enter the password.",
            ))
            .to_reply());
        } else if body.password.len() < 8 || body.password.len() > 40 {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                3,
                "The password must be between 8 and 40 characteres.",
            ))
            .to_reply());
        }

        match sqlx::query("SELECT COUNT(id) AS count FROM users WHERE username = $1")
            .bind(&body.username)
            .fetch_one(&client)
            .await
        {
            Err(e) => {
                eprintln!("SignUp: Error: {:?}", e);
                Ok(ResponseBody::<Error>::new_error(Error::new_str(5, "Unknown.")).to_reply())
            }
            Ok(with_username) => {
                let count: i64 = with_username.get(0);
                if count > 0 {
                    return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                        4,
                        "The username already exists.",
                    ))
                    .to_reply());
                }

                let mut user = UserModel::default();

                user.set_username(&body.username);
                let password = user.set_password(&body.password);

                match sqlx::query("INSERT INTO users (id, username, password, profile_image, online, last_online, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&user.get_id())
                    .bind(&user.get_username())
                    .bind(&password)
                    .bind(&user.get_profile_image())
                    .bind(&user.is_online())
                    .bind(&user.get_last_online())
                    .bind(&user.get_created_at())
                    .execute(&client)
                    .await {
                    Err(e) => {
                        eprintln!("Signup: Error: {:?}", e);
                        Ok(ResponseBody::<Error>::new_error(Error::new_str(5, "Cannot create the user.")).to_reply())
                    },
                    Ok(result) => {
                        if result.rows_affected() == 1 {
                            Ok(ResponseBody::new_success(SignUpAndSignInResponse {
                                id: user.get_id().to_string(),
                                username: user.get_username(),
                                profile_image: user.get_profile_image(),
                            }).to_reply())
                        } else {
                            Ok(ResponseBody::<Error>::new_error(Error::new_str(5, "Rows not affected.")).to_reply())
                        }
                    },
                }
            }
        }
    }

    warp::post()
        .and(users_path_prefix())
        .and(warp::path("signup"))
        .and(warp::body::json::<SignUpAndSigInBody>())
        .and(with_client(client.clone()))
        .and_then(handler)
}

// Error codes:
// 0 -> Username is empty.
// 1 -> Password is empty.
// 2 -> Username does not exist.
// 3 -> The password is incorrect.
fn signin(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(body: SignUpAndSigInBody, client: PgPool) -> Result<impl Reply, Infallible> {
        if body.username.is_empty() {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                0,
                "You must enter the username.",
            ))
            .to_reply());
        } else if body.password.is_empty() {
            return Ok(ResponseBody::<Error>::new_error(Error::new_str(
                1,
                "You must enter the password.",
            ))
            .to_reply());
        }

        match UserModel::from_username(&client, &body.username, true).await {
            Err(e) => {
                eprintln!("Signin: Error: {:?}", e);
                Ok(ResponseBody::<Error>::new_error(Error::new_str(
                    2,
                    "The username does not exist.",
                ))
                .to_reply())
            }
            Ok(user) => {
                if !user.verify_password(body.password) {
                    Ok(ResponseBody::<Error>::new_error(Error::new_str(
                        3,
                        "The password is incorrect.",
                    ))
                    .to_reply())
                } else {
                    Ok(ResponseBody::new_success(SignUpAndSignInResponse {
                        id: user.get_id().to_string(),
                        username: user.get_username(),
                        profile_image: user.get_profile_image(),
                    })
                    .to_reply())
                }
            }
        }
    }

    warp::post()
        .and(users_path_prefix())
        .and(warp::path("signin"))
        .and(warp::body::json::<SignUpAndSigInBody>())
        .and(with_client(client.clone()))
        .and_then(handler)
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list(client)
        .or(find(client))
        .or(signup(client))
        .or(signin(client))
}
