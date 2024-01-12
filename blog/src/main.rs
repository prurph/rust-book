use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate blueberries for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate blueberries for lunch today", post.content());
}
