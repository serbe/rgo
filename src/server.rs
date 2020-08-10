use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient, StreamHandler};
// use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
// use deadpool_postgres::Pool;
// use log::info;
use rand::{self, rngs::ThreadRng, Rng};
// use serde_json::json;

// use crate::db::WsMsg;
use crate::error::ServiceError;

pub struct Msg(pub String);

impl Message for Msg {
    type Result = Result<String, ServiceError>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Msg>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

// pub struct ClientMessage {
//     pub id: usize,
//     pub msg: String,
// }

// impl Message for ClientMessage {
//     type Result = ();
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Join {
//     pub id: usize,
// }

pub struct Server {
    sessions: HashMap<usize, Recipient<Msg>>,
    rng: ThreadRng,
    // db: Addr<DB>,
}

impl Default for Server {
    fn default() -> Server {
        Server {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            // db,
        }
    }
}

// impl Server {
//     fn send_message(&self, message: &str, id: usize) {
//         if let Some(addr) = self.sessions.get(&id) {
//             let _ = addr.do_send(Msg(message.to_owned()));
//         }
//     }
// }

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}
struct MyWs {
    // pool: Pool,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
