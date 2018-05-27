extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate vdom_rsjs;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fmt::Debug;
use std::net::SocketAddr;

use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;
use websocket::async::Server;

use tokio_core::reactor::{Handle, Core};
use futures::{Future, Sink, Stream};
use vdom_rsjs::{VNode, VTag, VProperty};

type ShouldRender = bool;

#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Increment,
    Decrement,
}

#[derive(Debug)]
struct Counter {
    count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct FullUpdate {
    tree: VNode<Action>,
}

impl Counter {
    fn update(&mut self, msg: Action) -> ShouldRender {
        match msg {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        true
    }

    fn render(&self) -> VNode<Action> {
        VNode::Tag(VTag {
            name: "div".into(),
            properties: HashMap::new(),
            children: vec![
                VNode::Text(self.count.to_string()),
                VNode::Tag(VTag {
                    name: "br".into(),
                    properties: HashMap::new(),
                    children: vec![],
                    key: None,
                    namespace: None,
                }),
                VNode::Tag(VTag {
                    name: "button".into(),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("onclick".into(), VProperty::Action(Action::Increment));
                        props
                    },
                    children: vec![
                        VNode::Text("increment".into()),
                    ],
                    key: None,
                    namespace: None,
                }),
                VNode::Tag(VTag {
                    name: "button".into(),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("onclick".into(), VProperty::Action(Action::Decrement));
                        props
                    },
                    children: vec![
                        VNode::Text("decrement".into()),
                    ],
                    key: None,
                    namespace: None,
                }),
            ],
            key: None,
            namespace: None,
        })
    }
}

fn handle_message(addr: &SocketAddr, state: &mut Counter, msg: OwnedMessage) -> Option<OwnedMessage> {
    println!("{}: Message from Client: {:?}", addr, msg);
    match msg {
        OwnedMessage::Ping(msg) => Some(OwnedMessage::Pong(msg)),
        OwnedMessage::Pong(_) => None,
        OwnedMessage::Text(msg) => {
            let action: Action = match serde_json::from_str(&msg) {
                Ok(action) => action,
                Err(err) => {
                    println!("{}: error deserializing {:?}", addr, err);
                    return None;
                }
            };
            if state.update(action) {
                let tree = FullUpdate { tree: state.render() };
                let json = serde_json::to_string(&tree).unwrap();
                Some(OwnedMessage::Text(json))
            } else {
                None
            }
        }
        OwnedMessage::Binary(_) => {
            println!("{}: unexpected binary message", addr);
            None
        }
        OwnedMessage::Close(_) => {
            None
        }
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let server = Server::bind("127.0.0.1:8080", &handle).unwrap();

    let f = server.incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(|(upgrade, addr)| {
            println!("Got a connection from {}", addr);

            if !upgrade.protocols().iter().any(|s| s == "vdom-rsjs-websocket") {
                spawn_future(upgrade.reject(), "Upgrade Rejection", &handle);
                return Ok(());
            }

            let f = upgrade
                .use_protocol("vdom-rsjs-websocket")
                .accept()
                .and_then(move |(s, _)| {
                    let state = Counter { count: 0 };
                    let tree = state.render();
                    println!("{}: initial state: {:?}", addr, state);
                    println!("{}: initial tree: {:?}", addr, tree);

                    let update = FullUpdate { tree };
                    let json = serde_json::to_string(&update).unwrap();
                    s.send(Message::text(json).into())
                        .and_then(|s| Ok((s, state)))
                })
                .and_then(move |(s, mut state)| {
                    let (sink, stream) = s.split();
                    stream
                        .take_while(|m| Ok(!m.is_close()))
                        .filter_map(move |m| handle_message(&addr, &mut state, m))
                        .forward(sink)
                        .and_then(|(_, sink)| {
                            sink.send(OwnedMessage::Close(None))
                        })
                });

            spawn_future(f, "Client Status", &handle);
            Ok(())
        });

    core.run(f).unwrap();
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
    where F: Future<Item = I, Error = E> + 'static,
          E: Debug
{
    handle.spawn(f.map_err(move |e| println!("{}: '{:?}'", desc, e))
                  .map(move |_| println!("{}: Finished.", desc)));
}
