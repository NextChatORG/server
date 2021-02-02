use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use sqlx::{Done, Error, PgPool, postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDataOptional {
    id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,

    online: Option<bool>,
    last_online: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}

impl UserDataOptional {
    pub fn to_user_data(&self) -> UserData {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Cannot get timestamp.");
        let now_time = NaiveDateTime::from_timestamp(now.as_secs() as i64, 0);

        UserData {
            id: self.id.unwrap_or(Uuid::new_v4()),
            username: self.username.clone().unwrap_or(String::new()),
            password: self.password.clone().unwrap_or(String::new()),

            online: self.online.unwrap_or(false),
            last_online: self.last_online.unwrap_or(now_time),
            created_at: self.created_at.unwrap_or(now_time),
        }
    }

    pub fn to_user(&self) -> User {
        User {
            data: self.to_user_data(),
            in_database: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    id: Uuid,
    username: String,
    password: String,

    online: bool,
    last_online: NaiveDateTime,
    created_at: NaiveDateTime,
}

impl UserData {
    pub fn from_row(row: &PgRow) -> Self {
        Self {
            id: row.try_get("id").expect("Cannot parse the user id."),
            username: row.try_get("username").expect("Cannot parse the username."),
            password: row.try_get("password").expect("Cannot parse the user password."),

            online: row.try_get("online").expect("Cannot parse the user online status."),
            last_online: row.try_get("last_online").expect("Cannot parse the user last online timestamp."),
            created_at: row.try_get("created_at").expect("Cannot parse the user created at timestamp."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    data: UserData,
    in_database: bool,
}

impl User {
    pub fn from_row(row: &PgRow) -> Self {
        Self {
            data: UserData::from_row(row),
            in_database: true,
        }
    }

    pub async fn from_id(client: &PgPool, id: &Uuid) -> Result<Self, Error> {
        let result = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(client)
            .await?;

        Ok(User::from_row(&result))
    }

    pub async fn from_username(client: &PgPool, username: &String) -> Result<Self, Error> {
        let result = sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(client)
            .await?;

        Ok(User::from_row(&result))
    }

    pub fn get_data(&self) -> UserData {
        self.data.clone()
    }

    pub async fn save(&mut self, client: &PgPool) -> Result<(), Error> {
        let query: &str = if self.in_database {
            "UPDATE FROM users SET username = $2, password = $3, online = $4, last_online = $5, created_at = $6 WHERE id = $1"
        } else {
            "INSERT INTO users (id, username, password, online, last_online, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
        };

        let result = sqlx::query(query)
            .bind(&self.data.id)
            .bind(&self.data.username)
            .bind(&self.data.password)
            .bind(&self.data.online)
            .bind(&self.data.last_online)
            .bind(&self.data.created_at)
            .execute(client)
            .await?
            .rows_affected();

        if result > 0 && !self.in_database {
            self.in_database = true;
        }

        Ok(())
    }
}
