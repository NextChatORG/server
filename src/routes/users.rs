use super::Error;
use crate::database::{User, UserData};
use actix_web::{get, post, web, HttpResponse, Result, Scope};
use serde::Deserialize;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[get("/all")]
async fn all(client: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match sqlx::query("SELECT id, username, online, last_online, created_at FROM users")
        .fetch_all(client.get_ref())
        .await
    {
        Err(_) => Err(Error::new("Cannot get the users.")),
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

#[get("/find")]
async fn find(
    web::Query(query): web::Query<FindUserQuery>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    if query.id.is_some() && query.username.is_some() {
        Err(Error::new(
            "Cannot find a user by id and username at the same time.",
        ))
    } else if let Some(id) = query.id {
        match User::from_id(client.get_ref(), &id, false).await {
            Err(e) => {
                println!("User by id: Error: {}", e);
                Err(Error::new("Cannot find this user."))
            }
            Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
        }
    } else if let Some(username) = query.username.clone() {
        match User::from_username(client.get_ref(), &username, false).await {
            Err(e) => {
                println!("User by username: Error: {}", e);
                Err(Error::new("Cannot find this user."))
            }
            Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
        }
    } else {
        Err(Error::new("Invalid query."))
    }
}

#[derive(Deserialize)]
struct SignUpBody {
    pub username: String,
    pub password: String,
}

#[post("/signup")]
async fn signup(
    user_data: web::Json<SignUpBody>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let with_username = sqlx::query("SELECT COUNT(id) AS count FROM users WHERE username = $1")
        .bind(&user_data.username.clone())
        .fetch_one(client.get_ref())
        .await
        .expect("Cannot execute the query.");

    let count: i64 = with_username.get(0);
    if count > 0 {
        return Err(Error::new("The username already exists."));
    }

    let mut user = User::default();

    user.get_data().set_username(user_data.username.clone());
    user.get_data().set_password(user_data.password.clone());

    match user.save(client.get_ref(), true).await {
        Err(e) => {
            println!("User Create: Error: {}", e);
            Err(Error::new("Cannot create the user."))
        }
        Ok(_) => Ok(HttpResponse::Ok().body(user.get_data().get_id().to_string())),
    }
}

#[derive(Deserialize)]
struct SignInBody {
    pub username: String,
    pub password: String,
}

#[post("/signin")]
async fn signin(
    user_data: web::Json<SignInBody>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    match User::from_username(client.get_ref(), &user_data.username.clone(), true).await {
        Err(e) => {
            println!("User sign in: Error: {}", e);
            Err(Error::new("The username does not exist."))
        }
        Ok(user) => {
            if !user.is_password(user_data.password.clone()) {
                Err(Error::new("The password is incorrect."))
            } else {
                Ok(HttpResponse::Ok().body(user.get_data().get_id().to_string()))
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
