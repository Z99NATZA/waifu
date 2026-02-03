use crate::modules::chat::domain::contract::ChatPort;
use crate::modules::chat::domain::model::Conversation;
use crate::modules::chat::domain::model::Message;

pub struct ReplyToConversation<P: ChatPort> {
    chat_port: P,
}

impl<P: ChatPort> ReplyToConversation<P> {
    pub fn new(chat_port: P) -> Self {
        Self { chat_port }
    }

    pub fn execute(
        &self,
        conversation: &Conversation,
        message: &Message
    ) -> String {
        self.chat_port.generate_reply(
            conversation,
            message
        )
    }
}
