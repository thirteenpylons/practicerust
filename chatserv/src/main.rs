

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    // Create a new instance of our handler struct:
    let mut handler = WebSocketServer;

    // ... and provide the event loop with a mutable reference to it.
    event_loop.run(&mut handler).unwrap();
}


struct WebSocketServer;

impl Handler for WebSocketServer {
    // Traits can have useful default implementation, so in fact
    // handler interface requires us to provide only two things:
    // concrete types for timeouts and messages.
    type Timeout = usize;
    type Message = ();
}