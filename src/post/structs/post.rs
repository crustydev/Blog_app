extern crate chrono;
use chrono::prelude::*;

extern crate uuid;
use uuid::Uuid;

use super::post::Post;
use super::comment::Comment;
use crate::vote::Like;
use crate::vote::Dislike;
use crate::user::user::User;


pub struct Post {
    pub author: User, 
    pub date: DateTime<Local>
    pub content: String,
    pub comments: Vec!<Comment>,
    pub upvotes: Vec!<UpVote>,
    pub downvotes: Vec!<DownVote>
    pub total_votes: i32,
    pub unique_id: String
}


impl Post {
    pub fn new(user: &User, input: &String) -> Post {
        let votes: i32 = 0;
        let unique_id: String = Uuid::new_v4().to_string();
        
        return Post {
            author: user,
            date: DateTime<Utc> = Utc::now(),
            content: input,
            comments: Vec!<Comment>::new();
            likes: Vec!<UpVote>::new(),
            dislikes: Vec!<DownVote>::new(),
            total_votes: votes,
            unique_id: unique_id
        }
    }
}
