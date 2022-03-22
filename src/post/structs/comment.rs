const COMMENT_MAX_CHAR: i32 = 300

use super::post::Post;
use crate::user::User;

struct Comment {
    pub super_struct: Post,
    pub tag: Post,
    pub max_char: i32
}


impl Comment {
    pub fn new(user: &User, input: &String, tag:&Post) -> Comment {
        let post: Post = Post::new(user, input);
        return Comment {
            super_struct: post,
            tag: tag,
            max_char: COMMENT_MAX_CHAR
        }
    }
}


