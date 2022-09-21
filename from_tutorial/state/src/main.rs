use std::mem;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        return Post {
            state: Some(Box::new(Draft {})),
            content: "".to_string(),
        };
    }

    pub fn append_content(&mut self, content: String) {
        let new_content = self
            .state
            .as_ref()
            .unwrap()
            .append_content(content, mem::take(&mut self.content));

        self.content = new_content;
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn append_content(&self, _new_content: String, old_content: String) -> String {
        old_content
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        return "";
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview { approved: 0 });
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn append_content(&self, new_content: String, mut old_content: String) -> String {
        old_content.push_str(&new_content);
        old_content
    }
}

struct PendingReview {
    approved: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        match self.approved {
            0 => {
                self.approved += 1;
                return self;
            }
            _ => return Box::new(Published {}),
        }
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

fn main() {
    let mut post = Post::new();

    post.append_content("test".to_string());
    println!("1 {}", post.content());

    post.request_review();
    println!("2 {}", post.content());

    post.append_content("cccc".to_string());
    println!("3 {}", post.content());

    post.approve();
    println!("4 {}", post.content());

    post.approve();
    println!("5 {}", post.content());
}
