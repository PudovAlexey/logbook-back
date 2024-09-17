pub struct ChatListByUserIdParams {
  pub  id: uuid::Uuid,
}
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgConnection, SelectableHelper};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema};

use crate::dive_chat::model::{
    Chat, ChatUser, Message
};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

use crate::schema::chat;
use crate::schema::chat_user;
use crate::schema::message;

use super::chat_producer::ChatProducer;

pub fn get_chat_list_by_user_id( connection: PooledPg, params: ChatListByUserIdParams) -> Result<Vec<Chat>, diesel::result::Error> {

    let ChatListByUserIdParams {id: user_auth_id} = params;

    let mut connection = connection;

    let chat_data = chat_user::table
    .left_join(chat::table.on(chat_user::columns::chat_id.eq(chat::columns::id)))
    .filter(chat_user::columns::user_id.eq(user_auth_id))
    .select((ChatUser::as_select(), Option::<Chat>::as_select()))
    .load(&mut connection)?;

    let chat_data: Vec<Chat> = chat_data.into_iter().fold(Vec::new(), |mut acc: Vec<Chat>, (_, chat)| {

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

pub fn get_message_list_by_id(connection: PooledPg, params: GetMessageListByIdParams) -> Result<Vec<Message>, diesel::result::Error> {
    let GetMessageListByIdParams {id: chat_id} = params;

    let mut connection = connection;

    let message_data = message::table
    .filter(message::columns::chat_id.eq(chat_id))
    .select(Message::as_select())
    .load(&mut connection)?;

    Ok(message_data)
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateChatParams {
    pub title: String,
    pub description: String,
    pub participants: Vec<uuid::Uuid>
}

pub fn create_chat_mutation(connection: PooledPg, params: CreateChatParams) -> Result<i32, diesel::result::Error> {
    let CreateChatParams {title: chat_title, description, participants} = params;

    let mut connection = connection;

    let mut new_chat: i32 = diesel::insert_into(chat::table)
    .values((
       chat::columns::title.eq(chat_title),
       chat::columns::description.eq(description)
    ))
    .returning(chat::columns::id)
    .get_result(&mut connection)?;

    let insert_values: Vec<_> = participants.iter().map(|participant| {
        (chat_user::columns::chat_id.eq(new_chat), chat_user::columns::user_id.eq(participant))
    }).collect();

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
}

pub fn create_message_mutation(connection: PooledPg, params: CreateMessageParams, chat_oriducer:  ChatProducer) -> Result<i32, diesel::result::Error> {
    let CreateMessageParams {chat_id, text} = params;

    let mut connection = connection;

    let new_message: Message = diesel::insert_into(message::table)
    .values((
        message::columns::chat_id.eq(chat_id),
        message::columns::text.eq(text),
    ))
    .returning(Message::as_select())
    .get_result(&mut connection)?;

    let Message {id: message_id, ..} = new_message;
    let mut mut_chat_producer = chat_oriducer;

    mut_chat_producer.send_message("dive_messages", new_message);

    Ok(message_id)
}