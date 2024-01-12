use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate blueberries for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate blueberries for lunch today", post.content());
}
