use blog::post::Post;

fn main() {
    let mut post = Post::new();

    post = post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post = post.request_review();
    assert_eq!("", post.content());

    post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
