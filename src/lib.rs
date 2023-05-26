pub struct DraftPost;
pub struct PendingReviewPost;

pub struct Post<State = DraftPost> {
    content: String,
    state: std::marker::PhantomData<State>,
}

impl Post<DraftPost> {
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
    pub fn approve(self) -> Post<PendingReviewPost> {
        self
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
