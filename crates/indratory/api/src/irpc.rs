pub use crate::proto_generated::*;
use alloc::vec::Vec;
use indranet_types::messaging::{MessageOrigin, SignedMessage};
pub use irpc::{client, server, Message};
pub type EgressMessages = Vec<(MessageOrigin, Vec<SignedMessage>)>;
