use crate::modules::chat::application::usecase::ReplyToConversation;
use crate::modules::chat::domain::model::{Conversation, Message};
use crate::modules::chat::infrastructure::ChatAi;

pub fn chat_handle() {
    let chat_ai = ChatAi;
    let reply = ReplyToConversation::new(chat_ai);
    
    let conversation = Conversation {
        history: vec![],
    };
    
    let message = Message {
        role: "user".to_string(),
        content: "ok from handle".to_string(),
    };
    
    let reply = reply.execute(&conversation, &message);
    println!("{}", reply);
}
