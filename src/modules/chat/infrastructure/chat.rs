use crate::modules::chat::domain::{contract::ChatPort, model::{Conversation, Message}};

pub struct ChatAi;

impl ChatPort for ChatAi {
    fn generate_reply(
        &self, 
        _conversation: &Conversation,
        message: &Message
    ) -> String {
        message.content.clone()
    }
}