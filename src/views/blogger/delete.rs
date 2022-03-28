use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpResponse, HttpRequest};

use crate::database::establish_connection;
use crate::models::blogger::blogger::Blogger;
use crate::schema::blogger;

use crate::auth::jwt::JwtToken;

pub async fn delete(req: HttpRequest) -> HttpResponse {

    let token: JwtToken = JwtToken::decode_from_request
                                    (req).unwrap();

    let connection = establish_connection();

    let blogger = blogger::table
        .filter(blogger::columns::id.eq(&token.blogger_id))
        .order(blogger::columns::id.asc())
        .load::<Blogger>(&connection)
        .unwrap();

    let _ = diesel::delete(&blogger[0]).execute(&connection);
    return HttpResponse::Ok().await.unwrap();
}

