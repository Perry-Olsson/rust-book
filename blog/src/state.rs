use crate::Post;

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn can_modify_content(&self) -> bool {
        false
    }
}

pub struct Draft {}

impl Draft {
    pub fn new() -> Draft {
        Draft { } 
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_modify_content(&self) -> bool {
        true
    }
}

pub struct PendingReview {}

impl PendingReview {
    pub fn new() -> PendingReview {
        PendingReview {}
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published::new())
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Rejected::new())
    }
}

pub struct Published {}

impl Published {
    fn new() -> Published {
        Published {}
    }
}

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

pub struct Rejected {}

impl Rejected {
    fn new() -> Rejected {
        Rejected {}
    }
}

impl State for Rejected {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(RejectedPendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_modify_content(&self) -> bool {
        true
    }
}

pub struct RejectedPendingReview {}

impl State for RejectedPendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Rejected::new())
    }
}

impl RejectedPendingReview {
    fn new() -> RejectedPendingReview {
        RejectedPendingReview {}
    }
}
