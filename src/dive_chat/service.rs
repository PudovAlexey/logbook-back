pub struct ChatListByUserIdParams {
  pub  id: uuid::Uuid,
}
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, PgConnection, SelectableHelper};

use crate::dive_chat::model::{
    Chat, ChatUser, Message
};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

use crate::schema::chat;
use crate::schema::chat_user;
use crate::schema::message;

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