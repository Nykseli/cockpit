use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::cockpit_bridge::CockpitBridge;
use crate::message::BridgeMessage;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    initialized: bool,
    state: Arc<Mutex<CockpitBridge>>,
    rx: Receiver<BridgeMessage>,
    socket_id: String,
}

impl MyWebSocket {
    pub fn new(state: Arc<Mutex<CockpitBridge>>) -> Self {
        let (tx, rx) = mpsc::channel();
        state.lock().unwrap().add_connection("socket1".into(), tx);
        Self {
            hb: Instant::now(),
            state,
            initialized: false,
            rx,
            socket_id: "socket1".into(),
        }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_init(&mut self, ctx: &mut <Self as Actor>::Context) {
        // TODO: properly build the channel stuff from bridge_state
        // TODO: src/ws/cockpitwebservice.c:on_web_socket_open
        //       send similar info to client as soon as the connection opens
        ctx.text("{\"command\":\"init\",\"version\":1,\"channel-seed\":\"1:\",\"host\":\"localhost\",\"csrf-token\":\"ef91b0a75d3784c01926b839bbd5b9535f37cff83adc5d3258f25ae366b469f9\",\"capabilities\":[\"multi\",\"credentials\",\"binary\"],\"system\":{\"version\":\"290+git\"}}");
    }

    fn handle_text(&mut self, ctx: &mut <Self as Actor>::Context, text: &str) {
        println!("Handling message: {text}");
        if !self.initialized {
            // TODO: makes sure this is actually the init command
            self.initialized = true;
            self.handle_init(ctx);
            return;
        }

        println!("Sending to bridge...");
        // TODO: Proper JSON check
        if text.trim().starts_with('{') {
            self.state.lock().unwrap().send_json(text);
        } else {
            self.state.lock().unwrap().send_message(text);
        }
        println!("Send to bridge and receiving...");
        let data = self.rx.recv().unwrap();
        println!("Got: {data:?}");

        match data {
            BridgeMessage::Text(text) => ctx.text(text),
            BridgeMessage::Binary(_) => panic!("BridgeMessage::Binary handler not implemented"),
        }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.state
            .lock()
            .unwrap()
            .remove_connection(&self.socket_id);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        /* println!("WS: {msg:?}"); */
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // TODO: first message should be the
                // TODO: figure out what kind of message it is and send the type of message
                //       to cockpit-bridge
                self.handle_text(ctx, &text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
