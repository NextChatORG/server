use super::{with_client, Error, ResponseBody};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::convert::Infallible;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

fn friends_path_prefix() -> BoxedFilter<()> {
    warp::path("friends").boxed()
}

fn are_friends(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(
        user_one: Uuid,
        user_two: Uuid,
        client: PgPool,
    ) -> Result<impl Reply, Infallible> {
        #[derive(Deserialize, Serialize)]
        struct Response {
            pub are_friends: bool,
            pub since: Option<NaiveDateTime>,
        }

        match sqlx::query("SELECT since FROM friends WHERE ((user_one = $1 AND user_two = $2) OR (user_one = $2 AND user_two = $1)) AND is_request = false")
            .bind(user_one)
            .bind(user_two)
            .fetch_one(&client)
            .await
        {
            Err(e) => {
                eprintln!("Friend request: {:?}", e);
                Ok(ResponseBody::new(400, Response { are_friends: false, since: None }).to_reply())
            },
            Ok(friend) => {
                let since: NaiveDateTime = friend.try_get("since").expect("Cannot parse since timestamp.");
                Ok(ResponseBody::new_success(Response { are_friends: true, since: Some(since) }).to_reply())
            }
        }
    }

    warp::get()
        .and(friends_path_prefix())
        .and(warp::path!(Uuid / Uuid))
        .and(with_client(client.clone()))
        .and_then(handler)
}

fn get_friends_of_user(
    client: &PgPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(user_id: Uuid, client: PgPool) -> Result<impl Reply, Infallible> {
        #[derive(Serialize)]
        struct Response {
            pub user_id: Uuid,
            pub since: NaiveDateTime,
        }

        match sqlx::query("SELECT user_one, user_two, since FROM friends WHERE (user_one = $1 OR user_two = $1) AND is_request = true")
            .bind(&user_id)
            .fetch_all(&client)
            .await
        {
            Err(e) => {
                eprintln!("Get Friends Of User: {:?}", e);
                Ok(ResponseBody::new(400, Error::new(0, format!("Cannot get the friends of {}.", user_id))).to_reply())
            },
            Ok(friends) => {
                let mut friends_res: Vec<Response> = Vec::new();

                for friend in friends {
                    let user_one: Uuid = friend.try_get("user_one").expect("Cannot parse the user one id.");
                    let user_two: Uuid = friend.try_get("user_two").expect("Cannot parse user two id.");

                    let since: NaiveDateTime = friend.try_get("since").expect("Cannot parse the since timestamp.");

                    if user_one == user_id {
                        friends_res.push(Response {
                            user_id: user_two,
                            since,
                        });
                    } else if user_two == user_id {
                        friends_res.push(Response {
                            user_id: user_one,
                            since,
                        });
                    }
                }

                Ok(ResponseBody::new(if friends_res.is_empty() { 204 } else { 200 }, friends_res).to_reply())
            }
        }
    }

    warp::get()
        .and(friends_path_prefix())
        .and(warp::path!("get-of" / Uuid))
        .and(with_client(client.clone()))
        .and_then(handler)
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    are_friends(client).or(get_friends_of_user(client))
}
