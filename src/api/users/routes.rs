use crate::api::users::user::{self, Meta, Param, UserForm, UserRespond, UsersRespond};
use crate::api::Respond;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::validate::validate;
use actix_web::{
    delete, get, post, put,
    web::{block, scope, Data, Json, Path, Query, ServiceConfig},
};

#[get("")]
pub async fn fetch(
    params: Query<Param>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UsersRespond, Meta>>, ApiError> {
    let users = block(move || user::all(&pool, params.into_inner())).await?;
    Ok(Json(users))
}

#[get("/{user_id}")]
pub async fn detail(
    user_id: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UserRespond, ()>>, ApiError> {
    let user = block(move || user::find_by("id", user_id.to_string(), &pool)).await?;
    Ok(Json(user))
}

#[get("/archive")]
pub async fn archive_list(
    params: Query<Param>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UsersRespond, Meta>>, ApiError> {
    let users = block(move || user::all_archive(&pool, params.into_inner())).await?;

    Ok(Json(users))
}

#[post("")]
pub async fn create(
    user_response: Json<UserForm>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UserRespond, ()>>, ApiError> {
    validate(&user_response)?;

    let new_user = block(move || user::store(&pool, user_response)).await?;

    Ok(Json(new_user))
}

#[put("/{user_id}")]
pub async fn edit(
    user_id: Path<String>,
    params: Json<UserForm>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UserRespond, ()>>, ApiError> {
    let update_user =
        block(move || user::update(&pool, user_id.to_string(), params.into_inner())).await?;
    Ok(Json(update_user))
}

#[delete("/{user_id}")]
pub async fn delete(
    user_id: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<Respond<UserRespond, ()>>, ApiError> {
    let user = block(move || user::destroy(user_id.to_string(), &pool)).await?;
    Ok(Json(user))
}

pub fn init_rotes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("users")
            .service(archive_list)
            .service(fetch)
            .service(create)
            .service(edit)
            .service(detail)
            .service(delete),
    );
}
