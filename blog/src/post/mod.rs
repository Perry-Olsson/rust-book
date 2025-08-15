mod state;

use state::{State, Draft};

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft::new())),
            content: String::new()
        }
    }

    pub fn add_text(mut self, text: &str) -> Post {
        if self.state.as_ref().unwrap().can_modify_content() {
            self.content.push_str(text);
        }
        self
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(mut self) -> Post {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
        self
    }

    pub fn approve(mut self) -> Post {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
        self
    }

    pub fn reject(mut self) -> Post {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject())
        }
        self
    }
}
