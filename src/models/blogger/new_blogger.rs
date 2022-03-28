extern crate bcrypt;

use crate::schema::blogger;
use bcrypt::{DEFAULT_COST, hash};


#[derive(Insertable, Clone, Debug)]
#[table_name="blogger"]
pub struct NewBlogger {
    pub username: String,
    pub email: String,
    pub password: String
}


impl NewBlogger {
    pub fn new(username: String, email: String,
                 password: String) -> NewBlogger {

        let hashed_password: String = hash(
            password.as_str(), DEFAULT_COST).unwrap();

        return NewBlogger {
            username: username,
            email: email,
            password: hashed_password
        }
    }
}


#[cfg(test)]
mod blogger_tests {
    use super::NewBlogger;

    #[test]
    fn new() {
        let username: String = String::from("ademola");
        let email: String = String::from("ad@gmail.com");
        let password: String = String::from("password");

        let expected_username: String = String::from("ademola");
        let expected_email: String = String::from("ad@gmail.com");
        
        let new_user: NewBlogger = NewBlogger::new(username, email, password);
        assert_eq!(new_user.username, expected_username);
        assert_eq!(new_user.email, expected_email);
    }
}