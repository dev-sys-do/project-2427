mod client;
mod protocol;
//mod server;

fn main() {
    // Parse arguments
    // Go to server or client mode

}

fn server_mode() {
    // Start server
    // Bind to port
    // Open output file
    // Wait for client
    // Receive & save file
    
    let hello = "HELLO".parse::<protocol::message::Message>().unwrap();
    print!("Parsed message: {}", hello);
    
}

fn client_mode() {
    // Start client
    // Connect to server
    // Open input file
    // Send input file
    // Show result
    
    let message = protocol::message::Message::ACK;
    print!("Message: {}", message);
}
