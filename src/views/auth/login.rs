use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use craet::database::establish_connection;
use crate::models::blogger::blogger::Blogger;
use crate::json_serialization::login::Login;
use crate::schema::blogger;
use crate::auth::jwt::JwtToken;

pub async fn login(credentials: web::Json<Login>) -> 
        HttpResponse {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let connection = establish_connection;

    let blogger = blogger::table
        .filter(blogger::columns::username.eq(username.as_str()))
        .load<Blogger>(&connection).unwrap();
    
    if blogger.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if blogger.len() > 1 {
        log::error!("multiple users have the username: {}",
                    credentials.username.clone());
        return HttpResponse::Conflict().await.unwrap()
    }

    match blogger[0].clone().verify(password) {
        true => {
            let token: String = JwtToken::encode(
                blogger[0].clone().id);
            HttpResponse::Ok().header(
                "token", token).await.unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}