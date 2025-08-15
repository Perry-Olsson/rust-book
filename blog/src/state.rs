use crate::Post;

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;
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

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
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

pub struct Rejected {
    in_review: bool
}

impl Rejected {
    fn new() -> Rejected {
        Rejected { in_review: false }
    }
}

impl State for Rejected {
    fn request_review(mut self: Box<Self>) -> Box<dyn State> {
        self.in_review = true;
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if !self.in_review {
            self
        } else {
            Box::new(PendingReview::new())
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
