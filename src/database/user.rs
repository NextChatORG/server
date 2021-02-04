use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Done, Error, PgPool, Row};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    id: Uuid,
    username: String,
    password: Option<String>,

    profile_image: String,

    online: bool,
    last_online: NaiveDateTime,
    created_at: NaiveDateTime,
}

impl UserData {
    pub fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Cannot get timestamp.");
        let now_time = NaiveDateTime::from_timestamp(now.as_secs() as i64, 0);

        Self {
            id: Uuid::new_v4(),
            username: String::new(),
            password: None,

            profile_image: String::new(),

            online: false,
            last_online: now_time,
            created_at: now_time,
        }
    }

    pub fn from_row(row: &PgRow, with_password: bool) -> Self {
        Self {
            id: row.try_get("id").expect("Cannot parse the user id."),
            username: row.try_get("username").expect("Cannot parse the username."),
            password: if with_password {
                Some(
                    row.try_get("password")
                        .expect("Cannot parse the user password."),
                )
            } else {
                None
            },

            profile_image: row
                .try_get("profile_image")
                .expect("Cannot parse the user profile image."),

            online: row
                .try_get("online")
                .expect("Cannot parse the user online status."),
            last_online: row
                .try_get("last_online")
                .expect("Cannot parse the user last online timestamp."),
            created_at: row
                .try_get("created_at")
                .expect("Cannot parse the user created at timestamp."),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = Some(crate::security::encrypt_password(password));
    }
}

#[derive(Clone, Debug)]
pub struct User {
    data: UserData,
    in_database: bool,
}

impl User {
    pub fn default() -> Self {
        Self {
            data: UserData::default(),
            in_database: false,
        }
    }

    pub fn from_row(row: &PgRow, with_password: bool) -> Self {
        Self {
            data: UserData::from_row(row, with_password),
            in_database: true,
        }
    }

    pub async fn from_id(client: &PgPool, id: &Uuid, with_password: bool) -> Result<Self, Error> {
        let mut query = String::from("SELECT id, username, ");
        if with_password {
            query.push_str("password, ");
        }
        query.push_str("profile_image, online, last_online, created_at FROM users WHERE id = $1");

        let result = sqlx::query(&query).bind(id).fetch_one(client).await?;

        Ok(User::from_row(&result, with_password))
    }

    pub async fn from_username(
        client: &PgPool,
        username: &String,
        with_password: bool,
    ) -> Result<Self, Error> {
        let mut query = String::from("SELECT id, username, ");
        if with_password {
            query.push_str("password, ");
        }
        query.push_str("profile_image, online, last_online, created_at FROM users WHERE username = $1");

        let result = sqlx::query(&query).bind(username).fetch_one(client).await?;

        Ok(User::from_row(&result, with_password))
    }

    pub fn get_data(&self) -> UserData {
        self.data.clone()
    }

    pub fn get_data_mut(&mut self) -> &mut UserData {
        &mut self.data
    }

    pub fn is_password(&self, password: String) -> bool {
        if self.data.password.is_none() {
            panic!("The user password cannot be found.");
        }

        crate::security::verify_password(password, self.data.password.clone().unwrap())
    }

    pub async fn save(&mut self, client: &PgPool, with_password: bool) -> Result<(), Error> {
        let query: &str = if self.in_database {
            if with_password {
                "UPDATE FROM users SET username = $2, password = $3, profile_image = $4, online = $5, last_online = $6, created_at = $7 WHERE id = $1"
            } else {
                "UPDATE FROM users SET username = $2, profile_iamge = $3, online = $4, last_online = $5, created_at = $6 WHERE id = $1"
            }
        } else {
            if with_password {
                "INSERT INTO users (id, username, password, profile_image, online, last_online, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
            } else {
                "INSERT INTO users (id, username, profile_image, online, last_online, created_at) VALUES ($1, $2, $3, $4, $5, $6)"
            }
        };

        let mut query = sqlx::query(query)
            .bind(&self.data.id)
            .bind(&self.data.username);

        if with_password {
            query = query.bind(&self.data.password);
        }

        let result = query
            .bind(&self.data.profile_image)
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
