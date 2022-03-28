use crate::diesel;
use diesel::prelude::*;

use crate::json_serialization::blogger::BloggerDetails;

use crate::database::establish_connection;
use crate::models::blogger::blogger::Blogger;
use crate::schema::blogger;


pub fn return_details(user_id: i32) -> BloggerDetails {
    let connection = establish_connection();

    let blogger = blogger::table
        .order(blogger::columns::id.asc())
        .filter(blogger::columns::id.eq(&user_id))
        .load::<Blogger>(&connection)
        .unwrap();
    
    
    return BloggerDetails::new(blogger[0]);
}