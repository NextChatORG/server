use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::convert::Infallible;
use super::{with_client, ResponseBody};
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

fn friends_path_prefix() -> BoxedFilter<()> {
    warp::path("friends").boxed()
}

fn are_friends(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    async fn handler(user_one: Uuid, user_two: Uuid, client: PgPool) -> Result<impl Reply, Infallible> {
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
        .and(warp::path!("are_friends" / Uuid / Uuid))
        .and(with_client(client.clone()))
        .and_then(handler)
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    are_friends(client)
}
