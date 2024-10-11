use crate::{dive_chat::{chat_socket::{event_emmiters::SEND_MESSAGE, model::{ChatSocketResponseSchema, ResponseStatus}}, model::Message, test_state}, users::model::USER};
use diesel::r2d2::event;
use serde_json::Value;

use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tracing::info;

use crate::dive_chat::chat_consumer::ChatConsumer;

use super::event_emmiters::{JOIN, ON_CONNECT};

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<test_state::Message>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Clone)]
pub struct ChatSocketState {
    pub chat_consumer: ChatConsumer,
}

pub async fn on_connect(socket: SocketRef) {
    socket.emit(
        ON_CONNECT,
        String::from("socket server successfully connected"),
    );

    socket.on(
        JOIN,
        |socket: SocketRef, Data::<i32>(room), store: State<ChatSocketState>| {
            println!("room connected, {}", room);

            let mut consumer = store.chat_consumer.clone();
            let socket = Arc::new(Mutex::new(socket));

            thread::spawn(move || loop {
                for ms in consumer.consume_events().iter() {
                    for m in ms.messages() {
                        let event_data = ChatConsumer::get_new_messages_by_chat_id(m);
                        let socket = socket.lock().unwrap();
                        let message: Result<Message, serde_json::Error> =
                        serde_json::from_value(event_data.clone());
                        
                        match message {
                            Ok(mess) => {
                                if mess.chat_id == room {
                                    let responce = ChatSocketResponseSchema {
                                        status: ResponseStatus::Success,
                                        data: event_data.to_string(),
                                    };

                                    socket.emit(SEND_MESSAGE, responce).ok();
                                }
                            }
                            Err(error) => {
                                println!("failed to parse message")
                            }
                        }
                    }
                    consumer.consume_messageset(ms);
                }
                consumer.commit_consumed();
            });
        },
    );
}