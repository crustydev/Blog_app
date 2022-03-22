pub struct DownVote {
    user: User,
    tag: Post
}



impl DownVote{
    pub fn new(voted_by: &User, voted_post: &Post) -> DownVote {
        return {
            user: voted_by,
            tag: voted_post
        }
    }
}

