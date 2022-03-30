/// helper function to pass along with requests a serialized struct
/// containing a vector of Articles. It returns in json format a container 
/// of all the articles belonging to a particular blogger.



use std::vec::Vec;

use crate::diesel;
use diesel::prelude::*;

use crate::json_serialization::articles::Articles;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::schema::article;


pub fn return_articles(user_id: &i32) -> Articles {
    let connection = establish_connection();

    let articles = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::blogger_id.eq(&user_id))
        .load::<Article>(&connection)
        .unwrap();

    let mut article_buffer = Vec::new();

    for article in articles {
        article_buffer.push(article);
    }

    let result: Articles = Articles { articles: article_buffer };
    return result
}