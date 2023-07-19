#![allow(dead_code)]

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

    fn user_submit(&mut self, msg: String) {
        self.chat_history.push(ChatMessage {user: true, text: msg})
    }
    fn bot_submit(&mut self, msg: String) {
        self.chat_history.push(ChatMessage {user: false, text: msg})
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chat() {
        let mut c: Chat = Chat::default();
        c.user_submit(
            "System: PHD in literature, particularly post-humanism and James Joyce"
                .to_string());
        c.user_submit("Write a poem about the CIA using recovered alien tech to false flag\
        alien abductions".to_string());

    }
}