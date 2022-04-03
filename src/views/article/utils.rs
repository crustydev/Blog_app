/// helper function to pass along with requests a serialized struct
/// containing a vector of Articles. It returns in json format a container 
/// of all the articles belonging to a particular blogger.


use crate::diesel;
use diesel::prelude::*;

use crate::json_serialization::articles::Articles;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::schema::article;



pub fn return_user_articles(user_id: &i32) -> Articles {
    let connection = establish_connection();

    let articles = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::blogger_id.eq(&user_id))
        .load::<Article>(&connection)
        .unwrap();

    return Articles { articles }
}


pub fn return_all_articles() -> Articles {
    let connection = establish_connection();

    let articles = article::table
        .order(article::columns::id.asc())
        .load::<Article>(&connection)
        .unwrap();
    
    return Articles { articles }
}