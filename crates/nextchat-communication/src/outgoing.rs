//! NextChat Communication outgoing module.

use super::CommunicationMessage;

pub trait PacketComposer {
    fn to_message(&self) -> CommunicationMessage;
}
