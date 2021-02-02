use actix_web::{get, HttpResponse, post, Result, Scope, web};
use crate::database::{User, UserData, UserDataOptional};
use sqlx::PgPool;
use super::Error;
use uuid::Uuid;

#[get("/all")]
async fn all(client: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match sqlx::query("SELECT * FROM users")
        .fetch_all(client.get_ref())
        .await
    {
        Err(_) => Err(Error::new("Cannot get the users.")),
        Ok(rows) => {
            let mut users: Vec<UserData> = Vec::new();
            for row in rows.iter() {
                users.push(UserData::from_row(row));
            }
        
            Ok(HttpResponse::Ok().json(users))
        },
    }
}

#[get("/id/{id}")]
async fn find_by_id(
    path: web::Path<(Uuid,)>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let id: Uuid = path.into_inner().0;
    match User::from_id(client.get_ref(), &id).await {
        Err(e) => {
            println!("User by id: Error: {}", e);
            Err(Error::new("Cannot find this user."))
        },
        Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
    }
}

#[get("/username/{username}")]
async fn find_by_username(
    path: web::Path<(String,)>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let username: String = path.into_inner().0;
    match User::from_username(client.get_ref(), &username).await {
        Err(e) => {
            println!("User by username: Error: {}", e);
            Err(Error::new("Cannot find this user."))
        },
        Ok(user) => Ok(HttpResponse::Ok().json(user.get_data())),
    }
}

#[post("/create")]
async fn create(
    user_data: web::Json<UserDataOptional>,
    client: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    if user_data.username.is_none() {
        return Err(Error::new("You need add the username."));
    } else if user_data.password.is_none() {
        return Err(Error::new("You need add the password of the account."));
    }

    let mut user: User = user_data.to_user();
    match user.save(client.get_ref()).await {
        Err(e) => {
            println!("User Create: Error: {}", e);
            Err(Error::new("Cannot create the user."))
        },
        Ok(_) => Ok(HttpResponse::Ok().json(user.get_data()))
    }
}

pub fn scope() -> Scope {
    web::scope("/users")
        .service(all)
        .service(
            web::scope("/find")
                .service(find_by_id)
                .service(find_by_username)
        )
        .service(create)
}
