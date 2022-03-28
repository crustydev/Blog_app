use std::vec::Vec;

use crate::diesel;
use diesel::prelude::*;

use crate::json_serialization::articles::Articles;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::schema::article;


pub return_article(user_id: &i32) -> Vec<Article> {
    let connection = establish_connection();

    let articles = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::blogger_id.eq(&user_id))
        .load::<Article>(&connection)
        .unwrap();

    let mut article_buffer = Vec::new();

    for article in articles {
        let result = Article {
            pub id: article.id,
            pub title: article.title,
            pub description: article.description,
            pub content: article.content,
            pub created_at: article.created_at,
            pub updated_at: article.updated_at,
            pub blogger_id: article.blogger_id
        }
        article_buffer.push(result);
    }

    return Articles::new(article_buffer);
}