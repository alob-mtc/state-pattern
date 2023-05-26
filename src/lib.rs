pub struct DraftPost;
pub struct PendingReviewPost;
pub struct ApprovedPost;

pub struct Post<State = DraftPost> {
    content: String,
    state: std::marker::PhantomData<State>,
}

impl Post {
    pub fn new() -> Post<DraftPost> {
        Post {
            content: String::new(),
            state: Default::default(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> Post<PendingReviewPost> {
        Post {
            content: self.content,
            state: std::marker::PhantomData::<PendingReviewPost>,
        }
    }
}

impl Post<PendingReviewPost> {
    pub fn approve(self) -> Post<ApprovedPost> {
        Post {
            content: self.content,
            state: std::marker::PhantomData::<ApprovedPost>,
        }
    }
}

impl Post<ApprovedPost> {
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl<State> Post<State> {
    pub fn version(&self) -> &str {
        "v1.0.0"
    }
}
