/// path:-> blogger/profile/{username}

use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpRequest, HttpResponse};

use crate::json_serialization::blogger::BloggerDetails;

use crate::database::establish_connection;
use crate::models::blogger::blogger::Blogger;
use crate::schema::blogger;


pub fn return_details(req: HttpRequest) -> HttpResponse {
    let connection = establish_connection();

    let username: String = req.match_info().get("username")
        .unwrap().to_string();


    let bloggers = blogger::table
        .order(blogger::columns::id.asc())
        .filter(blogger::columns::username.eq(&username))
        .load::<Blogger>(&connection)
        .unwrap();
    
    let blogger_details = BloggerDetails::new(bloggers[0].clone());
    
    return HttpResponse::Ok().json(&blogger_details) 
}