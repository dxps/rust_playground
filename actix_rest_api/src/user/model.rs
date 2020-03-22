use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::get_conn()?;
        let users = user::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::get_conn()?;
        let user = user::table.filter(user::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn create(user_dto: UserDto) -> Result<Self, ApiError> {
        let conn = db::get_conn()?;
        let user = User::from(user_dto);
        let user = diesel::insert_into(user::table).values(user).get_result(&conn)?;
        Ok(user)
    }

    pub fn update(id: Uuid, user_dto: UserDto) -> Result<Self, ApiError> {
        //FIXME: Update the `updated_at` column
        let conn = db::get_conn()?;
        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user_dto)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::get_conn()?;
        let res = diesel::delete(user::table.filter(user::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl From<UserDto> for User {
    fn from(user_dto: UserDto) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user_dto.email,
            password: user_dto.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
