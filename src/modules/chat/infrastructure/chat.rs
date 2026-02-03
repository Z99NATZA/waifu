use crate::modules::chat::domain::contract::ChatPort;
use crate::modules::chat::domain::model::Conversation;
use crate::modules::chat::domain::model::Message;

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