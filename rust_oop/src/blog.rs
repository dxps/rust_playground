// --------------------------------------------------------------------------------------
// This module is a sample implementation of the State pattern.
// --------------------------------------------------------------------------------------

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "" // the default implementation (thus, we don't have to explicitly
           // implement it in `DraftState` and `PendingReviewState`)
    }
}

struct DraftState {}
impl State for DraftState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReviewState {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // no effect (a post in draft state cannot be directly approved)
    }
}

struct PendingReviewState {}
impl State for PendingReviewState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // no effect (post is already in pending review)
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PublishedState {})
    }
}

struct PublishedState {}
impl State for PublishedState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // no effect (post is already published)
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // no effect (post is already published)
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// ____________________________________________________________________________

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(DraftState {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // `.content()` of each State implementation is being used
        // as we want to keep these rules inside the structs that implement State.
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
