use chrono::NaiveDateTime;

use bcrypt::verify;

use crate::schema::blogger;


#[derive(Queryable, Identifiable, Clone, Debug)]
#[table_name="blogger"]
pub struct Blogger {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Blogger {
    pub fn verify(self, password: String) -> bool {
        return verify(password.as_str(),
                        &self.password).unwrap();
    }
}