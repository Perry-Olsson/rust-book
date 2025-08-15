use crate::Post;

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;
}

pub struct Draft {
    rejected: bool
}

impl Draft {
    pub fn new() -> Draft {
        Draft { rejected: false } 
    }

    pub fn rejected() -> Draft {
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

pub struct PendingReview {
    approvals: u8,
    approvals_required: u8
}

impl PendingReview {
    pub fn new() -> PendingReview {
        PendingReview { approvals: 0, approvals_required: 1 }
    }

    pub fn rejected() -> PendingReview {
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

pub struct Published {}

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
