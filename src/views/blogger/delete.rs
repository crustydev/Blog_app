/// path: -> blogger/delete_account/confirm_delete
/// 
/// blogger/delete_account is a frontend view that prompts the user to 
/// confirm the password. the request to delete is only accepted if the 
/// user passed the correct password.
/// 
/// retrieves current user account from token to ensure that an account
/// can be deleted only by its owner or login

use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};
use crate::json_serialization::login::Login;

use crate::database::establish_connection;
use crate::models::blogger::blogger::Blogger;
use crate::schema::blogger;

use crate::auth::jwt::JwtToken;

/// the frontend asks for confirmation in form of a password and passes a
/// json struct containing the username and password to this backend view
pub async fn delete(req: HttpRequest, credentials: web::Json<Login>) 
                        -> HttpResponse {
    
    let token: JwtToken = JwtToken::decode_from_request(
                            req).unwrap();

    let password: String = credentials.password.clone();
    let connection = establish_connection();

    let bloggers = blogger::table
        .filter(blogger::columns::id.eq(&token.blogger_id))
        .load::<Blogger>(&connection).unwrap();
    
    
    match bloggers[0].clone().verify(password){
        true => {
            let _ = diesel::delete(&bloggers[0]).execute(&connection);
            return HttpResponse::Ok().await.unwrap()
        },
        false => return HttpResponse::Unauthorized().await.unwrap()
    }

}

