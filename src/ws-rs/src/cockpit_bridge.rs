use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::message::BridgeMessage;

const BPATH: &str = "/home/duck/Documents/cockpit/cockpit-tree/rust-rewrite/cockpit-bridge";

pub struct CockpitBridge {
    #[allow(dead_code)]
    thread_handle: JoinHandle<()>,
    process: Child,
    connections: Arc<Mutex<HashMap<String, Sender<BridgeMessage>>>>,
}

impl CockpitBridge {
    pub fn create(tx: Sender<String>) -> Self {
        let mut process = Command::new(BPATH)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut output = process.stdout.take().unwrap();
        let connections = Arc::new(Mutex::new(HashMap::<String, Sender<BridgeMessage>>::new()));
        let thead_con = connections.clone();
        let thread_handle = thread::spawn(move || {
            let connections = thead_con;
            let mut linebuf: [u8; 4096] = [0; 4096];
            loop {
                // TODO: handle reads that are longer than 4096
                let len = output.read(&mut linebuf).unwrap();
                if len > 0 {
                    let out_string: String = String::from_utf8_lossy(&linebuf[0..len]).into_owned();
                    println!("read n data! {}, {}", len, out_string);
                    // TODO: filter the right message
                    // TODO: handle the message lenght properly
                    let lines: Vec<&str> = out_string.lines().collect();
                    let msg = &lines[1..].join("\n");
                    tx.send(msg.clone()).unwrap();
                    let cons = connections.lock().unwrap();
                    for (key, socket_rx) in cons.iter() {
                        println!("Sending to: {key}");
                        socket_rx.send(BridgeMessage::Text(msg.clone())).unwrap();
                    }
                    println!("Data sent to sockets");
                }
            }
        });

        Self {
            thread_handle,
            process,
            connections,
        }
    }

    pub fn add_connection(&mut self, socket: String, tx: Sender<BridgeMessage>) {
        let connections = &mut self.connections.lock().unwrap();
        connections.insert(socket, tx);
    }

    pub fn remove_connection(&mut self, socket: &String) {
        let connections = &mut self.connections.lock().unwrap();
        connections.remove(socket);
    }

    /// Json messages
    pub fn send_json(&mut self, json: &str) {
        let stdin = self.process.stdin.as_mut().unwrap();

        // Unwrap to close the stdin, otherwise it will cause a lock
        stdin
            .write_all(format!("{}\n\n{json}", json.len() + 1 /* + 3 */).as_bytes())
            .unwrap();
    }

    /// Messages for messages like "1:1\nfoo"
    pub fn send_message(&mut self, msg: &str) {
        let stdin = self.process.stdin.as_mut().unwrap();

        // Unwrap to close the stdin, otherwise it will cause a lock
        stdin
            .write_all(format!("{}\n{msg}", msg.len() /* + 3 */).as_bytes())
            .unwrap();
    }
}
