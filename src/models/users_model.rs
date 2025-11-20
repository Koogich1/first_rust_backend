use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::users;
use crate::db::PgPooledConnection;

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub surname: Option<Option<String>>,
    pub avatar_url: Option<Option<String>>, 
    pub bio: Option<Option<String>>,
    pub is_active: Option<bool>,
}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub surname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub is_active: bool,
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
        users
            .select(User::as_select())
            .load(conn)
    }

    pub fn get_by_id(
        conn: &mut PgPooledConnection,
        id: i32
    ) -> Result<User, diesel::result::Error>{
        use crate::schema::users::dsl::{users};

        users
            .find(id)
            .select(User::as_select())
            .first(conn)
    }

    pub fn update (
        conn: &mut PgPooledConnection,
        user_id: i32,  
        updates: &UpdateUser,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
    
    let new_updated_at = chrono::Utc::now().naive_utc();

    let query = diesel::update(users.find(user_id))
        .set((
            updates.name.as_ref().map(|val| name.eq(val)),
            updates.email.as_ref().map(|val| email.eq(val)),
            updates.surname.as_ref().map(|val| surname.eq(val)),
            updates.avatar_url.as_ref().map(|val| avatar_url.eq(val)),
            updates.bio.as_ref().map(|val| bio.eq(val)),
            updates.is_active.as_ref().map(|val| is_active.eq(val)),
            updated_at.eq(new_updated_at),
        ));
    
    query
        .returning(User::as_returning())
        .get_result(conn)
    }

    pub fn delete (
        conn: &mut PgPooledConnection,
        user_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::users::dsl::{users};

        let deleted_count = diesel::delete(users.find(user_id))
            .execute(conn)?;

        if deleted_count == 0 {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(deleted_count)
        }
    }
}