use log::{debug, warn};

pub mod connection_state;
pub mod message;

use connection_state::ConnectionState;

#[derive(Debug)]
pub struct StateMachine {
    state: ConnectionState,
    pub(crate) file_size: Option<u64>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: ConnectionState::Closed,
            file_size: None,
        }
    }

    pub fn current_state(&self) -> &ConnectionState {
        &self.state
    }

    // TODO: Ideally, this would return a Result to indicate invalid transitions.
    pub fn transition(&mut self, new_state: ConnectionState) {
        match (&self.state, new_state) {
            (ConnectionState::Closed, ConnectionState::Listening) => {}
            (ConnectionState::Listening, ConnectionState::HelloReceived) => {}
            (ConnectionState::HelloReceived, ConnectionState::NACKSent) => {}
            (ConnectionState::HelloReceived, ConnectionState::ACKSent) => {}
            (ConnectionState::ACKSent, ConnectionState::Established) => {}
            
            (ConnectionState::Closed, ConnectionState::HelloSent) => {}
            (ConnectionState::HelloSent, ConnectionState::NACKReceived) => {}
            (ConnectionState::HelloSent, ConnectionState::ACKReceived) => {}
            (ConnectionState::ACKReceived, ConnectionState::Established) => {}
            
            (s, _) => {
                warn!(
                    "Invalid state transition from {:?} to {:?}, ignoring.",
                    s, new_state
                );
            }
        };
        debug!("Transitioning from {:?} to {:?}", self.state, new_state);
        self.state = new_state;
    }
}
