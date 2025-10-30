#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionState {
    // Initial+Final state
    Closed,
    
    HelloSent,
    Listening,

    HelloReceived,
    SendReceived,
    ACKSent,
    NACKSent,
    ACKReceived,
    Established,
    NACKReceived,
}
