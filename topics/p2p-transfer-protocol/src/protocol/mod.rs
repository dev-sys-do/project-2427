use log::{debug, warn};

pub mod message;
pub mod connection_state;

use connection_state::ConnectionState;

pub struct StateMachine {
    state: ConnectionState,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: ConnectionState::Closed,
        }
    }

    pub fn transition(&mut self, new_state: &ConnectionState) {
        debug!("Transitioning from {:?} to {:?}", self.state, new_state);
        match (&self.state, new_state) {
            (ConnectionState::Closed, ConnectionState::Listening) => {}
            (ConnectionState::Listening, ConnectionState::HelloReceived) => {}
            (ConnectionState::HelloReceived, ConnectionState::NACKSent) => {}
            (ConnectionState::HelloReceived, ConnectionState::ACKSent) => {}

            (ConnectionState::Closed, ConnectionState::HelloSent) => {}
            (ConnectionState::HelloSent, ConnectionState::NACKReceived) => {}
            (ConnectionState::HelloSent, ConnectionState::ACKReceived) => {}
            (ConnectionState::ACKReceived, ConnectionState::Established) => {}


            (s, _) => {warn!("Invalid state transition from {:?} to {:?}, ignoring.", s, new_state);}   
        };
        debug!("New state: {:?}", self.state);
    }
}
