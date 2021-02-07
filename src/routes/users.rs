use super::Error;
use crate::database::{User, UserData};
use actix_web::{get, post, web, HttpResponse, Result, Scope};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

// Error codes:
// 0 -> Cannot get the users.
#[get("/all")]
async fn all(client: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match sqlx::query("SELECT id, username, profile_image, online, last_online, created_at FROM users")
        .fetch_all(client.get_ref())
        .await
    {
        Err(_) => Err(Error::new_str(0, "Cannot get the users.")),
        Ok(rows) => {
            let mut users: Vec<UserData> = Vec::new();
            for row in rows.iter() {
                users.push(UserData::from_row(row, false));
            }

            Ok(HttpResponse::Ok().json(users))
        }
    }
}

#[derive(Deserialize)]
struct FindUserQuery {
    id: Option<Uuid>,
    username: Option<String>,
}

// Error codes:
// 0 -> Username and id in the query.
// 1 -> User id or username does not exist.
// 2 -> Invalid query.
#[get("/find")]
async fn find(
    web::Query(query): web::Query<FindUserQuery>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    if query.id.is_some() && query.username.is_some() {
        Err(Error::new_str(
            0,
            "Cannot find a user by id and username at the same time.",
        ))
    } else if let Some(id) = query.id {
        match User::from_id(client.get_ref(), &id, false).await {
            Err(e) => {
                println!("User by id: Error: {}", e);
                Err(Error::new(1, format!("Cannot find the user #{}", id)))
            }
            Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
        }
    } else if let Some(username) = query.username.clone() {
        match User::from_username(client.get_ref(), &username, false).await {
            Err(e) => {
                println!("User by username: Error: {}", e);
                Err(Error::new(1, format!("Cannot find the user by its name: {}", username)))
            }
            Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
        }
    } else {
        Err(Error::new_str(2, "Invalid query."))
    }
}

#[derive(Deserialize)]
struct SignUpBody {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
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
#[post("/signup")]
async fn signup(
    user_data: web::Json<SignUpBody>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    if user_data.username.len() == 0 {
        return Err(Error::new_str(0, "You must enter the username."));
    } else if user_data.username.len() < 4 || user_data.username.len() > 15 {
        return Err(Error::new_str(1, "The username must be between 4 and 15 characteres."));
    } else if user_data.password.len() == 0 {
        return Err(Error::new_str(2, "You must enter the password."));
    } else if user_data.password.len() < 8 ||user_data.password.len() > 40 {
        return Err(Error::new_str(3, "The password must be between 8 and 40 characteres."));
    }

    let with_username = sqlx::query("SELECT COUNT(id) AS count FROM users WHERE username = $1")
        .bind(&user_data.username.clone())
        .fetch_one(client.get_ref())
        .await
        .expect("Cannot execute the query.");

    let count: i64 = with_username.get(0);
    if count > 0 {
        return Err(Error::new_str(4, "The username already exists."));
    }

    let mut user = User::default();

    user.get_data_mut().set_username(user_data.username.clone());
    user.get_data_mut().set_password(user_data.password.clone());

    match user.save(client.get_ref(), true).await {
        Err(e) => {
            println!("User Create: Error: {}", e);
            Err(Error::new_str(5, "Cannot create the user."))
        }
        Ok(_) => Ok(HttpResponse::Ok().json(SignUpAndSignInResponse {
            id: user.get_data().get_id().to_string(),
            username: user.get_data().get_username(),
            profile_image: user.get_data().get_profile_image()
        })),
    }
}

#[derive(Deserialize)]
struct SignInBody {
    pub username: String,
    pub password: String,
}


// Error codes:
// 0 -> Username is empty.
// 1 -> Password is empty.
// 2 -> Username does not exist.
// 3 -> The password is incorrect.
#[post("/signin")]
async fn signin(
    user_data: web::Json<SignInBody>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    if user_data.username.len() == 0 {
        return Err(Error::new_str(0, "You must enter the username."));
    } else if user_data.password.len() == 0 {
        return Err(Error::new_str(1, "You must enter the password."));
    }

    match User::from_username(client.get_ref(), &user_data.username.clone(), true).await {
        Err(e) => {
            println!("User sign in: Error: {}", e);
            Err(Error::new_str(2, "The username does not exist."))
        }
        Ok(user) => {
            if !user.is_password(user_data.password.clone()) {
                Err(Error::new_str(3, "The password is incorrect."))
            } else {
                Ok(HttpResponse::Ok().json(SignUpAndSignInResponse {
                    id: user.get_data().get_id().to_string(),
                    username: user.get_data().get_username(),
                    profile_image: user.get_data().get_profile_image()
                }))
            }
        }
    }
}

pub fn scope() -> Scope {
    web::scope("/users")
        .service(all)
        .service(find)
        .service(signup)
        .service(signin)
}
