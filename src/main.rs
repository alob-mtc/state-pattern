use state_pattern::Post;

fn main() {
    let mut post = Post::new();
    assert_eq!("v1.0.0", post.version());

    post.add_text("I am a software engineer");

    let post = post.request_review();
    assert_eq!("v1.0.0", post.version());

    let post = post.approve();
    assert_eq!("v1.0.0", post.version());
    assert_eq!("I am a software engineer", post.content());
}
