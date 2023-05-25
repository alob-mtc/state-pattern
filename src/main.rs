use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I am a software engineer");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I am a software engineer", post.content());
}
