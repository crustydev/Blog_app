/// path:-> /blogger/create
/// Retrieves new blogger details from the serialized Json struct passed
/// alongside the request and creates a new blogger.
///
   

use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::blogger::NewBloggerJson;
use crate::models::blogger::new_blogger::NewBlogger;
use crate::schema::blogger;


pub async fn create (new_blogger: web::Json<NewBloggerJson>) 
        -> HttpResponse {
    let connection = establish_connection();
    
    let username: String = new_blogger.username.clone();
    let email: String = new_blogger.email.clone();
    let password: String = new_blogger.password.clone();

    let new_blogger = NewBlogger::new(username, email, password);

    let insert_result = diesel::insert_into(blogger::table)
            .values(&new_blogger)
            .execute(&connection);
    
    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap()
    }
}