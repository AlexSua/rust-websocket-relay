use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws;

use super::models::websocket_server_messages;
use crate::lock_traits::ReadLockTrait;

#[derive(Clone, Message)]
#[rtype(result = "()")]
struct WebsocketMessage {
    text: String,
    addr: Addr<MyWs>,
}

pub struct MyData {
    pub channels: Arc<RwLock<HashMap<String, Addr<MyWs>>>>,
}

pub struct MyWs {
    pub channels: Arc<RwLock<HashMap<String, Addr<MyWs>>>>,
    pub room_id: String,
    pub remote_addr: Option<Addr<MyWs>>,
}

impl MyWs {
    fn check_and_insert_channel(&mut self, ctx_address: Addr<MyWs>) -> &str {
        let mut status = websocket_server_messages::status::WAITING;
        self.channels.lock_and_write(|channelsguard| -> () {
            if let Some(result) = channelsguard.get(&self.room_id) {
                self.remote_addr = Some(result.clone());
                let _request = result.try_send(WebsocketMessage {
                    text: websocket_server_messages::remote::OPEN.to_owned(),
                    addr: ctx_address,
                });
                status = websocket_server_messages::status::CONNECTED;
            } else {
                channelsguard.insert(self.room_id.clone(), ctx_address);
                self.remote_addr = None;
            }
        });
        return status;
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<WebsocketMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.text.clone());

        if msg
            .text
            .starts_with(websocket_server_messages::remote::OPEN)
        {
            self.remote_addr.replace(msg.addr.clone());
            self.channels.lock_and_write(|channelsguard| -> () {
                channelsguard.remove(&self.room_id);
            });
            ctx.text(websocket_server_messages::status::CONNECTED);
        }

        if msg
            .text
            .starts_with(websocket_server_messages::remote::CLOSE)
        {
            ctx.text(self.check_and_insert_channel(ctx.address()));
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if let Some(remote) = self.remote_addr.clone() {
                    let request = remote.try_send(WebsocketMessage {
                        text: text.to_string(),
                        addr: ctx.address(),
                    });
                    if request.is_err() {
                        ctx.text(self.check_and_insert_channel(ctx.address()));
                    }
                } else {
                    ctx.text(websocket_server_messages::error::message_not_received(
                        text.to_string(),
                    ));
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                self.finished(ctx);
                ctx.close(reason)
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(self.check_and_insert_channel(ctx.address()));
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        if let Some(remote) = self.remote_addr.clone() {
            let _request = remote.try_send(WebsocketMessage {
                text: websocket_server_messages::remote::CLOSE.to_owned(),
                addr: ctx.address(),
            });
        } else {
            self.channels.lock_and_write(|channelsguard| -> () {
                channelsguard.remove(self.room_id.as_str());
            });
        }
        ctx.stop();
    }
}
