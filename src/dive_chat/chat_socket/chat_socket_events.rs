use crate::{
    dive_chat::{
        chat_socket::{
            event_emmiters::SEND_MESSAGE,
            model::{ChatSocketResponseSchema, ResponseStatus},
        },
        kafka_chat_handler::KafkaChatHandler,
        model::{Message, UserWithAuthor},
        test_state,
    },
    users::model::USER,
};

use socketioxide::extract::{Data, SocketRef, State};
use std::{
    sync::{Arc, Mutex},
    thread,
};

use super::{
    event_emmiters::{JOIN, ON_CONNECT},
    model::MessageParams,
};

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
    pub kafka_chat_handler: KafkaChatHandler,
}

pub async fn on_connect(socket: SocketRef) {
    socket.emit(
        ON_CONNECT,
        String::from("socket server successfully connected"),
    );

    socket.on(
        JOIN,
        |socket: SocketRef, Data::<MessageParams>(room), store: State<ChatSocketState>| {
            println!("room connected, {}", room.room_id);

            let mut consumer = store.kafka_chat_handler.clone();
            let socket = Arc::new(Mutex::new(socket));

            thread::spawn(move || loop {
                for ms in consumer.consume_events().iter() {
                    for m in ms.messages() {
                        let event_data = KafkaChatHandler::get_new_messages_by_chat_id(m);
                        let socket = socket.lock().unwrap();
                        let message: Result<UserWithAuthor, serde_json::Error> =
                            serde_json::from_value(event_data.clone());

                        println!("{:?}", message);
                        match message {
                            Ok(mess) => {
                                match mess.author {
                                    Some(author) => {
                                        if author.id != room.user_uuid {
                                            let responce = ChatSocketResponseSchema {
                                                status: ResponseStatus::Success,
                                                data: event_data.to_string(),
                                            };

                                            socket.emit(SEND_MESSAGE, event_data.to_string()).ok();
                                        }
                                    }
                                    None => {
                                        println!("error")
                                    }
                                }
                                // if mess.author != room.user_uuid {
                                //     let responce = ChatSocketResponseSchema {
                                //         status: ResponseStatus::Success,
                                //         data: event_data.to_string(),
                                //     };

                                //     socket.emit(SEND_MESSAGE, responce).ok();
                                // }
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
