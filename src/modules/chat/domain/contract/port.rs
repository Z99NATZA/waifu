use crate::modules::chat::domain::model::{Conversation, Message};

pub trait ChatPort {
    fn generate_reply(
        &self, 
        conversation: &Conversation,
        message: &Message
    ) -> String;
}


