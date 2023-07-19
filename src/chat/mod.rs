
pub struct ChatMessage {
    user: bool,
    text: String,
    // time: std::time::Instant,
}


pub struct Chat {
    chat_history: Vec<ChatMessage>,
    length: usize,
    // model: llm::Model
}
impl Chat {
    fn default() -> Self {
        Chat {
            chat_history: Vec::new(),
            length: 0,
        }
    }

    fn user_submit(&mut self,) -> Self {
        todo!()
    }

}