//! NextChat Server friends service module.

use std::convert::Infallible;

use nextchat_database::{models::friends::FriendModel, Client, NaiveDateTime, Uuid};
use serde::Serialize;
use warp::Reply;

use crate::response::Response;

pub async fn get_friend_model_of(
    client: &Client,
    user_one: &Uuid,
    user_two: &Uuid,
) -> Result<FriendModel, nextchat_database::Error> {
    match nextchat_database::query("SELECT transmitter, receiver, state, since FROM friends WHERE (transmitter = $1 AND receiver = $2) OR (transmitter = $2 AND receiver = $1)")
    .bind(user_one)
    .bind(user_two)
    .fetch_one(client)
    .await
    {
        Err(e) => Err(e),
        Ok(row) => Ok(FriendModel::from_row(&row)),
    }
}

pub async fn are_friends_handler(
    user_one: Uuid,
    user_two: Uuid,
    client: Client,
) -> Result<impl Reply, Infallible> {
    #[derive(Serialize)]
    struct ResponseData {
        pub are_friends: bool,
        pub since: Option<NaiveDateTime>,
    }

    match get_friend_model_of(&client, &user_one, &user_two).await {
        Err(_) => Ok(Response::new(
            400,
            ResponseData {
                are_friends: false,
                since: None,
            },
        )
        .to_reply()),
        Ok(friend) => {
            let are_friends = friend.get_state().is_approved();
            Ok(Response::new_success(ResponseData {
                since: if are_friends {
                    Some(friend.get_since())
                } else {
                    None
                },
                are_friends,
            })
            .to_reply())
        }
    }
}
