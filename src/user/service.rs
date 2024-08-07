use crate::user_weight_record::UserWeightRecord;
use crate::weight_record;
use crate::{user_weight_record, weight_record::CreateWeightRecord};

use super::{CreateUser, User};
use axum::{http::StatusCode, Json};
use sqlx::{MySql, Pool};

pub async fn fetch_users(pool: &Pool<MySql>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "
        SELECT id, username, create_time, update_time FROM user
        ;
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn fetch_users_by_id(pool: &Pool<MySql>, id: i64) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT id, username, create_time, update_time FROM user
        WHERE id = ?
        ",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn insert_user(pool: &Pool<MySql>, user: CreateUser) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("INSERT INTO user (username) VALUES (?)", user.username)
        .execute(pool)
        .await?;

    Ok(result.last_insert_id())
}

pub async fn update_user(
    pool: &Pool<MySql>,
    user: CreateUser,
    id: u64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE user SET username = ? WHERE id = ?",
        user.username,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn fetch_weight_record_by_user_id(
    pool: &Pool<MySql>,
    user_id: u64,
) -> Result<Vec<UserWeightRecord>, sqlx::Error> {
    let records = sqlx::query_as!(
        UserWeightRecord,
        "
        select 
            user_weight_record.id as id,
            user.id as user_id,
            user.username as username,
            weight_record.weight as weight,
            weight_record.date as date
        from user_weight_record
        join user on user_weight_record.user_id = user.id
        join weight_record on user_weight_record.weight_record_id = weight_record.id
        where user_weight_record.user_id = ?
        ",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn create_weight_record_by_user_id(
    pool: &Pool<MySql>,
    user_id: u64,
    weight_record: CreateWeightRecord,
) -> Result<u64, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let weight_record_id =
        weight_record::insert_weight_record(&mut *transaction, weight_record).await?;

    let id =
        user_weight_record::insert_user_weight_record(&mut *transaction, user_id, weight_record_id)
            .await?;

    transaction.commit().await?;

    Ok(id)
}
