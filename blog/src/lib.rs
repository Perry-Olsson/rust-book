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

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_modify_content() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_add_text_when_post_is_in_review() {
        let mut post = Post::new();

        post.add_text("hello");
        post.request_review();
        post.add_text(", I'm in review");
        post.approve();
        assert_eq!("hello", post.content());
    }

    #[test]
    fn cannot_add_text_when_post_is_published() {
        let mut post = Post::new();

        post.add_text("hello");
        post.request_review();
        post.approve();
        post.add_text(", I'mm published");
        assert_eq!("hello", post.content());
    }

    #[test]
    fn cannot_add_text_when_post_is_in_review_after_rejection() {
        let mut post = Post::new();

        post.add_text("hello");
        post.request_review();
        post.reject();
        post.add_text(", I've been rejected and need to be changed");
        post.request_review();
        post.add_text("hello, I'm in review again");
        post.approve();
        post.add_text("hello, I have 1 approval");
        post.approve();
        assert_eq!("hello, I've been rejected and need to be changed", post.content());
    }

    #[test]
    fn rejecting_a_post_reverts_it_back_to_draft() {
        let mut post = Post::new();

        post.add_text("hello");
        post.request_review();
        post.reject();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
    }

    #[test]
    fn rejecting_a_post_requres_two_approvals() {
        let mut post = Post::new();

        post.add_text("hello");
        post.request_review();
        post.reject();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.request_review();
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("hello", post.content());
    }

    #[test]
    fn rejecting_a_draft_or_published_post_has_no_sideaffect() {
        let mut post = Post::new();

        post.add_text("hello");
        post.reject();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("hello", post.content());
    }

    #[test]
    fn draft_post_should_return_empty_content() {
        let mut post = Post::new();

        post.add_text("hello");

        assert_eq!("", post.content());
    }

    #[test]
    fn post_pending_review_should_return_empty_content() {
        let mut post = Post::new();

        post.add_text("hello");

        post.request_review();

        assert_eq!("", post.content());
    }

    #[test]
    fn approved_post_should_return_content() {
        let mut post = Post::new();

        post.add_text("hello");

        post.request_review();

        post.approve();

        assert_eq!("hello", post.content());
    }


    #[test]
    fn add_text_should_append_string_slice() {
        let mut post = Post::new();

        post.add_text("hello");
        post.add_text(", world");

        post.request_review();
        post.approve();

        assert_eq!("hello, world", post.content());
    }
}
