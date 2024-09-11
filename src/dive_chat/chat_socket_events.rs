
use serde_json::Value;

use socketioxide::extract::{
    SocketRef, 
    Data,
    AckSender,
    Bin
};
use tracing::info;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    socket.emit("on_connect", String::from("socket server successfully connected"));


    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        },
    );
}