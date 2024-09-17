
use diesel::r2d2::event;
use serde_json::Value;

use socketioxide::extract::{
    SocketRef, 
    Data,
    AckSender,
    Bin
};
use tracing::info;

use crate::dive_chat::chat_consumer::ChatConsumer;


pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    let mut consumer = ChatConsumer::new(vec![ "localhost:9092".to_string() ], String::from("dive_messages"));
    socket.emit("on_connect", String::from("socket server successfully connected"));

    socket.emit("hello_world", String::from("Hello"));

    // socket.on(
    //     "message-with-ack",
    //     |Data::<Value>(data), ack: AckSender, Bin(bin)| {
    //         info!("Received event: {:?} {:?}", data, bin);
    //         ack.bin(bin).send(data).ok();
    //     },
    // );

    // std::thread::spawn(move || {
        loop {
            for ms in consumer.consume_events().iter() {
              for m in ms.messages() {
        
                // when the consumer receives an event, this block is executed 
                let event_data = ChatConsumer::get_new_messages_by_chat_id(m);
                socket.emit("hello_world", "Hello Братан").ok();
                println!("you can send message {:?}", event_data);
                // let action = event_data["action"].to_string();
                
                // println!("{}", texts.to_json());
                // if action == "\"add\"" {
                //   texts.add_text( event_data["value"].to_string() );
        
                // } else if action == "\"remove\"" {
                //   let index = event_data["value"].to_string().parse::<usize>().unwrap();
                //   texts.remove_text( index );
        
                // } else {
                //   println!("Invalid action");
                // }
        
        
                // producer.send_data_to_topic( "texts", texts.to_json() );
              }
              consumer.consume_messageset(ms);
            }
            consumer.commit_consumed();
          }
    // });
}