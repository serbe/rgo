use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
// use log::info;

use crate::db::DB;
use crate::error::ServiceError;
use crate::server::{Connect, Disconnect, Msg, Server};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn wsroute(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let db = DB::new(srv.get_ref().clone()).start();
    ws::start(
        Session {
            id: 0,
            hb: Instant::now(),
            server: srv.get_ref().clone(),
            db,
        },
        &req,
        stream,
    )
}

struct Session {
    id: usize,
    hb: Instant,
    server: Addr<Server>,
    db: Addr<DB>,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.server
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<Msg> for Session {
    type Result = Result<String, ServiceError>;

    fn handle(&mut self, msg: Msg, _ctx: &mut Self::Context) -> Self::Result {
        Ok(msg.0)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(msg) => self
                .db
                .send(Msg(msg))
                .into_actor(self)
                .then(|res, _self_actor, ctx| {
                    match res {
                        Ok(res_wsmsg) => match res_wsmsg {
                            Ok(txt) => ctx.text(txt),
                            Err(err) => println!("err wsmsg: {:?}", err.to_string()),
                        },
                        _ => println!("Something is wrong"),
                    }
                    fut::ready(())
                })
                .wait(ctx),
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl Session {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.server.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}
