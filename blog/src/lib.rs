pub mod post;

#[cfg(test)]
mod tests {
    use crate::post::Post;

    #[test]
    fn cannot_add_text_when_post_is_in_review() {
        assert_eq!(
            "hello",
            Post::new().add_text("hello")
                .request_review()
                .add_text(", I'm in review")
                .approve()
                .content()
            )
    }

    #[test]
    fn cannot_add_text_when_post_is_published() {
        assert_eq!(
            "hello",
            Post::new()
                .add_text("hello")
                .request_review()
                .approve()
                .add_text(", I'm published")
                .content()
            );
    }

    #[test]
    fn cannot_add_text_when_post_is_in_review_after_rejection() {
        assert_eq!(
            "hello, I've been rejected and need to be changed",
            Post::new()
                .add_text("hello")
                .request_review()
                .reject()
                .add_text(", I've been rejected and need to be changed")
                .request_review()
                .add_text("hello, I'm in review again")
                .approve()
                .add_text("hello, I have 1 approval")
                .approve()
                .content()
            );
    }

    #[test]
    fn rejecting_a_post_reverts_it_back_to_draft() {
        assert_eq!("", Post::new().add_text("hello")
            .request_review()
            .reject()
            .content());
        assert_eq!(
            "", 
            Post::new()
                .add_text("hello")
                .request_review()
                .reject()
                .approve()
                .content()
            );
    }

    #[test]
    fn rejecting_a_post_requres_two_approvals() {
        let mut post = Post::new();
        post = post.add_text("hello")
            .request_review()
            .reject();
        assert_eq!(
            "", 
            post.content()
            );
        post = post.approve();
        assert_eq!("", post.content());
        post = post.request_review().approve();
        assert_eq!("", post.content());
        post = post.approve();
        assert_eq!("hello", post.content());
    }

    #[test]
    fn rejecting_a_draft_or_published_post_has_no_sideaffect() {
        let mut post = Post::new();

        post = post.add_text("hello").reject();
        assert_eq!("", post.content());
        post = post.approve();
        assert_eq!("", post.content());
        post = post.request_review();
        assert_eq!("", post.content());
        post = post.approve();
        assert_eq!("hello", post.content());
    }

    #[test]
    fn draft_post_should_return_empty_content() {
        assert_eq!("", Post::new()
            .add_text("hello")
            .content());
    }

    #[test]
    fn post_pending_review_should_return_empty_content() {
        let mut post = Post::new();

        post = post.add_text("hello").request_review();

        assert_eq!("", post.content());
    }

    #[test]
    fn approved_post_should_return_content() {
        let post = Post::new();
        assert_eq!(
            "hello",
            post.add_text("hello")
                .request_review()
                .approve()
                .content()
        );
    }


    #[test]
    fn add_text_should_append_string_slice() {
        let post = Post::new()
            .add_text("hello")
            .add_text(", world")
            .request_review()
            .approve();

        assert_eq!("hello, world", post.content());
    }
}
