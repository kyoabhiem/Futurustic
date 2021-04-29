use crate::api::{
    Respond,
};
use crate::database::{self, PoolType};
use crate::errors::ApiError;
use crate::schema::users;
use actix_web::web::Json;
use bcrypt::hash;
use chrono::{Local, NaiveDateTime};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rayon::prelude::*;
use uuid::Uuid;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Clone, Deserialize, Insertable, Validate)]
#[table_name = "users"]
pub struct UserForm {
    #[validate(length(
        min = 4,
        message = "First name is required and must be at last 4 characters"
    ))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Clone, Deserialize, AsChangeset, Validate)]
#[table_name = "users"]
pub struct UserExist {
    #[validate(length(
        min = 4,
        message = "First name is required and must be at last 4 characters"
    ))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub updated_at: NaiveDateTime,
}
impl From<UserForm> for User {
    fn from(user: UserForm) -> Self {
        User {
            id: Uuid::new_v5(
                &Uuid::NAMESPACE_OID,
                format!("{}{}", &user.email, Local::now().timestamp()).as_bytes(),
            )
            .to_string(),
            name: user.name,
            email: user.email,
            password: hash(user.password, 5).unwrap(),
            created_by: None,
            created_at: Local::now().naive_local(),
            updated_by: None,
            updated_at: Local::now().naive_local(),
            deleted_by: None,
            deleted_at: None,
        }
    }
}
impl From<UserForm> for UserExist {
    fn from(user: UserForm) -> Self {
        UserExist {
            name: user.name,
            email: user.email,
            password: hash(user.password, 5).unwrap(),
            updated_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[serde(default)]
#[derive(Clone, Deserialize, Debug)]
pub struct Param {
    pub offset: i64,
    pub limit: i64,
}

impl Default for Param {
    fn default() -> Self {
        Param {
            offset: 0,
            limit: 10,
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserRespond {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersRespond(pub Vec<UserRespond>);

impl From<User> for UserRespond {
    fn from(user: User) -> Self {
        UserRespond {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
        }
    }
}

impl From<Vec<User>> for UsersRespond {
    fn from(users: Vec<User>) -> Self {
        UsersRespond(users.into_par_iter().map(|user| user.into()).collect())
    }
}

pub fn all(pool: &PoolType, param: Param) -> Result<Respond<UsersRespond, Meta>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;
    let query = user()
        .filter(deleted_at.is_null())
        .offset(param.offset)
        .limit(param.limit)
        .order_by(created_at.desc())
        .load(&connection)?;

    let count = users
        .filter(deleted_at.is_null())
        .count()
        .get_result(&connection)?;

    let _data: UsersRespond = query.into();

    Ok(Respond {
        data: _data,
        meta: Option::Some(Meta {
            limit: param.limit,
            offset: param.offset,
            total: count,
        }),
    })
}

pub fn all_archive(pool: &PoolType, param: Param) -> Result<Respond<UsersRespond, Meta>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;
    let data_archive_user = user()
        .filter(deleted_at.is_not_null())
        .offset(param.offset)
        .limit(param.limit)
        .load(&connection)?;
    let count = user()
        .filter(deleted_at.is_not_null())
        .count()
        .get_result(&connection)?;

    Ok(Respond {
        data: data_archive_user.into(),
        meta: Option::Some(Meta {
            limit: param.limit,
            offset: param.offset,
            total: count,
        }),
    })
}

pub fn find_by(
    field: &'static str,
    value: String,
    pool: &PoolType,
) -> Result<Respond<UserRespond, ()>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;
    let mut query = user().filter(deleted_at.is_null());
    let not_found = format!("User {} : {} not found", field, value);

    match field {
        "id" => query = query.filter(id.eq(value)),
        "email" => query = query.filter(email.eq(value)),
        "name" => query = query.filter(name.eq(value)),
        _ => {}
    }

    let user: User = query
        .first::<User>(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(Respond {
        data: user.into(),
        meta: None,
    })
}

pub fn store(
    pool: &PoolType,
    new_user: Json<UserForm>,
) -> Result<Respond<UserRespond, ()>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;
    let user: User = new_user.clone().into();
    diesel::insert_into(users)
        .values(user)
        .execute(&connection)?;
    let user = find_by("email", new_user.email.to_string(), pool);
    user
}

pub fn update(
    pool: &PoolType,
    user_id: String,
    update_user: UserForm,
) -> Result<Respond<UserRespond, ()>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;

    let update_user: UserExist = update_user.clone().into();

    diesel::update(users)
        .filter(id.eq(user_id.clone()))
        .set(update_user)
        .execute(&connection)?;

    let user = find_by("id", user_id, &pool);
    user
}

pub fn destroy(user_id: String, pool: &PoolType) -> Result<Respond<UserRespond, ()>, ApiError> {
    use crate::schema::users::dsl::*;

    let connection = pool.get()?;
    let user = find_by("id", user_id.clone(), &pool);

    diesel::update(users)
        .filter(id.eq(user_id))
        .set(deleted_at.eq(Local::now().naive_local()))
        .execute(&connection)?;

    user
}

fn user() -> users::BoxedQuery<'static, database::DbType> {
    users::table.into_boxed()
}
