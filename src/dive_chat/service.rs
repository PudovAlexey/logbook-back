pub struct ChatListByUserIdParams {
    pub id: uuid::Uuid,
}
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgConnection, SelectableHelper};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dive_chat::model::{Chat, ChatUser, Message, UserWithAuthor};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

use crate::schema::chat;
use crate::schema::chat_user;
use crate::schema::message;
use crate::schema::users;
use crate::users::model::USER;

use super::kafka_chat_handler::KafkaChatHandler;

pub fn get_chat_list_by_user_id(
    connection: PooledPg,
    params: ChatListByUserIdParams,
) -> Result<Vec<Chat>, diesel::result::Error> {
    let ChatListByUserIdParams { id: user_auth_id } = params;

    let mut connection = connection;

    let chat_data = chat_user::table
        .left_join(chat::table.on(chat_user::columns::chat_id.eq(chat::columns::id)))
        .filter(chat_user::columns::user_id.eq(user_auth_id))
        .select((ChatUser::as_select(), Option::<Chat>::as_select()))
        .load(&mut connection)?;

    let chat_data: Vec<Chat> =
        chat_data
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Chat>, (_, chat)| {
                if chat.is_some() {
                    acc.push(chat.unwrap())
                }

                acc
            });

    Ok(chat_data)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetMessageListByIdParams {
    pub id: i32,
}

pub fn get_message_list_by_id(
    connection: PooledPg,
    params: GetMessageListByIdParams,
) -> Result<Vec<UserWithAuthor>, diesel::result::Error> {
    let GetMessageListByIdParams { id: chat_id } = params;

    let mut connection = connection;

    let message_data: Vec<(Message, Option<USER>)> = message::table
        .left_join(users::table.on(message::columns::user_id.eq(users::columns::id.nullable())))
        .filter(message::columns::chat_id.eq(chat_id))
        .select((Message::as_select(), Option::<USER>::as_select()))
        .load(&mut connection)?;

    let map_messages = message_data
        .iter()
        .map(|(message, author)| UserWithAuthor {
            message: (*message).clone(),
            author: author.clone(),
        })
        .collect();

    // Ok(UserWithAuthor {
    //     message: message_data[0],
    //     author: message_data[1]
    // })

    Ok(map_messages)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateChatParams {
    pub title: String,
    pub description: String,
    pub participants: Vec<uuid::Uuid>,
}

pub fn create_chat_mutation(
    connection: PooledPg,
    params: CreateChatParams,
) -> Result<i32, diesel::result::Error> {
    let CreateChatParams {
        title: chat_title,
        description,
        participants,
    } = params;

    let mut connection = connection;

    let mut new_chat: i32 = diesel::insert_into(chat::table)
        .values((
            chat::columns::title.eq(chat_title),
            chat::columns::description.eq(description),
        ))
        .returning(chat::columns::id)
        .get_result(&mut connection)?;

    let insert_values: Vec<_> = participants
        .iter()
        .map(|participant| {
            (
                chat_user::columns::chat_id.eq(new_chat),
                chat_user::columns::user_id.eq(participant),
            )
        })
        .collect();

    let _: i32 = diesel::insert_into(chat_user::table)
        .values(insert_values)
        .returning(chat_user::columns::id)
        .get_result(&mut connection)?;

    Ok(new_chat)
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateMessageParams {
    pub chat_id: i32,
    pub text: String,
    pub user: USER,
}

pub fn create_message_mutation(
    connection: PooledPg,
    params: CreateMessageParams,
    chat_oriducer: KafkaChatHandler,
) -> Result<i32, diesel::result::Error> {
    let CreateMessageParams {
        chat_id,
        text,
        user,
    } = params;

    let mut connection = connection;
    let user_id = user.id;

    let new_message: Message = diesel::insert_into(message::table)
        .values((
            message::columns::chat_id.eq(chat_id),
            message::columns::text.eq(text),
            message::columns::user_id.eq(user_id),
        ))
        .returning(Message::as_select())
        .get_result(&mut connection)?;

    let Message { id: message_id, .. } = new_message;
    let mut mut_chat_producer = chat_oriducer;

    mut_chat_producer.send_message(
        "dive_messages",
        UserWithAuthor {
            message: new_message,
            author: Some(user),
        },
    );

    Ok(message_id)
}
