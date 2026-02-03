use crate::modules::chat::application::usecase::ContinueConversation;
use crate::modules::chat::domain::model::{Conversation, Message};
use crate::modules::chat::infrastructure::ChatAi;

pub fn chat_handle() {
    let chat_ai = ChatAi;
    let chat = ContinueConversation::new(chat_ai);
    
    let conversation = Conversation {
        history: vec![],
    };
    
    let message = Message {
        role: "user".to_string(),
        content: "ok from handle".to_string(),
    };
    
    let reply = chat.execute(&conversation, &message);
    println!("{}", reply);
}
