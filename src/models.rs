use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::users;
use crate::db::PgPooledConnection;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn create(
        conn: &mut PgPooledConnection,
        new_user_data: NewUserRequest,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let new_user = NewUser {
            name: new_user_data.name,
            email: new_user_data.email,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }
    pub fn get_all(
        conn: &mut PgPooledConnection,
    ) -> Result<Vec<User>, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        users.load(conn)
    }

    pub fn get_by_id(
        conn: &mut PgPooledConnection,
        id: i32
    ) -> Result<User, diesel::result::Error>{
        use crate::schema::users::dsl::{users, id as user_id};

        users
            .filter(user_id.eq(id))
            .first(conn)
    }
}