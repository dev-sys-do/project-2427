#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ConnectionState {
    // Initial+Final state
    Closed,

    HelloSent,
    Listening,

    HelloReceived,
    ACKSent,
    #[allow(dead_code)] // TODO: Implement NACK handling.
    NACKSent,
    ACKReceived,
    Established,
    NACKReceived,
}
