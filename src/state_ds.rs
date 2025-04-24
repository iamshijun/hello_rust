
// Rust权威指南 第17章 状态模式

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State> ;
    fn approve(self: Box<Self>) -> Box<dyn State> ;
    fn content<'a>(&self,post:&'a Post) -> &'a str {
        ""
    }
}
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self,post:&'a Post) -> &'a str {
        post.content.as_str()
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    fn new(content: String) -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content,
        }
    }
    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
        // let a: Option<&Box<dyn State>> = self.state.as_ref();
        // let b: &Box<dyn State> = a.unwrap();
        // b.request_review();
        //b.content(&self);

    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(Published{}));
        }
    }
    fn test(self) -> Box<dyn State> {
        let c = self.state;//move out of self
        let s = self.content;
        // let c2 = self.state;
        return c.unwrap();
    }
    fn content<'a>(&'a self) -> &'a str {
        
        let a: Option<&Box<dyn State>> = self.state.as_ref();
        let b: &Box<dyn State> = a.unwrap();
        b.content(self)
    }
}