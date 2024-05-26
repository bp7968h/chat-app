extern crate mio;
use mio::*;
use std::net::SocketAddr;
use mio::tcp::*;
use std::collections::HashMap;

struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize
}

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>, token: Token, events: EventSet){

    }
}

fn main(){
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;

    event_loop.register(&server_socket,
        Token(0),
        EventSet::readable(),
        PollOpt::edge()).unwrap();

// Create a new instance of our handler struct:
    // ... and provide the event loop with a mutable reference to it:
    event_loop.run(&mut handler).unwrap();
}
