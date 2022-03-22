pub struct UpVote {
    user: User,
    tag: Post
}


impl UpVote {
    pub fn new(voted_by: &User, voted_post: &Post) -> UpVote {
        return {
            user: voted_by,
            tag: voted_post
        }
    }
}