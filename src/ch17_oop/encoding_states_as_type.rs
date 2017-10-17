#[test]
fn test_main() {
    use self::Either::{Left, Right};

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    if let Left(post) = post.approve() {
        if let Right(post) = post.approve() {
            assert_eq!("I ate a salad for lunch today", post.content());
        } else {
            panic!("unexpected");
        }
    } else {
        panic!("unexpected");
    }
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            approved_cnt: 0,
            content: self.content,
        }
    }
}

struct PendingReviewPost {
    approved_cnt: u8,
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Either<PendingReviewPost, Post> {
        use self::Either::{Left, Right};

        if self.approved_cnt == 1 {
            Right(Post {
                content: self.content,
            })
        } else {
            Left(PendingReviewPost {
                approved_cnt: 1,
                content: self.content,
            })
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}