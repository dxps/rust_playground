use blog::Post;

fn main() {
    let mut post = Post::new();

    let some_txt = "It's a new day to be happy";

    post.add_text(some_txt);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(some_txt, post.content());
}
