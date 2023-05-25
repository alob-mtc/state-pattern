use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I am a software engineer");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I am a software engineer", post.content());
}
