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
        self.content.push_str(text);
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

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {
    rejected: bool
}

impl Draft {
    fn new() -> Draft {
        Draft { rejected: false } 
    }

    fn rejected() -> Draft {
        Draft { rejected: true }
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        if self.rejected { Box::new(PendingReview::rejected()) } else { Box::new(PendingReview::new()) }
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approvals: u8,
    approvals_required: u8
}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview { approvals: 0, approvals_required: 1 }
    }

    fn rejected() -> PendingReview {
        PendingReview { approvals: 0, approvals_required: 2 }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals += 1;
        if self.approvals == self.approvals_required {
            Box::new(Published{})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft::rejected())
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content[..]
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
